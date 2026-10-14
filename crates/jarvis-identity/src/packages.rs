//! Explicit reviewed first-party ownership; no runtime prefix discovery.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Executable {
    pub current: &'static str,
    pub planned: &'static str,
    pub shipping: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageRole {
    Application,
    Foundation,
    TestSupport,
    Maintenance,
    Specification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageIdentity {
    pub current: &'static str,
    pub planned: &'static str,
    pub role: PackageRole,
    pub current_import: Option<&'static str>,
    pub planned_import: Option<&'static str>,
    pub manifest: &'static str,
    pub executables: &'static [Executable],
}
pub const PACKAGES: &[PackageIdentity] = &[
    PackageIdentity {
        current: "agent-vesper-acp",
        planned: "jarvis-acp",
        role: PackageRole::Application,
        current_import: Some("agent_vesper_acp"),
        planned_import: Some("jarvis_acp"),
        manifest: "apps/agent-vesper-acp/Cargo.toml",
        executables: &[
            Executable {
                current: "agent-vesper-acp",
                planned: "jarvis-acp",
                shipping: true,
            },
            Executable {
                current: "agent-vesper-acp-test-driver",
                planned: "jarvis-acp-test-driver",
                shipping: false,
            },
        ],
    },
    PackageIdentity {
        current: "agent-vesper-tui",
        planned: "jarvis-tui",
        role: PackageRole::Application,
        current_import: Some("agent_vesper_tui"),
        planned_import: Some("jarvis_tui"),
        manifest: "apps/agent-vesper-tui/Cargo.toml",
        executables: &[Executable {
            current: "agent-vesper-tui",
            planned: "jarvis",
            shipping: true,
        }],
    },
    PackageIdentity {
        current: "vesper-config",
        planned: "jarvis-config",
        role: PackageRole::Foundation,
        current_import: Some("vesper_config"),
        planned_import: Some("jarvis_config"),
        manifest: "crates/vesper-config/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-acp",
        planned: "jarvis-acp-protocol",
        role: PackageRole::Foundation,
        current_import: Some("vesper_acp"),
        planned_import: Some("jarvis_acp_protocol"),
        manifest: "crates/vesper-acp/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-agent",
        planned: "jarvis-agent",
        role: PackageRole::Foundation,
        current_import: Some("vesper_agent"),
        planned_import: Some("jarvis_agent"),
        manifest: "crates/vesper-agent/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-auth",
        planned: "jarvis-auth",
        role: PackageRole::Foundation,
        current_import: Some("vesper_auth"),
        planned_import: Some("jarvis_auth"),
        manifest: "crates/vesper-auth/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-cognition",
        planned: "jarvis-cognition",
        role: PackageRole::Foundation,
        current_import: Some("vesper_cognition"),
        planned_import: Some("jarvis_cognition"),
        manifest: "crates/vesper-cognition/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-harness",
        planned: "jarvis-harness",
        role: PackageRole::Foundation,
        current_import: Some("vesper_harness"),
        planned_import: Some("jarvis_harness"),
        manifest: "crates/vesper-harness/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-bridge",
        planned: "jarvis-bridge",
        role: PackageRole::Foundation,
        current_import: Some("vesper_bridge"),
        planned_import: Some("jarvis_bridge"),
        manifest: "crates/vesper-bridge/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-checkpoints",
        planned: "jarvis-checkpoints",
        role: PackageRole::Foundation,
        current_import: Some("vesper_checkpoints"),
        planned_import: Some("jarvis_checkpoints"),
        manifest: "crates/vesper-checkpoints/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-domain",
        planned: "jarvis-domain",
        role: PackageRole::Foundation,
        current_import: Some("vesper_domain"),
        planned_import: Some("jarvis_domain"),
        manifest: "crates/vesper-domain/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-mcp",
        planned: "jarvis-mcp",
        role: PackageRole::Foundation,
        current_import: Some("vesper_mcp"),
        planned_import: Some("jarvis_mcp"),
        manifest: "crates/vesper-mcp/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-memory",
        planned: "jarvis-memory",
        role: PackageRole::Foundation,
        current_import: Some("vesper_memory"),
        planned_import: Some("jarvis_memory"),
        manifest: "crates/vesper-memory/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-observability",
        planned: "jarvis-observability",
        role: PackageRole::Foundation,
        current_import: Some("vesper_observability"),
        planned_import: Some("jarvis_observability"),
        manifest: "crates/vesper-observability/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-policy",
        planned: "jarvis-policy",
        role: PackageRole::Foundation,
        current_import: Some("vesper_policy"),
        planned_import: Some("jarvis_policy"),
        manifest: "crates/vesper-policy/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-provider",
        planned: "jarvis-provider",
        role: PackageRole::Foundation,
        current_import: Some("vesper_provider"),
        planned_import: Some("jarvis_provider"),
        manifest: "crates/vesper-provider/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-provider-glm",
        planned: "jarvis-provider-glm",
        role: PackageRole::Foundation,
        current_import: Some("vesper_provider_glm"),
        planned_import: Some("jarvis_provider_glm"),
        manifest: "crates/vesper-provider-glm/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-provider-openai",
        planned: "jarvis-provider-openai",
        role: PackageRole::Foundation,
        current_import: Some("vesper_provider_openai"),
        planned_import: Some("jarvis_provider_openai"),
        manifest: "crates/vesper-provider-openai/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-provider-xai",
        planned: "jarvis-provider-xai",
        role: PackageRole::Foundation,
        current_import: Some("vesper_provider_xai"),
        planned_import: Some("jarvis_provider_xai"),
        manifest: "crates/vesper-provider-xai/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-provider-synthetic",
        planned: "jarvis-provider-synthetic",
        role: PackageRole::TestSupport,
        current_import: Some("vesper_provider_synthetic"),
        planned_import: Some("jarvis_provider_synthetic"),
        manifest: "crates/vesper-provider-synthetic/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-runtime",
        planned: "jarvis-runtime",
        role: PackageRole::Foundation,
        current_import: Some("vesper_runtime"),
        planned_import: Some("jarvis_runtime"),
        manifest: "crates/vesper-runtime/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-sandbox",
        planned: "jarvis-sandbox",
        role: PackageRole::Foundation,
        current_import: Some("vesper_sandbox"),
        planned_import: Some("jarvis_sandbox"),
        manifest: "crates/vesper-sandbox/Cargo.toml",
        executables: &[Executable {
            current: "sandbox_init",
            planned: "jarvis-sandbox-init",
            shipping: true,
        }],
    },
    PackageIdentity {
        current: "vesper-sessions",
        planned: "jarvis-sessions",
        role: PackageRole::Foundation,
        current_import: Some("vesper_sessions"),
        planned_import: Some("jarvis_sessions"),
        manifest: "crates/vesper-sessions/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-security",
        planned: "jarvis-security",
        role: PackageRole::Foundation,
        current_import: Some("vesper_security"),
        planned_import: Some("jarvis_security"),
        manifest: "crates/vesper-security/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-swarm",
        planned: "jarvis-swarm",
        role: PackageRole::Foundation,
        current_import: Some("vesper_swarm"),
        planned_import: Some("jarvis_swarm"),
        manifest: "crates/vesper-swarm/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-testkit",
        planned: "jarvis-testkit",
        role: PackageRole::TestSupport,
        current_import: Some("vesper_testkit"),
        planned_import: Some("jarvis_testkit"),
        manifest: "crates/vesper-testkit/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-voice",
        planned: "jarvis-voice",
        role: PackageRole::Foundation,
        current_import: Some("vesper_voice"),
        planned_import: Some("jarvis_voice"),
        manifest: "crates/vesper-voice/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-voice-kokoro",
        planned: "jarvis-voice-kokoro",
        role: PackageRole::Foundation,
        current_import: Some("vesper_voice_kokoro"),
        planned_import: Some("jarvis_voice_kokoro"),
        manifest: "crates/vesper-voice-kokoro/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-web",
        planned: "jarvis-web",
        role: PackageRole::Foundation,
        current_import: Some("vesper_web"),
        planned_import: Some("jarvis_web"),
        manifest: "crates/vesper-web/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        current: "vesper-web-fetch",
        planned: "jarvis-web-fetch",
        role: PackageRole::Foundation,
        current_import: Some("vesper_web_fetch"),
        planned_import: Some("jarvis_web_fetch"),
        manifest: "crates/vesper-web-fetch/Cargo.toml",
        executables: &[Executable {
            current: "vesper-web-fetch",
            planned: "jarvis-web-fetch",
            shipping: true,
        }],
    },
    PackageIdentity {
        current: "xtask",
        planned: "jarvis-xtask",
        role: PackageRole::Maintenance,
        current_import: None,
        planned_import: None,
        manifest: "xtask/Cargo.toml",
        executables: &[Executable {
            current: "xtask",
            planned: "jarvis-xtask",
            shipping: false,
        }],
    },
    PackageIdentity {
        current: "jarvis-identity",
        planned: "jarvis-identity",
        role: PackageRole::Specification,
        current_import: Some("jarvis_identity"),
        planned_import: Some("jarvis_identity"),
        manifest: "crates/jarvis-identity/Cargo.toml",
        executables: &[],
    },
];

/// Excluded first-party research manifests retain historical names. They are
/// never shipping packages and require separate review before any migration.
pub const EXCLUDED_MANIFESTS: &[(&str, &str)] = &[
    (
        "spikes/acp-rust-protocol-v1/Cargo.toml",
        "acp-rust-protocol-v1-spike",
    ),
    (
        "spikes/process-sandbox/Cargo.toml",
        "vesper-process-sandbox-spike",
    ),
    (
        "spikes/rust-sse-transport/Cargo.toml",
        "rust-sse-transport-spike",
    ),
    ("spikes/sqlite-fts5/Cargo.toml", "sqlite-fts5-spike"),
    (
        "spikes/web-oracle-portability/Cargo.toml",
        "web-oracle-portability",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePackage {
    pub name: String,
    pub manifest: String,
    pub binaries: Vec<String>,
    pub library: Option<String>,
    pub custom_build: bool,
    /// Non-dev dependencies, including build dependencies that can affect shipped code.
    pub normal_dependencies: Vec<String>,
}

/// Validates injected Cargo metadata. The caller owns discovery and all I/O.
pub fn validate_workspace(actual: &[WorkspacePackage]) -> Result<(), String> {
    use std::collections::BTreeSet;
    let names: BTreeSet<_> = actual.iter().map(|p| p.name.as_str()).collect();
    let expected: BTreeSet<_> = PACKAGES.iter().map(|p| p.current).collect();
    if names != expected || actual.len() != expected.len() {
        return Err("unregistered, missing or duplicate first-party package".into());
    }
    for package in actual {
        let registered = PACKAGES
            .iter()
            .find(|p| p.current == package.name)
            .ok_or("unregistered package")?;
        if package.manifest != registered.manifest {
            return Err(format!("manifest ownership mismatch: {}", package.name));
        }
        let bins: BTreeSet<_> = package.binaries.iter().map(String::as_str).collect();
        let registered_bins: BTreeSet<_> =
            registered.executables.iter().map(|b| b.current).collect();
        if bins != registered_bins
            || package.binaries.len() != bins.len()
            || registered.executables.len() != registered_bins.len()
        {
            return Err(format!("unregistered executable: {}", package.name));
        }
        if package.library.as_deref() != registered.current_import {
            return Err(format!("unregistered library import: {}", package.name));
        }
        if package.name == "jarvis-identity"
            && (package.custom_build || !package.normal_dependencies.is_empty())
        {
            return Err(
                "identity specification must have no production dependencies or build script"
                    .into(),
            );
        }
        if package.name != "xtask"
            && package
                .normal_dependencies
                .iter()
                .any(|n| n == "jarvis-identity")
        {
            return Err("identity specification is not admitted into active runtime".into());
        }
    }
    Ok(())
}
