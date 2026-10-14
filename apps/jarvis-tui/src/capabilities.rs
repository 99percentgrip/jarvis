//! Compatibility re-export of the shared provider capability index.
//!
//! Capability truth and suggestions live in `vesper-provider` so the TUI,
//! ACP host, and agent loop consume one fail-closed implementation.

pub use jarvis_provider::{CapabilityDenial, ModelCapabilityIndex};
