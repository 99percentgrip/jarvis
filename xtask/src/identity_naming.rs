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
    "docs/foundation/2026-10-15-rust-identity-stage2.md",
    "docs/foundation/2026-10-15-rust-identity-stage2-source.json",
    "docs/rust-identity-stage2-prd.md",
    "docs/architecture/rust-identity-stage2.md",
    "docs/workspace-map.md",
    "docs/dependencies.md",
    "docs/architecture.md",
];

pub fn owned_line(path: &str, line: &str) -> bool {
    let active = jarvis_identity::PACKAGES
        .iter()
        .any(|p| path.starts_with(p.manifest.trim_end_matches("Cargo.toml")));
    if !active && !OWNED_PATHS.contains(&path) {
        return false;
    }
    let declarations = path.starts_with("crates/jarvis-identity/") || OWNED_PATHS.contains(&path);
    let mut identifiers = if declarations {
        vec![
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
            "jarvis-xtask", // preserved Stage 1 proposal, not an active target
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
        ]
    } else {
        Vec::new()
    };
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
        if declarations && let Some(value) = m.planned {
            identifiers.push(value);
        }
    }
    for d in jarvis_identity::DISTRIBUTIONS {
        if declarations {
            identifiers.push(d.archive);
        }
    }
    // Identifiers containing the reserved stem must be explicit quoted/backtick
    // declarations. Bare upstream prose or unknown names still hit the embargo.
    let mut scrubbed = line.replace("use jarvis_identity::", "use owned_identity::");
    for id in identifiers {
        if id.starts_with("jarvis-") || id.starts_with("jarvis_") {
            scrubbed = scrub_technical_token(&scrubbed, id);
        }
        for quote in ['"', '`'] {
            scrubbed = scrubbed.replace(&format!("{quote}{id}{quote}"), "owned_identity");
            let escaped = id.replace('\\', "\\\\");
            scrubbed = scrubbed.replace(&format!("{quote}{escaped}{quote}"), "owned_identity");
        }
    }
    !super::contains_word_bounded(&scrubbed.to_lowercase(), "jarvis")
}

fn scrub_technical_token(line: &str, id: &str) -> String {
    let mut result = String::new();
    let mut start = 0;
    for (index, _) in line.match_indices(id) {
        let end = index + id.len();
        let identifier = |c: char| c.is_alphanumeric() || c == '_' || c == '-';
        if line[..index].chars().next_back().is_some_and(identifier)
            || line[end..].chars().next().is_some_and(identifier)
        {
            continue;
        }
        result.push_str(&line[start..index]);
        result.push_str("owned_identity");
        start = end;
    }
    result.push_str(&line[start..]);
    result
}

/// Resolve only approved directory/token moves to immutable historical hit bytes.
/// New prose, unknown identifiers and duplicate counts remain subject to the ratchet.
pub fn historical_hit(path: &str, line: &str) -> (String, String) {
    let mut path = path.to_owned();
    let mut line = line.to_owned();
    for package in jarvis_identity::PACKAGES {
        let current = package.manifest.trim_end_matches("Cargo.toml");
        let legacy = package.legacy_manifest.trim_end_matches("Cargo.toml");
        if let Some(rest) = path.strip_prefix(current) {
            path = format!("{legacy}{rest}");
        }
        if package.current != package.legacy {
            // Technical token boundaries, never arbitrary upstream prose.
            line = line.replace(
                &format!("`{}`", package.current),
                &format!("`{}`", package.legacy),
            );
        }
    }
    (path, line)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_registered_directory_moves_resolve_to_historical_hits() {
        assert_eq!(
            historical_hit(
                "crates/jarvis-domain/src/tool.rs",
                "legacy upstream mention"
            ),
            (
                "crates/vesper-domain/src/tool.rs".into(),
                "legacy upstream mention".into()
            )
        );
        assert_eq!(
            historical_hit("crates/unregistered/src/tool.rs", "new upstream mention"),
            (
                "crates/unregistered/src/tool.rs".into(),
                "new upstream mention".into()
            )
        );
    }
    #[test]
    fn declarations_are_scoped_and_other_upstream_mentions_stay_frozen() {
        assert!(owned_line(
            "crates/jarvis-domain/src/lib.rs",
            "use jarvis_domain::Value;"
        ));
        assert!(!owned_line(
            "crates/jarvis-domain/src/lib.rs",
            "jarvis upstream prose"
        ));
        assert!(!owned_line(
            "crates/jarvis-domain/src/lib.rs",
            "jarvis-domain-unreviewed"
        ));
        assert!(owned_line(
            "crates/jarvis-identity/src/lib.rs",
            "slug: \"jarvis\""
        ));
        assert!(owned_line(
            "docs/architecture/identity-architecture.md",
            "`jarvis-identity`"
        ));
        assert!(!owned_line("docs/historical.md", "`jarvis-identity`"));
        assert!(!owned_line("crates/jarvis-domain/src/lib.rs", "\"jarvis\""));
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
