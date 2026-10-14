#![forbid(unsafe_code)]
//! Native OpenAI adapter components. No external agent runtime is required.
//! Host registration remains gated on the complete adapter acceptance suite.

pub mod auth;
mod catalog;
mod discovery;
pub use discovery::AvailableModels;
mod policy;
mod usage;
pub use policy::OpenAiSuperpowerPolicy;
pub mod credentials;
mod factory;
mod http_error;
mod transport;
mod wire;

pub use catalog::{DEFAULT_MODEL, OpenAiCatalog, REASONING_LEVELS};
pub use factory::OpenAiFactory;
pub use transport::OpenAiSession;

#[cfg(test)]
mod http_error_bounds_tests;
#[cfg(all(test, feature = "integration-test-harness"))]
mod http_error_parameter_tests;
#[cfg(all(test, feature = "integration-test-harness"))]
mod http_error_tests;
#[cfg(test)]
mod lens_wire_tests;
#[cfg(test)]
mod tests;

/// Stable single-provider identity, independent of billing/authentication mode.
pub fn provider_id() -> jarvis_domain::ProviderId {
    jarvis_domain::ProviderId::new("openai").expect("static provider id")
}

pub(crate) fn error(
    message: &'static str,
    category: jarvis_domain::ErrorCategory,
    visible: bool,
) -> jarvis_provider::ProviderError {
    jarvis_provider::ProviderError {
        provider_id: provider_id(),
        provider_code: None,
        http_status: None,
        continuation_possible: false,
        info: jarvis_domain::ErrorInfo {
            category,
            retryability: jarvis_domain::Retryability::Never,
            retry_after_ms: None,
            visible_output_emitted: visible,
            safe_message: jarvis_domain::SafeMessage::new(message).expect("static safe message"),
            diagnostics: Default::default(),
            provider_code: None,
            causes: vec![],
        },
        metadata: Default::default(),
    }
}
