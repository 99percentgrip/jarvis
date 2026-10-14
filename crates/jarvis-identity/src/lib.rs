#![forbid(unsafe_code)]
//! Planned identity contracts only. No runtime composition or side effects.

mod mappings;
mod packages;
mod platform;
pub use mappings::*;
pub use packages::*;
pub use platform::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repository {
    pub owner: &'static str,
    pub name: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product {
    pub display: &'static str,
    pub slug: &'static str,
    pub repository: Repository,
    pub credential_service: &'static str,
    pub session_format: &'static str,
    pub checkpoint_namespace: &'static str,
    pub acp_id: &'static str,
    pub release_namespace: &'static str,
    pub controller_name: &'static str,
    pub session_schema_version: u32,
    pub release_owner: &'static str,
}
pub const PRODUCT: Product = Product {
    display: "J.A.R.V.I.S.",
    slug: "jarvis",
    repository: Repository {
        owner: "99percentgrip",
        name: "jarvis",
    },
    credential_service: "99percentgrip.jarvis",
    session_format: "jarvis-session",
    checkpoint_namespace: "jarvis-checkpoint",
    acp_id: "jarvis",
    release_namespace: "jarvis.release-recovery.v1",
    controller_name: "jarvis-release-controller",
    session_schema_version: 1,
    release_owner: "99percentgrip/jarvis",
};
pub const PACKAGE_OWNER: Repository = PRODUCT.repository;
pub const PACKAGE_NAMESPACE: &str = PRODUCT.slug;
pub const LEGACY_REPOSITORY: Repository = Repository {
    owner: "99percentgrip",
    name: "agent-vesper",
};
pub const SOURCE_COMMIT: &str = "692985a1ceeb2740024c8a4b26770d5afb4ee889";
pub const SOURCE_TAG_OBJECT: &str = "4f53195320e4a0a8439a933e9378655a0616cba4";
pub const SOURCE_RELEASE: &str = "v0.24.14";
pub const SOURCE_REFERENCE: &str = "upstream/vesper/v0.24.14";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportPolicy {
    pub explicit_authorization_required: bool,
    pub automatic_credential_sharing: bool,
    pub source_mutation: bool,
    pub interrupted_operation_replay: bool,
    pub release_ledger_import: bool,
    pub preserve_message_and_session_ids: bool,
}
pub const IMPORT_POLICY: ImportPolicy = ImportPolicy {
    explicit_authorization_required: true,
    automatic_credential_sharing: false,
    source_mutation: false,
    interrupted_operation_replay: false,
    release_ledger_import: false,
    preserve_message_and_session_ids: true,
};
