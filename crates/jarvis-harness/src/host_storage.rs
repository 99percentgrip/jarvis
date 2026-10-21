#![forbid(unsafe_code)]
//! Persistent ownership and lifecycle records for RRC-managed storage.
//!
//! The Host Resource Governor is the policy owner. This module is its durable
//! storage boundary: resources are registered before use, lifecycle changes are
//! atomic, and reclamation refuses unknown, active, recovery-bearing, or
//! path-ambiguous data.

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const REGISTRY_SCHEMA: u32 = 1;
const OWNER_MARKER: &str = ".jarvis-storage-owner-v1";
const REGISTRY_FILE: &str = "resources-v1.json";
const REGISTRY_LOCK: &str = "resources.lock";
const MAX_CLEANUP_ATTEMPTS: u32 = 3;

/// Storage classes are ranked from cheap disposable data to expensive caches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedResourceClass {
    Staging,
    SyntheticArtifact,
    OperationScratch,
    RepairWorkspace,
    CargoCacheSegment,
    CargoTargetObservation,
    CompactEvidence,
}

impl ManagedResourceClass {
    fn cleanup_rank(self) -> u8 {
        match self {
            Self::Staging => 0,
            Self::SyntheticArtifact => 1,
            Self::OperationScratch => 2,
            Self::RepairWorkspace => 3,
            Self::CargoCacheSegment => 4,
            Self::CargoTargetObservation | Self::CompactEvidence => u8::MAX,
        }
    }
}

/// Lifecycle state recorded independently from the existence of a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedResourceState {
    Active,
    RecoveryRequired,
    Retained,
    Completed,
    Expired,
    OrphanedOwned,
    CleanupPending,
    Removed,
}

impl ManagedResourceState {
    fn cleanup_candidate(self) -> bool {
        matches!(self, Self::Completed | Self::Expired | Self::OrphanedOwned)
    }
}

/// Retention is explicit so a successful operation does not imply every byte
/// may be removed and a failed operation does not retain every compiler output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum ResourceRetention {
    UntilSettled,
    PreserveForRecovery,
    Explicit,
    ExpireAtUnixSeconds(u64),
}

/// Process identity supplements the PID. A PID alone never proves that a lease
/// is still owned by the same process after restart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLeaseIdentity {
    pub lease_id: String,
    pub owner_pid: u32,
    pub process_start_identity: Option<String>,
    pub operation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedResourceRecord {
    pub resource_id: String,
    pub operation_id: String,
    pub canonical_path: PathBuf,
    pub resource_class: ManagedResourceClass,
    pub ownership_identity: String,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
    pub lifecycle_state: ManagedResourceState,
    pub lease: Option<ResourceLeaseIdentity>,
    pub retention: ResourceRetention,
    pub cleanup_eligible: bool,
    pub contains_source_changes: bool,
    pub required_for_evidence: bool,
    pub logical_bytes: u64,
    pub last_successful_verification_unix_seconds: Option<u64>,
    pub cleanup_attempts: u32,
    pub cleanup_result: Option<String>,
}

impl ManagedResourceRecord {
    #[must_use]
    pub fn new(
        resource_id: impl Into<String>,
        operation_id: impl Into<String>,
        canonical_path: PathBuf,
        resource_class: ManagedResourceClass,
        ownership_identity: impl Into<String>,
        lifecycle_state: ManagedResourceState,
        retention: ResourceRetention,
    ) -> Self {
        let now = unix_seconds();
        Self {
            resource_id: resource_id.into(),
            operation_id: operation_id.into(),
            canonical_path,
            resource_class,
            ownership_identity: ownership_identity.into(),
            created_at_unix_seconds: now,
            updated_at_unix_seconds: now,
            lifecycle_state,
            lease: None,
            retention,
            cleanup_eligible: false,
            contains_source_changes: false,
            required_for_evidence: false,
            logical_bytes: 0,
            last_successful_verification_unix_seconds: None,
            cleanup_attempts: 0,
            cleanup_result: None,
        }
    }

    fn safe_cleanup_candidate(&self, now: u64) -> bool {
        let retention_allows = match self.retention {
            ResourceRetention::UntilSettled => true,
            ResourceRetention::ExpireAtUnixSeconds(deadline) => now >= deadline,
            ResourceRetention::PreserveForRecovery | ResourceRetention::Explicit => false,
        };
        self.lifecycle_state.cleanup_candidate()
            && self.cleanup_eligible
            && retention_allows
            && self.lease.is_none()
            && !self.contains_source_changes
            && !self.required_for_evidence
            && self.cleanup_attempts < MAX_CLEANUP_ATTEMPTS
            && self.resource_class.cleanup_rank() != u8::MAX
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StorageRegistryDocument {
    schema_version: u32,
    ownership_identity: String,
    authorized_roots: Vec<PathBuf>,
    resources: Vec<ManagedResourceRecord>,
    historical_peak_managed_bytes: u64,
    last_observed_managed_bytes: Option<u64>,
    last_observed_at_unix_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageInventory {
    pub active_count: usize,
    pub retained_count: usize,
    pub reclaimable_count: usize,
    pub protected_recovery_count: usize,
    pub managed_logical_bytes: u64,
    pub reclaimable_logical_bytes: u64,
    pub historical_peak_managed_bytes: u64,
    pub recent_growth_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReclamationReport {
    pub attempted: usize,
    pub removed: usize,
    pub failed: usize,
    pub logical_bytes_removed: u64,
    pub available_bytes_before: u64,
    pub available_bytes_after: u64,
    pub actual_available_bytes_gained: u64,
    pub removed_resource_ids: Vec<String>,
    pub preserved_resource_ids: Vec<String>,
    pub failures: Vec<String>,
}

/// RAII proof that one controller operation currently owns a managed path.
/// Dropping the lease clears the durable active marker and records the
/// caller-selected conservative settlement state.
#[derive(Debug)]
pub struct ManagedResourceLease {
    registry: StorageResourceRegistry,
    resource_id: String,
    settlement_state: ManagedResourceState,
    cleanup_eligible: bool,
    settled: bool,
}

impl ManagedResourceLease {
    pub fn settle(mut self, state: ManagedResourceState, cleanup_eligible: bool) -> io::Result<()> {
        self.registry.settle_lease(
            &self.resource_id,
            state,
            cleanup_eligible,
            "lease settled explicitly",
        )?;
        self.settled = true;
        Ok(())
    }
}

impl Drop for ManagedResourceLease {
    fn drop(&mut self) {
        if !self.settled {
            let _ = self.registry.settle_lease(
                &self.resource_id,
                self.settlement_state,
                self.cleanup_eligible,
                "lease settled on owner drop",
            );
        }
    }
}

/// Persistent registry rooted in controller-owned application state.
#[derive(Debug, Clone)]
pub struct StorageResourceRegistry {
    state_root: PathBuf,
    ownership_identity: String,
}

impl StorageResourceRegistry {
    pub fn open(
        state_root: impl Into<PathBuf>,
        ownership_identity: impl Into<String>,
        authorized_roots: &[PathBuf],
    ) -> io::Result<Self> {
        let state_root = state_root.into();
        if state_root.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "storage state root is a symlink",
            ));
        }
        fs::create_dir_all(&state_root)?;
        let state_root = state_root.canonicalize()?;
        let ownership_identity = ownership_identity.into();
        validate_identifier(&ownership_identity, "ownership identity")?;
        initialize_owner_marker(&state_root, &ownership_identity)?;
        let normalized_roots = authorized_roots
            .iter()
            .map(|root| canonical_boundary(root))
            .collect::<io::Result<Vec<_>>>()?;
        let registry = Self {
            state_root,
            ownership_identity,
        };
        registry.with_document(|document| {
            if document.ownership_identity != registry.ownership_identity {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "storage registry ownership identity does not match",
                ));
            }
            for root in normalized_roots {
                if !document.authorized_roots.contains(&root) {
                    document.authorized_roots.push(root);
                }
            }
            document.authorized_roots.sort();
            document.authorized_roots.dedup();
            Ok(())
        })?;
        Ok(registry)
    }

    #[must_use]
    pub fn state_root(&self) -> &Path {
        &self.state_root
    }

    pub fn register(&self, mut record: ManagedResourceRecord) -> io::Result<()> {
        if record.ownership_identity != self.ownership_identity {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "managed resource belongs to a different storage owner",
            ));
        }
        validate_identifier(&record.resource_id, "resource id")?;
        validate_identifier(&record.operation_id, "operation id")?;
        record.canonical_path = canonical_boundary(&record.canonical_path)?;
        record.updated_at_unix_seconds = unix_seconds();
        self.with_document(|document| {
            validate_resource_boundary(document, &record.canonical_path)?;
            if document
                .resources
                .iter()
                .any(|existing| existing.resource_id == record.resource_id)
            {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!("managed resource {} already exists", record.resource_id),
                ));
            }
            if document.resources.iter().any(|existing| {
                existing.lifecycle_state != ManagedResourceState::Removed
                    && existing.canonical_path == record.canonical_path
            }) {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "managed resource path is already registered: {}",
                        record.canonical_path.display()
                    ),
                ));
            }
            document.resources.push(record);
            Ok(())
        })
    }

    pub fn update(
        &self,
        resource_id: &str,
        update: impl FnOnce(&mut ManagedResourceRecord) -> io::Result<()>,
    ) -> io::Result<()> {
        self.with_document(|document| {
            let record = document
                .resources
                .iter_mut()
                .find(|record| record.resource_id == resource_id)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("managed resource {resource_id} is not registered"),
                    )
                })?;
            update(record)?;
            record.updated_at_unix_seconds = unix_seconds();
            Ok(())
        })
    }

    pub fn transition(
        &self,
        resource_id: &str,
        next: ManagedResourceState,
        cleanup_eligible: bool,
    ) -> io::Result<()> {
        self.update(resource_id, |record| {
            validate_transition(record.lifecycle_state, next)?;
            if next == ManagedResourceState::Removed {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Removed is reserved for verified reclamation",
                ));
            }
            record.lifecycle_state = next;
            record.cleanup_eligible = cleanup_eligible;
            Ok(())
        })
    }

    pub fn acquire_lease(&self, resource_id: &str, lease: ResourceLeaseIdentity) -> io::Result<()> {
        self.update(resource_id, |record| {
            if record.lifecycle_state == ManagedResourceState::Removed {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "cannot lease a removed managed resource",
                ));
            }
            if record.lifecycle_state == ManagedResourceState::CleanupPending {
                return Err(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "managed resource cleanup is in progress",
                ));
            }
            if record.lease.is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "managed resource already has an active lease",
                ));
            }
            record.lease = Some(lease);
            record.lifecycle_state = ManagedResourceState::Active;
            record.cleanup_eligible = false;
            Ok(())
        })
    }

    pub fn release_lease(&self, resource_id: &str) -> io::Result<()> {
        self.update(resource_id, |record| {
            record.lease = None;
            Ok(())
        })
    }

    pub fn lease(
        &self,
        resource_id: &str,
        operation_id: &str,
        settlement_state: ManagedResourceState,
        cleanup_eligible: bool,
    ) -> io::Result<ManagedResourceLease> {
        validate_identifier(operation_id, "operation id")?;
        let lease = ResourceLeaseIdentity {
            lease_id: format!("{}-{}-{}", std::process::id(), unix_seconds(), resource_id),
            owner_pid: std::process::id(),
            process_start_identity: process_start_identity(std::process::id()),
            operation_id: operation_id.to_owned(),
        };
        self.acquire_lease(resource_id, lease)?;
        Ok(ManagedResourceLease {
            registry: self.clone(),
            resource_id: resource_id.to_owned(),
            settlement_state,
            cleanup_eligible,
            settled: false,
        })
    }

    fn settle_lease(
        &self,
        resource_id: &str,
        state: ManagedResourceState,
        cleanup_eligible: bool,
        result: &str,
    ) -> io::Result<()> {
        self.update(resource_id, |record| {
            record.lease = None;
            record.lifecycle_state = state;
            record.cleanup_eligible = cleanup_eligible;
            record.cleanup_result = Some(result.into());
            Ok(())
        })
    }

    /// Clear only leases whose recorded process identity can be proven dead or
    /// reused. Ambiguous identities remain active and therefore unreclaimable.
    pub fn reconcile_stale_leases(&self) -> io::Result<usize> {
        self.with_document_result(|document| {
            let mut reconciled = 0;
            for record in &mut document.resources {
                let Some(lease) = record.lease.as_ref() else {
                    continue;
                };
                if lease_owner_is_live(lease) {
                    continue;
                }
                record.lease = None;
                record.lifecycle_state = ManagedResourceState::RecoveryRequired;
                record.cleanup_eligible = false;
                record.cleanup_result = Some("stale lease reconciled; recovery retained".into());
                record.updated_at_unix_seconds = unix_seconds();
                reconciled += 1;
            }
            Ok(reconciled)
        })
    }

    pub fn resources(&self) -> io::Result<Vec<ManagedResourceRecord>> {
        self.read_document().map(|document| document.resources)
    }

    pub fn observe_inventory(&self) -> io::Result<StorageInventory> {
        self.with_document_result(|document| {
            let now = unix_seconds();
            let mut inventory = inventory_from(document, now);
            let previous = document.last_observed_managed_bytes.unwrap_or(0);
            inventory.recent_growth_bytes =
                inventory.managed_logical_bytes.saturating_sub(previous);
            document.historical_peak_managed_bytes = document
                .historical_peak_managed_bytes
                .max(inventory.managed_logical_bytes);
            document.last_observed_managed_bytes = Some(inventory.managed_logical_bytes);
            document.last_observed_at_unix_seconds = Some(now);
            inventory.historical_peak_managed_bytes = document.historical_peak_managed_bytes;
            Ok(inventory)
        })
    }

    /// Reconcile only interrupted cleanup. Active leases remain preserved until
    /// a platform process-identity observer proves them stale.
    pub fn reconcile_interrupted_cleanup(&self) -> io::Result<()> {
        self.with_document(|document| {
            for record in &mut document.resources {
                if record.lifecycle_state != ManagedResourceState::CleanupPending {
                    continue;
                }
                match fs::symlink_metadata(&record.canonical_path) {
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        record.lifecycle_state = ManagedResourceState::Removed;
                        record.cleanup_eligible = false;
                        record.cleanup_result = Some("cleanup completed before restart".into());
                    }
                    Ok(_) => {
                        record.lifecycle_state = ManagedResourceState::Completed;
                        record.cleanup_result =
                            Some("cleanup interrupted; safe retry required".into());
                    }
                    Err(error) => {
                        record.lifecycle_state = ManagedResourceState::RecoveryRequired;
                        record.cleanup_eligible = false;
                        record.cleanup_result =
                            Some(format!("cleanup reconciliation failed closed: {error}"));
                    }
                }
                record.updated_at_unix_seconds = unix_seconds();
            }
            Ok(())
        })
    }

    /// Reclaim eligible resources in lifecycle/rank/age order. Actual free
    /// space before and after is measured on `capacity_path`; logical removed
    /// bytes remain a separate diagnostic and are never represented as capacity.
    pub fn reclaim(
        &self,
        capacity_path: &Path,
        required_available_bytes: u64,
        managed_low_watermark_bytes: u64,
    ) -> io::Result<ReclamationReport> {
        let capacity_path = existing_ancestor(capacity_path)?;
        let before = fs2::available_space(&capacity_path)?;
        let now = unix_seconds();
        let mut candidates = self
            .read_document()?
            .resources
            .into_iter()
            .filter(|record| record.safe_cleanup_candidate(now))
            .collect::<Vec<_>>();
        candidates.sort_by_key(|record| {
            (
                record.resource_class.cleanup_rank(),
                record.updated_at_unix_seconds,
                Reverse(record.logical_bytes),
                record.resource_id.clone(),
            )
        });
        let mut report = ReclamationReport {
            available_bytes_before: before,
            available_bytes_after: before,
            ..ReclamationReport::default()
        };

        for candidate in candidates {
            let inventory = self.observe_inventory()?;
            let available = fs2::available_space(&capacity_path)?;
            if available >= required_available_bytes
                && inventory.managed_logical_bytes <= managed_low_watermark_bytes
            {
                break;
            }
            report.attempted += 1;
            self.update(&candidate.resource_id, |record| {
                if !record.safe_cleanup_candidate(now) {
                    return Err(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "managed resource eligibility changed before cleanup",
                    ));
                }
                record.lifecycle_state = ManagedResourceState::CleanupPending;
                record.cleanup_attempts = record.cleanup_attempts.saturating_add(1);
                record.cleanup_result = Some("cleanup started".into());
                Ok(())
            })?;

            let outcome = self.validate_and_remove(&candidate);
            match outcome {
                Ok(()) => {
                    self.update(&candidate.resource_id, |record| {
                        record.lifecycle_state = ManagedResourceState::Removed;
                        record.cleanup_eligible = false;
                        record.lease = None;
                        record.cleanup_result = Some("removed and remeasured".into());
                        record.last_successful_verification_unix_seconds = Some(unix_seconds());
                        Ok(())
                    })?;
                    report.removed += 1;
                    report.logical_bytes_removed = report
                        .logical_bytes_removed
                        .saturating_add(candidate.logical_bytes);
                    report.removed_resource_ids.push(candidate.resource_id);
                }
                Err(error) => {
                    let message = format!("{}: {error}", candidate.resource_id);
                    self.update(&candidate.resource_id, |record| {
                        record.lifecycle_state = ManagedResourceState::Completed;
                        record.cleanup_eligible = record.cleanup_attempts < MAX_CLEANUP_ATTEMPTS;
                        record.cleanup_result = Some(format!("cleanup failed: {error}"));
                        Ok(())
                    })?;
                    report.failed += 1;
                    report.failures.push(message);
                }
            }
        }

        let final_document = self.read_document()?;
        report.preserved_resource_ids = final_document
            .resources
            .iter()
            .filter(|record| record.lifecycle_state != ManagedResourceState::Removed)
            .map(|record| record.resource_id.clone())
            .collect();
        report.available_bytes_after = fs2::available_space(&capacity_path)?;
        report.actual_available_bytes_gained = report
            .available_bytes_after
            .saturating_sub(report.available_bytes_before);
        Ok(report)
    }

    fn validate_and_remove(&self, record: &ManagedResourceRecord) -> io::Result<()> {
        let document = self.read_document()?;
        validate_resource_boundary(&document, &record.canonical_path)?;
        let current = document
            .resources
            .iter()
            .find(|candidate| candidate.resource_id == record.resource_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "resource disappeared"))?;
        if current.lifecycle_state != ManagedResourceState::CleanupPending
            || current.lease.is_some()
            || current.contains_source_changes
            || current.required_for_evidence
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "resource is not in an exact cleanup-pending state",
            ));
        }
        let metadata = match fs::symlink_metadata(&current.canonical_path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(error),
        };
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cleanup root is a symlink or junction",
            ));
        }
        remove_owned_path(&current.canonical_path, &metadata)
    }

    fn read_document(&self) -> io::Result<StorageRegistryDocument> {
        let lock = self.lock_file()?;
        FileExt::lock_shared(&lock)?;
        let result = self.load_document();
        let _ = FileExt::unlock(&lock);
        result
    }

    fn with_document(
        &self,
        update: impl FnOnce(&mut StorageRegistryDocument) -> io::Result<()>,
    ) -> io::Result<()> {
        self.with_document_result(|document| {
            update(document)?;
            Ok(())
        })
    }

    fn with_document_result<T>(
        &self,
        update: impl FnOnce(&mut StorageRegistryDocument) -> io::Result<T>,
    ) -> io::Result<T> {
        let lock = self.lock_file()?;
        lock.lock_exclusive()?;
        let mut document = self.load_document()?;
        let result = update(&mut document)?;
        self.save_document(&document)?;
        let _ = FileExt::unlock(&lock);
        Ok(result)
    }

    fn lock_file(&self) -> io::Result<File> {
        refuse_metadata_symlink(&self.state_root.join(REGISTRY_LOCK))?;
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.state_root.join(REGISTRY_LOCK))
    }

    fn load_document(&self) -> io::Result<StorageRegistryDocument> {
        let path = self.state_root.join(REGISTRY_FILE);
        refuse_metadata_symlink(&path)?;
        let mut document = match fs::read(&path) {
            Ok(bytes) => {
                serde_json::from_slice::<StorageRegistryDocument>(&bytes).map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("malformed managed-resource registry: {error}"),
                    )
                })?
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => StorageRegistryDocument {
                schema_version: REGISTRY_SCHEMA,
                ownership_identity: self.ownership_identity.clone(),
                authorized_roots: Vec::new(),
                resources: Vec::new(),
                historical_peak_managed_bytes: 0,
                last_observed_managed_bytes: None,
                last_observed_at_unix_seconds: None,
            },
            Err(error) => return Err(error),
        };
        if document.schema_version != REGISTRY_SCHEMA {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "unsupported managed-resource schema {}",
                    document.schema_version
                ),
            ));
        }
        if document.ownership_identity != self.ownership_identity {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "managed-resource registry has a foreign owner",
            ));
        }
        for root in &mut document.authorized_roots {
            *root = canonical_boundary(root)?;
        }
        let authorized_roots = document.authorized_roots.clone();
        for record in &mut document.resources {
            if record.ownership_identity != self.ownership_identity {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "managed resource record belongs to a different storage owner",
                ));
            }
            validate_identifier(&record.resource_id, "resource id")?;
            record.canonical_path = canonical_boundary(&record.canonical_path)?;
            validate_resource_boundary_roots(&authorized_roots, &record.canonical_path)?;
        }
        Ok(document)
    }

    fn save_document(&self, document: &StorageRegistryDocument) -> io::Result<()> {
        refuse_metadata_symlink(&self.state_root.join(REGISTRY_FILE))?;
        let bytes = serde_json::to_vec_pretty(document)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        // A crashed writer may leave its temporary file behind. A unique,
        // RAII-owned name avoids blocking every subsequent controller restart.
        let mut file = tempfile::NamedTempFile::new_in(&self.state_root)?;
        file.write_all(&bytes)?;
        file.write_all(b"\n")?;
        file.as_file().sync_all()?;
        file.persist(self.state_root.join(REGISTRY_FILE))
            .map_err(|error| error.error)?;
        File::open(&self.state_root)?.sync_all()
    }
}

fn refuse_metadata_symlink(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_symlink() || !metadata.is_file() => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "managed storage metadata is not a regular file",
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn initialize_owner_marker(root: &Path, owner: &str) -> io::Result<()> {
    let marker = root.join(OWNER_MARKER);
    if let Ok(metadata) = fs::symlink_metadata(&marker)
        && (!metadata.is_file() || metadata.is_symlink())
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "storage owner marker is not a regular owned file",
        ));
    }
    match fs::read_to_string(&marker) {
        Ok(existing) if existing.trim() == owner => Ok(()),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "storage state root has a foreign ownership marker",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            // Only an empty fresh directory may acquire the controller owner marker.
            // In particular, never adopt an unmarked legacy or foreign store.
            if fs::read_dir(root)?.next().is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "unmarked nonempty storage root cannot be adopted",
                ));
            }
            let mut options = OpenOptions::new();
            options.create_new(true).write(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(marker)?;
            writeln!(file, "{owner}")?;
            file.sync_all()
        }
        Err(error) => Err(error),
    }
}

fn inventory_from(document: &StorageRegistryDocument, now: u64) -> StorageInventory {
    let mut inventory = StorageInventory {
        active_count: 0,
        retained_count: 0,
        reclaimable_count: 0,
        protected_recovery_count: 0,
        managed_logical_bytes: 0,
        reclaimable_logical_bytes: 0,
        historical_peak_managed_bytes: document.historical_peak_managed_bytes,
        recent_growth_bytes: 0,
    };
    for record in &document.resources {
        if record.lifecycle_state == ManagedResourceState::Removed {
            continue;
        }
        inventory.managed_logical_bytes = inventory
            .managed_logical_bytes
            .saturating_add(record.logical_bytes);
        if record.lifecycle_state == ManagedResourceState::Active || record.lease.is_some() {
            inventory.active_count += 1;
        }
        if record.lifecycle_state == ManagedResourceState::Retained
            || record.retention == ResourceRetention::Explicit
        {
            inventory.retained_count += 1;
        }
        if record.lifecycle_state == ManagedResourceState::RecoveryRequired
            || record.contains_source_changes
            || record.required_for_evidence
            || record.retention == ResourceRetention::PreserveForRecovery
        {
            inventory.protected_recovery_count += 1;
        }
        if record.safe_cleanup_candidate(now) {
            inventory.reclaimable_count += 1;
            inventory.reclaimable_logical_bytes = inventory
                .reclaimable_logical_bytes
                .saturating_add(record.logical_bytes);
        }
    }
    inventory
}

fn validate_transition(from: ManagedResourceState, to: ManagedResourceState) -> io::Result<()> {
    let allowed = match from {
        ManagedResourceState::Active => matches!(
            to,
            ManagedResourceState::Completed
                | ManagedResourceState::RecoveryRequired
                | ManagedResourceState::Retained
        ),
        ManagedResourceState::RecoveryRequired => {
            matches!(
                to,
                ManagedResourceState::Active | ManagedResourceState::Completed
            )
        }
        ManagedResourceState::Retained => {
            matches!(
                to,
                ManagedResourceState::Active | ManagedResourceState::Completed
            )
        }
        ManagedResourceState::Completed => matches!(
            to,
            ManagedResourceState::Active
                | ManagedResourceState::Expired
                | ManagedResourceState::Retained
                | ManagedResourceState::RecoveryRequired
        ),
        ManagedResourceState::Expired | ManagedResourceState::OrphanedOwned => matches!(
            to,
            ManagedResourceState::Active
                | ManagedResourceState::Completed
                | ManagedResourceState::RecoveryRequired
        ),
        ManagedResourceState::CleanupPending => false,
        ManagedResourceState::Removed => false,
    };
    if allowed || from == to {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid managed-resource transition {from:?} -> {to:?}"),
        ))
    }
}

fn validate_resource_boundary(
    document: &StorageRegistryDocument,
    candidate: &Path,
) -> io::Result<()> {
    validate_resource_boundary_roots(&document.authorized_roots, candidate)
}

fn validate_resource_boundary_roots(
    authorized_roots: &[PathBuf],
    candidate: &Path,
) -> io::Result<()> {
    if authorized_roots
        .iter()
        .any(|root| candidate == root || candidate.starts_with(root))
    {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "managed resource is outside declared ownership roots: {}",
            candidate.display()
        ),
    ))
}

fn canonical_boundary(path: &Path) -> io::Result<PathBuf> {
    if !path.is_absolute() || path.components().any(|part| part == Component::ParentDir) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "managed path must be absolute without traversal: {}",
                path.display()
            ),
        ));
    }
    // Resolve the parent boundary, but never follow the cleanup root itself.
    // That keeps a later symlink substitution detectable by symlink_metadata
    // instead of resolving it outside the registry and poisoning every read.
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "managed path has no parent"))?;
    let parent = fs::canonicalize(parent)?;
    let leaf = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "managed path has no leaf"))?;
    Ok(parent.join(leaf))
}

#[cfg(target_os = "linux")]
fn process_start_identity(pid: u32) -> Option<String> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // comm may contain spaces and parentheses; fields after the final ')' begin
    // with field 3. starttime is field 22, therefore index 19 in this suffix.
    stat.rsplit_once(')')?
        .1
        .split_whitespace()
        .nth(19)
        .map(str::to_owned)
}

#[cfg(not(target_os = "linux"))]
fn process_start_identity(_pid: u32) -> Option<String> {
    None
}

fn lease_owner_is_live(lease: &ResourceLeaseIdentity) -> bool {
    #[cfg(target_os = "linux")]
    {
        let Some(expected) = lease.process_start_identity.as_ref() else {
            return true; // No start token: a live PID cannot be disproved.
        };
        if let Some(observed) = process_start_identity(lease.owner_pid) {
            return *expected == observed;
        }
        // Only absence of the PID while this process can observe procfs
        // proves a dead owner. A denied/malformed stat read stays ambiguous.
        fs::metadata("/proc/self/stat").is_err()
            || !matches!(
                fs::metadata(format!("/proc/{}", lease.owner_pid)),
                Err(error) if error.kind() == io::ErrorKind::NotFound
            )
    }
    #[cfg(not(target_os = "linux"))]
    {
        // Without a platform start-token observer, fail closed: an ambiguous
        // lease stays active and cannot become a cleanup candidate.
        let _ = lease;
        true
    }
}

fn validate_identifier(value: &str, label: &str) -> io::Result<()> {
    if value.is_empty()
        || value.len() > 160
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid {label}"),
        ));
    }
    Ok(())
}

fn remove_owned_path(path: &Path, metadata: &fs::Metadata) -> io::Result<()> {
    if metadata.is_file() {
        return fs::remove_file(path);
    }
    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "managed cleanup root is not a regular file or directory",
        ));
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        let metadata = fs::symlink_metadata(&child)?;
        if metadata.file_type().is_symlink() || metadata.is_file() {
            fs::remove_file(&child)?;
        } else if metadata.is_dir() {
            remove_owned_path(&child, &metadata)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("refusing to remove special file {}", child.display()),
            ));
        }
    }
    fs::remove_dir(path)
}

fn existing_ancestor(path: &Path) -> io::Result<PathBuf> {
    let mut current = path;
    loop {
        if current.exists() {
            return current.canonicalize();
        }
        current = current.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "no existing filesystem ancestor")
        })?;
    }
}

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry(root: &Path, managed: &Path) -> StorageResourceRegistry {
        fs::create_dir_all(managed).unwrap();
        StorageResourceRegistry::open(
            root.join("state"),
            "jarvis.release-recovery.storage.v1",
            &[managed.to_path_buf()],
        )
        .unwrap()
    }

    fn disposable(
        id: &str,
        operation: &str,
        path: PathBuf,
        class: ManagedResourceClass,
        logical_bytes: u64,
    ) -> ManagedResourceRecord {
        let mut record = ManagedResourceRecord::new(
            id,
            operation,
            path,
            class,
            "jarvis.release-recovery.storage.v1",
            ManagedResourceState::Completed,
            ResourceRetention::UntilSettled,
        );
        record.cleanup_eligible = true;
        record.logical_bytes = logical_bytes;
        record
    }

    #[test]
    fn proactive_cleanup_removes_only_registered_disposable_resources() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let disposable_path = managed.join("staging-1");
        fs::create_dir(&disposable_path).unwrap();
        fs::write(disposable_path.join("payload"), vec![0_u8; 4096]).unwrap();
        let unknown = managed.join("unowned.txt");
        fs::write(&unknown, b"owner data").unwrap();
        registry
            .register(disposable(
                "staging-1",
                "epoch-1",
                disposable_path.clone(),
                ManagedResourceClass::Staging,
                4096,
            ))
            .unwrap();

        let report = registry.reclaim(&managed, u64::MAX, 0).unwrap();

        assert_eq!(report.removed_resource_ids, ["staging-1"]);
        assert!(!disposable_path.exists());
        assert_eq!(fs::read(&unknown).unwrap(), b"owner data");
        assert_eq!(report.logical_bytes_removed, 4096);
        assert_eq!(
            report.actual_available_bytes_gained,
            report
                .available_bytes_after
                .saturating_sub(report.available_bytes_before),
            "actual capacity is remeasured and never inferred from logical bytes"
        );
    }

    #[test]
    fn active_recovery_evidence_and_explicit_retention_are_preserved() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        for (id, state) in [
            ("active", ManagedResourceState::Active),
            ("recovery", ManagedResourceState::RecoveryRequired),
            ("evidence", ManagedResourceState::Completed),
            ("retained", ManagedResourceState::Retained),
        ] {
            let path = managed.join(id);
            fs::create_dir(&path).unwrap();
            fs::write(path.join("data"), id).unwrap();
            let mut record = disposable(
                id,
                "epoch-protected",
                path,
                ManagedResourceClass::OperationScratch,
                1024,
            );
            record.lifecycle_state = state;
            if id == "active" {
                record.lease = Some(ResourceLeaseIdentity {
                    lease_id: "lease-1".into(),
                    owner_pid: 999_999,
                    process_start_identity: Some("start-token".into()),
                    operation_id: "epoch-protected".into(),
                });
            }
            if id == "evidence" {
                record.required_for_evidence = true;
            }
            if id == "retained" {
                record.retention = ResourceRetention::Explicit;
            }
            registry.register(record).unwrap();
        }

        let report = registry.reclaim(&managed, u64::MAX, 0).unwrap();
        assert_eq!(report.removed, 0);
        for id in ["active", "recovery", "evidence", "retained"] {
            assert!(managed.join(id).exists(), "{id} must be preserved");
        }
    }

    #[cfg(unix)]
    #[test]
    fn registry_metadata_symlinks_and_unmarked_store_are_not_adopted() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        fs::create_dir(&managed).unwrap();
        let foreign = temp.path().join("foreign");
        fs::create_dir(&foreign).unwrap();
        fs::write(
            foreign.join(OWNER_MARKER),
            b"jarvis.release-recovery.storage.v1\n",
        )
        .unwrap();
        let redirected = temp.path().join("redirected");
        symlink(&foreign, &redirected).unwrap();
        assert_eq!(
            StorageResourceRegistry::open(
                &redirected,
                "jarvis.release-recovery.storage.v1",
                std::slice::from_ref(&managed),
            )
            .unwrap_err()
            .kind(),
            io::ErrorKind::PermissionDenied
        );
        let marker_link = temp.path().join("marker-link");
        fs::create_dir(&marker_link).unwrap();
        symlink(foreign.join(OWNER_MARKER), marker_link.join(OWNER_MARKER)).unwrap();
        assert_eq!(
            StorageResourceRegistry::open(
                marker_link,
                "jarvis.release-recovery.storage.v1",
                std::slice::from_ref(&managed),
            )
            .unwrap_err()
            .kind(),
            io::ErrorKind::PermissionDenied
        );
        let registry = registry(temp.path(), &managed);
        let outside = temp.path().join("outside");
        let original = fs::read(registry.state_root().join(REGISTRY_FILE)).unwrap();
        fs::write(&outside, &original).unwrap();
        fs::remove_file(registry.state_root().join(REGISTRY_FILE)).unwrap();
        symlink(&outside, registry.state_root().join(REGISTRY_FILE)).unwrap();
        assert!(registry.observe_inventory().is_err());
        assert_eq!(fs::read(&outside).unwrap(), original);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_cleanup_root_fails_closed_without_touching_target() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let outside = temp.path().join("outside");
        fs::create_dir(&outside).unwrap();
        fs::write(outside.join("keep"), b"keep").unwrap();
        let link = managed.join("link");
        symlink(&outside, &link).unwrap();
        let record = disposable(
            "link",
            "epoch-link",
            link.clone(),
            ManagedResourceClass::Staging,
            1024,
        );
        registry.register(record).unwrap();

        let report = registry.reclaim(&managed, u64::MAX, 0).unwrap();
        assert_eq!(report.removed, 0);
        assert_eq!(report.failed, 1);
        assert_eq!(fs::read(outside.join("keep")).unwrap(), b"keep");
        assert_eq!(
            registry.resources().unwrap()[0].lifecycle_state,
            ManagedResourceState::Completed,
            "a refused symlink cleanup must remain readable and retry-bounded"
        );
    }

    #[test]
    fn restart_reconciles_interrupted_cleanup_without_duplicate_allocation() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry_store = registry(temp.path(), &managed);
        let removed = managed.join("removed");
        fs::create_dir(&removed).unwrap();
        registry_store
            .register(disposable(
                "removed",
                "epoch-restart",
                removed.clone(),
                ManagedResourceClass::SyntheticArtifact,
                10,
            ))
            .unwrap();
        registry_store
            .update("removed", |record| {
                record.lifecycle_state = ManagedResourceState::CleanupPending;
                Ok(())
            })
            .unwrap();
        fs::remove_dir(&removed).unwrap();

        let reopened = registry(temp.path(), &managed);
        reopened.reconcile_interrupted_cleanup().unwrap();
        let records = reopened.resources().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].lifecycle_state, ManagedResourceState::Removed);
    }

    #[test]
    fn existing_unmarked_storage_and_stale_atomic_write_do_not_get_adopted_or_blocked() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        fs::create_dir(&managed).unwrap();
        let foreign = temp.path().join("unmarked");
        fs::create_dir(&foreign).unwrap();
        fs::write(foreign.join("keep"), b"foreign").unwrap();
        assert_eq!(
            StorageResourceRegistry::open(
                &foreign,
                "jarvis.release-recovery.storage.v1",
                std::slice::from_ref(&managed),
            )
            .unwrap_err()
            .kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(fs::read(foreign.join("keep")).unwrap(), b"foreign");
        assert!(!foreign.join(OWNER_MARKER).exists());

        let registry = registry(temp.path(), &managed);
        let stale = registry
            .state_root()
            .join(format!(".{REGISTRY_FILE}.{}.tmp", std::process::id()));
        fs::write(&stale, b"stale interrupted write").unwrap();
        registry.observe_inventory().unwrap();
        assert_eq!(fs::read(&stale).unwrap(), b"stale interrupted write");
    }

    #[test]
    fn malformed_registry_and_foreign_marker_fail_closed() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        fs::write(registry.state_root().join(REGISTRY_FILE), b"{broken").unwrap();
        assert_eq!(
            registry.resources().unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );

        let foreign = temp.path().join("foreign-state");
        fs::create_dir(&foreign).unwrap();
        fs::write(foreign.join(OWNER_MARKER), b"agent-vesper\n").unwrap();
        assert_eq!(
            StorageResourceRegistry::open(
                foreign,
                "jarvis.release-recovery.storage.v1",
                &[managed],
            )
            .unwrap_err()
            .kind(),
            io::ErrorKind::PermissionDenied
        );
    }

    #[test]
    fn foreign_owned_record_cannot_be_registered_or_loaded_for_reclamation() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("foreign");
        fs::create_dir(&path).unwrap();
        fs::write(path.join("keep"), b"foreign").unwrap();
        let mut record = disposable(
            "foreign",
            "epoch-foreign",
            path.clone(),
            ManagedResourceClass::Staging,
            1,
        );
        record.ownership_identity = "other.owner".into();
        assert_eq!(
            registry.register(record).unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(fs::read(path.join("keep")).unwrap(), b"foreign");
        let owned = disposable(
            "owned",
            "epoch-foreign",
            path.clone(),
            ManagedResourceClass::Staging,
            1,
        );
        registry.register(owned).unwrap();
        let document_path = registry.state_root().join(REGISTRY_FILE);
        let mut document: serde_json::Value =
            serde_json::from_slice(&fs::read(&document_path).unwrap()).unwrap();
        document["resources"][0]["ownership_identity"] = "other.owner".into();
        fs::write(&document_path, serde_json::to_vec(&document).unwrap()).unwrap();
        assert_eq!(
            registry.resources().unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(fs::read(path.join("keep")).unwrap(), b"foreign");
    }

    #[test]
    fn cleanup_pending_resource_cannot_be_leased_or_reactivated() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("candidate");
        fs::create_dir(&path).unwrap();
        registry
            .register(disposable(
                "candidate",
                "epoch-cleanup",
                path,
                ManagedResourceClass::OperationScratch,
                1,
            ))
            .unwrap();
        registry
            .update("candidate", |record| {
                record.lifecycle_state = ManagedResourceState::CleanupPending;
                Ok(())
            })
            .unwrap();
        let lease = ResourceLeaseIdentity {
            lease_id: "late-worker".into(),
            owner_pid: std::process::id(),
            process_start_identity: None,
            operation_id: "epoch-cleanup".into(),
        };
        assert_eq!(
            registry
                .acquire_lease("candidate", lease)
                .unwrap_err()
                .kind(),
            io::ErrorKind::WouldBlock
        );
        assert!(registry.resources().unwrap()[0].lease.is_none());
    }

    #[test]
    fn concurrent_registration_is_serialized_and_duplicate_paths_refuse() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("one");
        fs::create_dir(&path).unwrap();
        registry
            .register(disposable(
                "one",
                "epoch-concurrent",
                path.clone(),
                ManagedResourceClass::OperationScratch,
                1,
            ))
            .unwrap();
        let error = registry
            .register(disposable(
                "two",
                "epoch-concurrent",
                path,
                ManagedResourceClass::OperationScratch,
                1,
            ))
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    }

    #[test]
    fn lease_drop_settles_active_resource_without_making_it_reclaimable() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("target-observation");
        fs::create_dir(&path).unwrap();
        let mut record = disposable(
            "target-observation",
            "epoch-lease",
            path,
            ManagedResourceClass::CargoTargetObservation,
            1,
        );
        record.lifecycle_state = ManagedResourceState::Retained;
        record.retention = ResourceRetention::Explicit;
        record.cleanup_eligible = false;
        registry.register(record).unwrap();
        {
            let _lease = registry
                .lease(
                    "target-observation",
                    "epoch-lease",
                    ManagedResourceState::Retained,
                    false,
                )
                .unwrap();
            assert_eq!(registry.observe_inventory().unwrap().active_count, 1);
        }
        let records = registry.resources().unwrap();
        assert!(records[0].lease.is_none());
        assert_eq!(records[0].lifecycle_state, ManagedResourceState::Retained);
        assert!(!records[0].cleanup_eligible);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn missing_start_identity_does_not_clear_a_live_owner_lease() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("active");
        fs::create_dir(&path).unwrap();
        let mut record = disposable(
            "active",
            "epoch-live",
            path,
            ManagedResourceClass::RepairWorkspace,
            1,
        );
        record.lifecycle_state = ManagedResourceState::Active;
        record.cleanup_eligible = false;
        record.lease = Some(ResourceLeaseIdentity {
            lease_id: "ambiguous".into(),
            owner_pid: std::process::id(),
            process_start_identity: None,
            operation_id: "epoch-live".into(),
        });
        registry.register(record).unwrap();
        assert_eq!(registry.reconcile_stale_leases().unwrap(), 0);
        assert!(registry.resources().unwrap()[0].lease.is_some());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn restart_reconciliation_retains_stale_lease_for_recovery() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("repair");
        fs::create_dir(&path).unwrap();
        let mut record = disposable(
            "repair",
            "epoch-restart",
            path,
            ManagedResourceClass::RepairWorkspace,
            1,
        );
        record.lifecycle_state = ManagedResourceState::Active;
        record.cleanup_eligible = false;
        record.lease = Some(ResourceLeaseIdentity {
            lease_id: "dead-owner".into(),
            owner_pid: u32::MAX,
            process_start_identity: Some("not-live".into()),
            operation_id: "epoch-restart".into(),
        });
        registry.register(record).unwrap();

        assert_eq!(registry.reconcile_stale_leases().unwrap(), 1);
        let records = registry.resources().unwrap();
        assert!(records[0].lease.is_none());
        assert_eq!(
            records[0].lifecycle_state,
            ManagedResourceState::RecoveryRequired
        );
        assert!(!records[0].cleanup_eligible);
    }

    #[test]
    fn logical_reclamation_and_actual_capacity_are_distinct_receipts() {
        let temp = tempfile::tempdir().unwrap();
        let managed = temp.path().join("managed");
        let registry = registry(temp.path(), &managed);
        let path = managed.join("logical");
        fs::create_dir(&path).unwrap();
        fs::write(path.join("sparse"), b"small physical allocation").unwrap();
        registry
            .register(disposable(
                "logical",
                "epoch-btrfs",
                path,
                ManagedResourceClass::CargoCacheSegment,
                100 * 1024 * 1024 * 1024,
            ))
            .unwrap();

        let report = registry.reclaim(&managed, u64::MAX, 0).unwrap();
        assert_eq!(report.logical_bytes_removed, 100 * 1024 * 1024 * 1024);
        assert_ne!(
            report.actual_available_bytes_gained, report.logical_bytes_removed,
            "logical bytes must not be fabricated as filesystem capacity"
        );
    }
}
