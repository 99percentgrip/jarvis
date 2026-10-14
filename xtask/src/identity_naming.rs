//! Narrow first-party declarations do not exempt historical upstream branding.
const OWNED_PATHS: &[&str] = &[
    "AGENTS.md",
    "crates/AGENTS.md",
    "docs/AGENTS.md",
    "docs/foundation/AGENTS.md",
    "docs/architecture/AGENTS.md",
    "docs/architecture/identity-architecture.md",
    "docs/identity-stage1-prd.md",
    "docs/foundation/2026-10-14-identity-stage1.md",
];

pub fn owned_line(path: &str, line: &str) -> bool {
    if !path.starts_with("crates/jarvis-identity/") && !OWNED_PATHS.contains(&path) {
        return false;
    }
    let mut identifiers = vec![
        "jarvis",
        ".jarvis",
        ".jarvis-scope-id",
        "jarvis-checkpoint",
        "jarvis.release-recovery.v1",
        "99percentgrip.jarvis",
        "99percentgrip/jarvis",
        "Jarvis",
        "Jarvis/Data",
        "Jarvis/Cache",
        "Jarvis/State",
        "Jarvis/Logs",
        "Programs/Jarvis",
        "Library/Application Support/Jarvis",
        "Library/Caches/Jarvis",
        "Library/Logs/Jarvis",
        "jarvis/logs",
        "jarvis/install",
        "jarvis-identity/AGENTS.md",
        "crates/jarvis-identity/AGENTS.md",
        "crates/jarvis-identity",
        "jarvis-identity",
        "crates/jarvis-identity/Cargo.toml",
        "crates/jarvis-identity/src/lib.rs",
        "crates/jarvis-identity/src/packages.rs",
        "crates/jarvis-identity/src/mappings.rs",
        "crates/jarvis-identity/src/platform.rs",
        "crates/jarvis-identity/tests/contracts.rs",
    ];
    for p in jarvis_identity::PACKAGES {
        identifiers.push(p.planned);
        if let Some(import) = p.planned_import {
            identifiers.push(import);
        }
        for b in p.executables {
            identifiers.push(b.planned);
        }
    }
    for m in jarvis_identity::MAPPINGS {
        if let Some(value) = m.planned {
            identifiers.push(value);
        }
    }
    for d in jarvis_identity::DISTRIBUTIONS {
        identifiers.push(d.archive);
    }
    // Identifiers containing the reserved stem must be explicit quoted/backtick
    // declarations. Bare upstream prose or unknown names still hit the embargo.
    let mut scrubbed = line.replace("use jarvis_identity::", "use owned_identity::");
    for id in identifiers {
        for quote in ['"', '`'] {
            scrubbed = scrubbed.replace(&format!("{quote}{id}{quote}"), "owned_identity");
            let escaped = id.replace('\\', "\\\\");
            scrubbed = scrubbed.replace(&format!("{quote}{escaped}{quote}"), "owned_identity");
        }
    }
    !super::contains_word_bounded(&scrubbed.to_lowercase(), "jarvis")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn declarations_are_scoped_and_other_upstream_mentions_stay_frozen() {
        assert!(owned_line(
            "crates/jarvis-identity/src/lib.rs",
            "slug: \"jarvis\""
        ));
        assert!(owned_line(
            "docs/architecture/identity-architecture.md",
            "`jarvis-identity`"
        ));
        assert!(!owned_line("docs/historical.md", "`jarvis-identity`"));
        assert!(!owned_line("crates/vesper-domain/src/lib.rs", "\"jarvis\""));
        assert!(!owned_line(
            "crates/jarvis-identity/src/lib.rs",
            "jarvis upstream prose"
        ));
        assert!(!owned_line(
            "docs/architecture/identity-architecture.md",
            "`jarvis-unreviewed`"
        ));
    }
}
