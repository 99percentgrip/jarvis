#![forbid(unsafe_code)]
//! Security primitives that describe authority without granting it.

mod endpoint;
mod environment;
mod output;
mod path;
mod private_state;
#[cfg(windows)]
mod private_state_windows;
mod sandbox;
mod scope;
mod secret;
mod untrusted;
mod url_redaction;

pub use endpoint::EndpointTrust;
pub use environment::{EnvironmentScrubber, ScrubbedEnvironment};
pub use output::BoundedOutput;
pub use path::{PathCapability, RelativePath, RootIdentity};
pub use private_state::write_private_atomic;
pub use private_state::{
    PrivateTemporaryDirectory, is_legacy_private_namespace, prepare_owned_directory,
    prepare_private_directory, private_temporary_directory, private_temporary_workspace,
    validate_new_or_owned_directory, validate_owned_directory, validate_private_file,
    validate_private_path,
};
pub use sandbox::{CapabilityStatus, IsolationRequirement, SandboxCapabilities, SecurityStrength};
pub use scope::{SecretScope, SecretScopeError, is_multiplex_active, set_multiplex_active};
pub use secret::{
    SecretExposure, SecretReference, SecretReferenceError, SecretSource, SecretValue,
};
pub use untrusted::UntrustedContext;
pub use url_redaction::{RedactedUrl, RedactedUrlError};
