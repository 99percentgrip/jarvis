//! Validation for application-private storage, never for user tool/workspace paths.
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

fn denied() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "private application storage ownership refused",
    )
}

const OWNER_FILE: &str = ".jarvis-owner";
const OWNER: &[u8] = b"99percentgrip.jarvis/runtime-v1\n";

/// Owned ephemeral storage; cleanup applies only to the freshly allocated directory.
pub struct PrivateTemporaryDirectory(tempfile::TempDir);

impl PrivateTemporaryDirectory {
    pub fn path(&self) -> &Path {
        self.0.path()
    }
}

pub fn private_temporary_directory(prefix: &str) -> io::Result<PrivateTemporaryDirectory> {
    private_temporary_directory_in(prefix, &std::env::temp_dir())
}

fn private_temporary_directory_in(
    prefix: &str,
    base: &Path,
) -> io::Result<PrivateTemporaryDirectory> {
    let directory = private_temporary_workspace_in(prefix, base)?;
    prepare_owned_directory(directory.path())?;
    Ok(PrivateTemporaryDirectory(directory))
}

/// Fresh capability-owned workspace, without metadata inside a copied project tree.
/// The returned exclusive RAII allocation is its ownership evidence; it never opens
/// or adopts an existing directory. Native ACLs are secured before caller writes.
pub fn private_temporary_workspace(prefix: &str) -> io::Result<tempfile::TempDir> {
    private_temporary_workspace_in(prefix, &std::env::temp_dir())
}

fn private_temporary_workspace_in(prefix: &str, base: &Path) -> io::Result<tempfile::TempDir> {
    validate_private_path(base)?;
    if prefix.is_empty()
        || prefix.len() > 64
        || !prefix
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err(denied());
    }
    let directory = tempfile::Builder::new().prefix(prefix).tempdir_in(base)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(directory.path(), fs::Permissions::from_mode(0o700))?;
    }
    #[cfg(windows)]
    crate::private_state_windows::secure_empty(directory.path())?;
    prepare_private_directory(directory.path())?;
    Ok(directory)
}

/// Same-directory staged replacement for application-private configuration.
pub fn write_private_atomic(target: &Path, bytes: &[u8]) -> io::Result<()> {
    use io::Write;
    validate_private_path(target)?;
    let parent = target.parent().ok_or_else(denied)?;
    prepare_owned_directory(parent)?;
    let mut staged = tempfile::NamedTempFile::new_in(parent)?;
    staged.write_all(bytes)?;
    staged.as_file().sync_all()?;
    staged.persist(target).map_err(|error| error.error)?;
    #[cfg(unix)]
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

/// Startup discovery is read-only; a missing or empty private root has no records.
pub fn validate_new_or_owned_directory(path: &Path) -> io::Result<()> {
    validate_private_path(path)?;
    if path.exists() && fs::read_dir(path)?.next().is_some() {
        validate_owned_directory(path)?;
    }
    Ok(())
}

/// Existing application stores require our marker; discovery never creates one.
pub fn validate_owned_directory(path: &Path) -> io::Result<()> {
    validate_private_path(path)?;
    if !path.exists() {
        return Ok(());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::metadata(path)?;
        if metadata.uid() != rustix::process::geteuid().as_raw() || metadata.mode() & 0o077 != 0 {
            return Err(denied());
        }
    }
    #[cfg(windows)]
    crate::private_state_windows::verify(path)?;
    let marker = path.join(OWNER_FILE);
    validate_private_path(&marker)?;
    if fs::metadata(&marker)?.len() != OWNER.len() as u64 || fs::read(marker)? != OWNER {
        return Err(denied());
    }
    Ok(())
}

/// Initialize empty private storage. Nonempty unowned storage requires Stage 4 import.
pub fn prepare_owned_directory(path: &Path) -> io::Result<()> {
    validate_private_path(path)?;
    // An override may insert intermediate directories below the known private
    // application boundary. Establish/refuse that owner before any child write.
    if let Some(parent) = path.ancestors().skip(1).find(|ancestor| {
        ancestor
            .file_name()
            .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case(".jarvis"))
    }) {
        prepare_owned_directory(parent)?;
    }
    if path.exists() && fs::read_dir(path)?.next().is_some() {
        return validate_owned_directory(path);
    }
    prepare_private_directory(path)?;
    // The path was empty/uninitialized: only this explicitly designated private
    // root may be restricted. Never chmod its existing ancestors or nonempty data.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    #[cfg(windows)]
    crate::private_state_windows::secure_empty(path)?;
    let parent = path.parent().ok_or_else(denied)?;
    let mut staged = tempfile::NamedTempFile::new_in(parent)?;
    {
        use io::Write;
        staged.write_all(OWNER)?;
        staged.as_file().sync_all()?;
    }
    #[cfg(windows)]
    crate::private_state_windows::secure_empty(staged.path())?;
    match staged.persist_noclobber(path.join(OWNER_FILE)) {
        Ok(file) => {
            file.sync_all()?;
        }
        Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e.error),
    }
    validate_owned_directory(path)
}

/// Reject traversal, legacy-private namespaces and redirected storage components.
/// This does not grant tool authority or constrain ordinary user project access.
pub fn validate_private_path(path: &Path) -> io::Result<()> {
    if !path.is_absolute() || is_legacy_private_namespace(path) {
        return Err(denied());
    }
    let mut current = PathBuf::new();
    for part in path.components() {
        if matches!(part, Component::ParentDir | Component::CurDir) {
            return Err(denied());
        }
        let text = part.as_os_str().to_string_lossy().to_ascii_lowercase();
        if [".agent-vesper", ".glm-acp", ".vesper-scope-id"].contains(&text.as_str()) {
            return Err(denied());
        }
        current.push(part);
        if !matches!(part, Component::Normal(_)) {
            continue;
        }
        match fs::symlink_metadata(&current) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    // macOS system aliases precede any application-owned boundary.
                    if cfg!(target_os = "macos")
                        && [Path::new("/var"), Path::new("/tmp")].contains(&current.as_path())
                    {
                        continue;
                    }
                    return Err(denied());
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    if meta.file_attributes() & 0x400 != 0 {
                        return Err(denied());
                    }
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    if [".jarvis", "jarvis"].contains(&text.as_str())
                        && meta.uid() != rustix::process::geteuid().as_raw()
                    {
                        return Err(denied());
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Lexical application ownership only; suitable for explicitly selected shared
/// protocol endpoints without imposing private-store containment on user projects.
pub fn is_legacy_private_namespace(path: &Path) -> bool {
    if path.components().any(|part| {
        matches!(
            part.as_os_str()
                .to_string_lossy()
                .to_ascii_lowercase()
                .as_str(),
            ".agent-vesper" | ".glm-acp" | ".vesper-scope-id"
        )
    }) {
        return true;
    }
    let normalized = path
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    [
        "/.config/agent-vesper",
        "/.local/share/agent-vesper",
        "/.local/state/agent-vesper",
        "/.cache/agent-vesper",
        "/application support/agent vesper",
        "/application support/agent-vesper",
        "/application support/agentvesper",
        "/library/caches/agent vesper",
        "/library/caches/agent-vesper",
        "/library/caches/agentvesper",
        "/library/logs/agent vesper",
        "/library/logs/agent-vesper",
        "/library/logs/agentvesper",
        "/appdata/roaming/agent vesper",
        "/appdata/roaming/agent-vesper",
        "/appdata/local/agent-vesper",
        "/appdata/local/programs/agentvesper",
        "/appdata/local/agent vesper",
    ]
    .iter()
    .any(|s| {
        normalized.find(s).is_some_and(|i| {
            normalized
                .as_bytes()
                .get(i + s.len())
                .is_none_or(|c| *c == b'/')
        })
    })
}

/// Verify an existing sensitive file before reading it; never change its ACL/mode.
pub fn validate_private_file(path: &Path) -> io::Result<()> {
    validate_private_path(path)?;
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() {
        return Err(denied());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.uid() != rustix::process::geteuid().as_raw() || metadata.mode() & 0o077 != 0 {
            return Err(denied());
        }
    }
    #[cfg(windows)]
    crate::private_state_windows::verify(path)?;
    Ok(())
}

/// Create only validated application-private directories with restrictive Unix modes.
/// Existing entries are never redirected, chmodded or adopted from legacy state.
pub fn prepare_private_directory(path: &Path) -> io::Result<()> {
    validate_private_path(path)?;
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        prepare_private_directory(parent)?;
    }
    if !path.exists() {
        let builder = fs::DirBuilder::new();
        #[cfg(unix)]
        let mut builder = builder;
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        match builder.create(path) {
            Ok(()) => {
                #[cfg(windows)]
                crate::private_state_windows::secure_empty(path)?;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    validate_private_path(path)?;
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() {
        return Err(denied());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if meta.uid() != rustix::process::geteuid().as_raw() || meta.mode() & 0o022 != 0 {
            return Err(denied());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporary_allocation_refuses_legacy_roots_before_creating_any_entry() {
        let fixture = tempfile::tempdir().unwrap();
        let legacy = fixture.path().join(".agent-vesper");
        std::fs::create_dir(&legacy).unwrap();
        std::fs::write(legacy.join("canary"), b"private legacy state").unwrap();
        assert!(private_temporary_directory_in("owned-fixture-", &legacy).is_err());
        assert!(private_temporary_workspace_in("owned-fixture-", &legacy).is_err());
        assert_eq!(std::fs::read_dir(&legacy).unwrap().count(), 1);
        assert_eq!(
            std::fs::read(legacy.join("canary")).unwrap(),
            b"private legacy state"
        );
    }

    #[test]
    fn exclusive_temporary_workspace_keeps_project_contents_metadata_free() {
        let base = tempfile::tempdir().unwrap();
        let workspace = private_temporary_workspace_in("owned-verifier-", base.path()).unwrap();
        let path = workspace.path().to_path_buf();
        assert_eq!(std::fs::read_dir(&path).unwrap().count(), 0);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(&path).unwrap().permissions().mode() & 0o077,
                0
            );
        }
        #[cfg(windows)]
        crate::private_state_windows::verify(&path).unwrap();
        drop(workspace);
        assert!(!path.exists());
        assert_eq!(std::fs::read_dir(base.path()).unwrap().count(), 0);
    }

    #[test]
    fn empty_platform_legacy_roots_are_refused_before_first_run_mutation() {
        let fixture = tempfile::tempdir().unwrap();
        for suffix in [
            ".config/agent-vesper",
            ".local/share/agent-vesper",
            ".cache/agent-vesper",
            ".local/state/agent-vesper",
            "Library/Application Support/Agent Vesper",
            "Library/Caches/Agent Vesper",
            "Library/Logs/Agent Vesper",
            "AppData/Roaming/Agent Vesper",
            "AppData/Local/Agent Vesper",
        ] {
            let root = fixture.path().join(suffix);
            fs::create_dir_all(&root).unwrap();
            assert!(prepare_owned_directory(&root).is_err(), "{suffix}");
            assert_eq!(fs::read_dir(root).unwrap().count(), 0);
        }
    }

    #[test]
    fn atomic_private_replacement_preserves_whole_records_and_ownership() {
        let fixture = tempfile::tempdir().unwrap();
        let root = fixture.path().join("private");
        let path = root.join("settings.json");
        let first = vec![b'a'; 8192];
        let second = vec![b'b'; 8192];
        write_private_atomic(&path, &first).unwrap();
        std::thread::scope(|scope| {
            for bytes in [&first, &second] {
                let path = &path;
                scope.spawn(move || {
                    for _ in 0..8 {
                        write_private_atomic(path, bytes).unwrap();
                    }
                });
            }
            for _ in 0..32 {
                let read = fs::read(&path).unwrap();
                assert!(read == first || read == second);
            }
        });
        validate_private_file(&path).unwrap();
        validate_owned_directory(&root).unwrap();
        assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    }

    #[test]
    fn ownership_publication_is_atomic_and_concurrent() {
        let root = tempfile::tempdir().unwrap();
        let private = root.path().join(".jarvis");
        std::thread::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|| prepare_owned_directory(&private).unwrap());
            }
        });
        validate_owned_directory(&private).unwrap();
        assert_eq!(fs::read(private.join(OWNER_FILE)).unwrap(), OWNER);
        assert_eq!(fs::read_dir(&private).unwrap().count(), 1);
    }

    #[test]
    fn deep_private_child_initializes_application_owner_before_publication() {
        for name in [".jarvis".to_owned(), ".jarvis".to_ascii_uppercase()] {
            let fixture = tempfile::tempdir().unwrap();
            let application = fixture.path().join(name);
            let child = application.join("deep/settings");
            prepare_owned_directory(&child).unwrap();
            validate_owned_directory(&application).unwrap();
            validate_owned_directory(&child).unwrap();
        }
    }

    #[test]
    fn deep_private_child_refuses_foreign_ancestor_without_mutation() {
        for name in [".jarvis".to_owned(), ".jarvis".to_ascii_uppercase()] {
            let fixture = tempfile::tempdir().unwrap();
            let application = fixture.path().join(name);
            fs::create_dir(&application).unwrap();
            let canary = application.join("session.json");
            fs::write(&canary, b"synthetic-unowned-state").unwrap();
            let child = application.join("deep/settings");
            assert!(prepare_owned_directory(&child).is_err());
            assert!(write_private_atomic(&child.join("settings.json"), b"new").is_err());
            assert_eq!(fs::read(&canary).unwrap(), b"synthetic-unowned-state");
            assert_eq!(fs::read_dir(&application).unwrap().count(), 1);
            assert!(!application.join("deep").exists());
        }
    }

    #[test]
    fn nonempty_unowned_directory_is_never_adopted() {
        let root = tempfile::tempdir().unwrap();
        let private = root.path().join(".jarvis");
        fs::create_dir(&private).unwrap();
        fs::write(
            private.join("session.json"),
            b"synthetic-interrupted-operation",
        )
        .unwrap();
        assert!(prepare_owned_directory(&private).is_err());
        assert_eq!(
            fs::read(private.join("session.json")).unwrap(),
            b"synthetic-interrupted-operation"
        );
        assert!(!private.join(OWNER_FILE).exists());
    }

    #[test]
    fn first_run_is_idempotent_and_concurrent() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join(".jarvis/memory");
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| prepare_private_directory(&path).unwrap());
            }
        });
        prepare_private_directory(&path).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o700
            );
        }
    }

    #[test]
    fn legacy_private_state_and_traversal_are_refused_without_mutation() {
        let root = tempfile::tempdir().unwrap();
        let old = root.path().join(".agent-vesper");
        fs::create_dir(&old).unwrap();
        fs::write(old.join("sessions.json"), b"synthetic-old-operation").unwrap();
        assert!(prepare_private_directory(&old).is_err());
        assert!(prepare_private_directory(&root.path().join(".jarvis/../escape")).is_err());
        assert_eq!(
            fs::read(old.join("sessions.json")).unwrap(),
            b"synthetic-old-operation"
        );
        assert_eq!(fs::read_dir(old).unwrap().count(), 1);
        // The same user-owned project remains a valid tool/workspace location.
        assert!(root.path().is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn redirected_private_state_never_touches_legacy_target() {
        use std::os::unix::fs::symlink;
        let root = tempfile::tempdir().unwrap();
        let old = root.path().join(".agent-vesper");
        fs::create_dir(&old).unwrap();
        fs::write(old.join("credentials"), b"synthetic-secret").unwrap();
        symlink(&old, root.path().join(".jarvis")).unwrap();
        assert!(prepare_private_directory(&root.path().join(".jarvis/checkpoints")).is_err());
        assert_eq!(
            fs::read(old.join("credentials")).unwrap(),
            b"synthetic-secret"
        );
        assert!(!old.join("checkpoints").exists());
    }
}
