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
    pub legacy: &'static str,
    pub legacy_manifest: &'static str,
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
        legacy: "agent-vesper-acp",
        legacy_manifest: "apps/agent-vesper-acp/Cargo.toml",
        current: "jarvis-acp",
        planned: "jarvis-acp",
        role: PackageRole::Application,
        current_import: Some("jarvis_acp"),
        planned_import: Some("jarvis_acp"),
        manifest: "apps/jarvis-acp/Cargo.toml",
        executables: &[
            Executable {
                current: "jarvis-acp",
                planned: "jarvis-acp",
                shipping: true,
            },
            Executable {
                current: "jarvis-acp-test-driver",
                planned: "jarvis-acp-test-driver",
                shipping: false,
            },
        ],
    },
    PackageIdentity {
        legacy: "agent-vesper-tui",
        legacy_manifest: "apps/agent-vesper-tui/Cargo.toml",
        current: "jarvis-tui",
        planned: "jarvis-tui",
        role: PackageRole::Application,
        current_import: Some("jarvis_tui"),
        planned_import: Some("jarvis_tui"),
        manifest: "apps/jarvis-tui/Cargo.toml",
        executables: &[Executable {
            current: "jarvis-tui",
            planned: "jarvis-tui",
            shipping: true,
        }],
    },
    PackageIdentity {
        legacy: "vesper-config",
        legacy_manifest: "crates/vesper-config/Cargo.toml",
        current: "jarvis-config",
        planned: "jarvis-config",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_config"),
        planned_import: Some("jarvis_config"),
        manifest: "crates/jarvis-config/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-acp",
        legacy_manifest: "crates/vesper-acp/Cargo.toml",
        current: "jarvis-acp-protocol",
        planned: "jarvis-acp-protocol",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_acp_protocol"),
        planned_import: Some("jarvis_acp_protocol"),
        manifest: "crates/jarvis-acp-protocol/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-agent",
        legacy_manifest: "crates/vesper-agent/Cargo.toml",
        current: "jarvis-agent",
        planned: "jarvis-agent",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_agent"),
        planned_import: Some("jarvis_agent"),
        manifest: "crates/jarvis-agent/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-auth",
        legacy_manifest: "crates/vesper-auth/Cargo.toml",
        current: "jarvis-auth",
        planned: "jarvis-auth",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_auth"),
        planned_import: Some("jarvis_auth"),
        manifest: "crates/jarvis-auth/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-cognition",
        legacy_manifest: "crates/vesper-cognition/Cargo.toml",
        current: "jarvis-cognition",
        planned: "jarvis-cognition",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_cognition"),
        planned_import: Some("jarvis_cognition"),
        manifest: "crates/jarvis-cognition/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-harness",
        legacy_manifest: "crates/vesper-harness/Cargo.toml",
        current: "jarvis-harness",
        planned: "jarvis-harness",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_harness"),
        planned_import: Some("jarvis_harness"),
        manifest: "crates/jarvis-harness/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-bridge",
        legacy_manifest: "crates/vesper-bridge/Cargo.toml",
        current: "jarvis-bridge",
        planned: "jarvis-bridge",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_bridge"),
        planned_import: Some("jarvis_bridge"),
        manifest: "crates/jarvis-bridge/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-checkpoints",
        legacy_manifest: "crates/vesper-checkpoints/Cargo.toml",
        current: "jarvis-checkpoints",
        planned: "jarvis-checkpoints",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_checkpoints"),
        planned_import: Some("jarvis_checkpoints"),
        manifest: "crates/jarvis-checkpoints/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-domain",
        legacy_manifest: "crates/vesper-domain/Cargo.toml",
        current: "jarvis-domain",
        planned: "jarvis-domain",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_domain"),
        planned_import: Some("jarvis_domain"),
        manifest: "crates/jarvis-domain/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-mcp",
        legacy_manifest: "crates/vesper-mcp/Cargo.toml",
        current: "jarvis-mcp",
        planned: "jarvis-mcp",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_mcp"),
        planned_import: Some("jarvis_mcp"),
        manifest: "crates/jarvis-mcp/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-memory",
        legacy_manifest: "crates/vesper-memory/Cargo.toml",
        current: "jarvis-memory",
        planned: "jarvis-memory",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_memory"),
        planned_import: Some("jarvis_memory"),
        manifest: "crates/jarvis-memory/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-observability",
        legacy_manifest: "crates/vesper-observability/Cargo.toml",
        current: "jarvis-observability",
        planned: "jarvis-observability",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_observability"),
        planned_import: Some("jarvis_observability"),
        manifest: "crates/jarvis-observability/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-policy",
        legacy_manifest: "crates/vesper-policy/Cargo.toml",
        current: "jarvis-policy",
        planned: "jarvis-policy",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_policy"),
        planned_import: Some("jarvis_policy"),
        manifest: "crates/jarvis-policy/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-provider",
        legacy_manifest: "crates/vesper-provider/Cargo.toml",
        current: "jarvis-provider",
        planned: "jarvis-provider",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_provider"),
        planned_import: Some("jarvis_provider"),
        manifest: "crates/jarvis-provider/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-provider-glm",
        legacy_manifest: "crates/vesper-provider-glm/Cargo.toml",
        current: "jarvis-provider-glm",
        planned: "jarvis-provider-glm",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_provider_glm"),
        planned_import: Some("jarvis_provider_glm"),
        manifest: "crates/jarvis-provider-glm/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-provider-openai",
        legacy_manifest: "crates/vesper-provider-openai/Cargo.toml",
        current: "jarvis-provider-openai",
        planned: "jarvis-provider-openai",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_provider_openai"),
        planned_import: Some("jarvis_provider_openai"),
        manifest: "crates/jarvis-provider-openai/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-provider-xai",
        legacy_manifest: "crates/vesper-provider-xai/Cargo.toml",
        current: "jarvis-provider-xai",
        planned: "jarvis-provider-xai",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_provider_xai"),
        planned_import: Some("jarvis_provider_xai"),
        manifest: "crates/jarvis-provider-xai/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-provider-synthetic",
        legacy_manifest: "crates/vesper-provider-synthetic/Cargo.toml",
        current: "jarvis-provider-synthetic",
        planned: "jarvis-provider-synthetic",
        role: PackageRole::TestSupport,
        current_import: Some("jarvis_provider_synthetic"),
        planned_import: Some("jarvis_provider_synthetic"),
        manifest: "crates/jarvis-provider-synthetic/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-runtime",
        legacy_manifest: "crates/vesper-runtime/Cargo.toml",
        current: "jarvis-runtime",
        planned: "jarvis-runtime",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_runtime"),
        planned_import: Some("jarvis_runtime"),
        manifest: "crates/jarvis-runtime/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-sandbox",
        legacy_manifest: "crates/vesper-sandbox/Cargo.toml",
        current: "jarvis-sandbox",
        planned: "jarvis-sandbox",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_sandbox"),
        planned_import: Some("jarvis_sandbox"),
        manifest: "crates/jarvis-sandbox/Cargo.toml",
        executables: &[Executable {
            current: "jarvis-sandbox-init",
            planned: "jarvis-sandbox-init",
            shipping: true,
        }],
    },
    PackageIdentity {
        legacy: "vesper-sessions",
        legacy_manifest: "crates/vesper-sessions/Cargo.toml",
        current: "jarvis-sessions",
        planned: "jarvis-sessions",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_sessions"),
        planned_import: Some("jarvis_sessions"),
        manifest: "crates/jarvis-sessions/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-security",
        legacy_manifest: "crates/vesper-security/Cargo.toml",
        current: "jarvis-security",
        planned: "jarvis-security",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_security"),
        planned_import: Some("jarvis_security"),
        manifest: "crates/jarvis-security/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-swarm",
        legacy_manifest: "crates/vesper-swarm/Cargo.toml",
        current: "jarvis-swarm",
        planned: "jarvis-swarm",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_swarm"),
        planned_import: Some("jarvis_swarm"),
        manifest: "crates/jarvis-swarm/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-testkit",
        legacy_manifest: "crates/vesper-testkit/Cargo.toml",
        current: "jarvis-testkit",
        planned: "jarvis-testkit",
        role: PackageRole::TestSupport,
        current_import: Some("jarvis_testkit"),
        planned_import: Some("jarvis_testkit"),
        manifest: "crates/jarvis-testkit/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-voice",
        legacy_manifest: "crates/vesper-voice/Cargo.toml",
        current: "jarvis-voice",
        planned: "jarvis-voice",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_voice"),
        planned_import: Some("jarvis_voice"),
        manifest: "crates/jarvis-voice/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-voice-kokoro",
        legacy_manifest: "crates/vesper-voice-kokoro/Cargo.toml",
        current: "jarvis-voice-kokoro",
        planned: "jarvis-voice-kokoro",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_voice_kokoro"),
        planned_import: Some("jarvis_voice_kokoro"),
        manifest: "crates/jarvis-voice-kokoro/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-web",
        legacy_manifest: "crates/vesper-web/Cargo.toml",
        current: "jarvis-web",
        planned: "jarvis-web",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_web"),
        planned_import: Some("jarvis_web"),
        manifest: "crates/jarvis-web/Cargo.toml",
        executables: &[],
    },
    PackageIdentity {
        legacy: "vesper-web-fetch",
        legacy_manifest: "crates/vesper-web-fetch/Cargo.toml",
        current: "jarvis-web-fetch",
        planned: "jarvis-web-fetch",
        role: PackageRole::Foundation,
        current_import: Some("jarvis_web_fetch"),
        planned_import: Some("jarvis_web_fetch"),
        manifest: "crates/jarvis-web-fetch/Cargo.toml",
        executables: &[Executable {
            current: "jarvis-web-fetch",
            planned: "jarvis-web-fetch",
            shipping: true,
        }],
    },
    PackageIdentity {
        legacy: "xtask",
        legacy_manifest: "xtask/Cargo.toml",
        current: "xtask",
        planned: "xtask",
        role: PackageRole::Maintenance,
        current_import: None,
        planned_import: None,
        manifest: "xtask/Cargo.toml",
        executables: &[Executable {
            current: "xtask",
            planned: "xtask",
            shipping: false,
        }],
    },
    PackageIdentity {
        legacy: "jarvis-identity",
        legacy_manifest: "crates/jarvis-identity/Cargo.toml",
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
