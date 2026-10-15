use jarvis_identity::*;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::Command,
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn actual_workspace() -> Vec<WorkspacePackage> {
    let root = root();
    let out = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
            "--locked",
        ])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let data: Value = serde_json::from_slice(&out.stdout).unwrap();
    let members: BTreeSet<_> = data["workspace_members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    data["packages"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| members.contains(p["id"].as_str().unwrap()))
        .map(|p| WorkspacePackage {
            name: p["name"].as_str().unwrap().to_owned(),
            manifest: Path::new(p["manifest_path"].as_str().unwrap())
                .canonicalize()
                .unwrap()
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/"),
            library: p["targets"]
                .as_array()
                .unwrap()
                .iter()
                .find(|t| t["kind"].as_array().unwrap().iter().any(|k| k == "lib"))
                .map(|t| t["name"].as_str().unwrap().to_owned()),
            binaries: p["targets"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|t| t["kind"].as_array().unwrap().iter().any(|k| k == "bin"))
                .map(|t| t["name"].as_str().unwrap().to_owned())
                .collect(),
            custom_build: p["targets"].as_array().unwrap().iter().any(|t| {
                t["kind"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|k| k == "custom-build")
            }),
            normal_dependencies: p["dependencies"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["kind"].as_str() != Some("dev"))
                .map(|d| d["name"].as_str().unwrap().to_owned())
                .collect(),
        })
        .collect()
}

#[test]
fn actual_cargo_packages_and_executables_are_registered() {
    validate_workspace(&actual_workspace()).unwrap();
    let root = root();
    let mut manifests = BTreeSet::from(["xtask/Cargo.toml".to_owned()]);
    for directory in ["crates", "apps", "spikes"] {
        for entry in std::fs::read_dir(root.join(directory)).unwrap() {
            let entry = entry.unwrap();
            if entry.path().join("Cargo.toml").is_file() {
                manifests.insert(format!(
                    "{directory}/{}/Cargo.toml",
                    entry.file_name().to_string_lossy()
                ));
            }
        }
    }
    let declared: BTreeSet<_> = PACKAGES
        .iter()
        .map(|p| p.manifest.to_owned())
        .chain(EXCLUDED_MANIFESTS.iter().map(|p| p.0.to_owned()))
        .collect();
    assert_eq!(manifests, declared, "unregistered first-party manifest");
    for (manifest, name) in EXCLUDED_MANIFESTS {
        let text = std::fs::read_to_string(root.join(manifest)).unwrap();
        assert!(text.contains(&format!("name = \"{name}\"")));
    }
}

#[test]
fn newly_added_packages_binaries_or_runtime_dependencies_fail_verification() {
    let actual = actual_workspace();
    let mut changed = actual.clone();
    changed.push(WorkspacePackage {
        name: "unregistered-package".into(),
        manifest: "crates/unregistered/Cargo.toml".into(),
        library: None,
        binaries: vec![],
        custom_build: false,
        normal_dependencies: vec![],
    });
    assert!(validate_workspace(&changed).is_err());
    let mut changed = actual.clone();
    changed[0].binaries.push("unregistered-binary".into());
    assert!(validate_workspace(&changed).is_err());
    let mut changed = actual.clone();
    changed[0].manifest = "other/Cargo.toml".into();
    assert!(validate_workspace(&changed).is_err());
    let mut changed = actual.clone();
    changed[0].library = Some("unregistered_library".into());
    assert!(validate_workspace(&changed).is_err());
    let mut changed = actual.clone();
    changed
        .iter_mut()
        .find(|p| p.name == "jarvis-identity")
        .unwrap()
        .custom_build = true;
    assert!(validate_workspace(&changed).is_err());
    for target in ["jarvis-identity", "jarvis-domain"] {
        let mut changed = actual.clone();
        changed
            .iter_mut()
            .find(|p| p.name == target)
            .unwrap()
            .normal_dependencies
            .push("jarvis-identity".into());
        assert!(validate_workspace(&changed).is_err());
    }
}

#[test]
fn package_executable_and_import_targets_are_collision_free() {
    let mut packages = BTreeSet::new();
    let mut binaries = BTreeSet::new();
    let mut imports = BTreeSet::new();
    for p in PACKAGES {
        assert!(
            p.planned
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        );
        assert!(packages.insert(p.planned));
        if let Some(import) = p.planned_import {
            assert!(imports.insert(import));
        }
        for b in p.executables {
            assert!(
                b.planned
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            );
            assert!(binaries.insert(b.planned));
        }
    }
    assert!(binaries.contains("jarvis-tui"));
    assert!(binaries.contains("jarvis-acp"));
}

#[test]
fn declared_legacy_contracts_have_current_source_evidence() {
    let root = root();
    for m in MAPPINGS {
        let source = std::fs::read_to_string(root.join(m.source)).unwrap();
        let active_private_mapping = m.classification == Classification::CompatibilitySensitive
            && matches!(
                m.surface,
                Surface::Configuration | Surface::Credential | Surface::ProjectScope
            )
            && m.planned.is_some_and(|target| source.contains(target));
        let active_environment_mapping = m.surface == Surface::Environment
            && RUNTIME_ENVIRONMENT
                .iter()
                .any(|entry| entry.legacy == m.legacy && source.contains(entry.current));
        assert!(
            source.contains(m.legacy) || active_private_mapping || active_environment_mapping,
            "missing legacy or activated private-contract source evidence: {m:?}"
        );
    }
    for p in PACKAGES {
        assert!(
            std::fs::read_to_string(root.join(p.manifest))
                .unwrap()
                .contains(&format!("name = \"{}\"", p.current))
        );
    }
}

#[test]
fn env_targets_are_unique_and_classified_without_automatic_fallback() {
    let mut seen = BTreeSet::new();
    for m in MAPPINGS
        .iter()
        .filter(|m| m.surface == Surface::Environment)
    {
        assert!(matches!(
            m.classification,
            Classification::CompatibilitySensitive | Classification::RequiresEvidence
        ));
        if let Some(target) = m.planned {
            assert!(seen.insert(target), "duplicate target {target}");
        }
    }
}

#[test]
fn sensitive_data_is_not_mechanically_renamed_and_preserved_names_have_no_target() {
    for m in MAPPINGS {
        if matches!(
            m.surface,
            Surface::Credential | Surface::Serialization | Surface::ProjectScope
        ) {
            assert!(matches!(
                m.classification,
                Classification::CompatibilitySensitive
                    | Classification::LegacyReader
                    | Classification::RequiresEvidence
            ));
        }
        if matches!(
            m.classification,
            Classification::ThirdParty | Classification::Provenance | Classification::LegacyReader
        ) {
            assert!(m.planned.is_none());
        }
    }
    const {
        assert!(IMPORT_POLICY.explicit_authorization_required);
        assert!(!IMPORT_POLICY.automatic_credential_sharing && !IMPORT_POLICY.source_mutation);
        assert!(
            !IMPORT_POLICY.interrupted_operation_replay && !IMPORT_POLICY.release_ledger_import
        );
        assert!(IMPORT_POLICY.preserve_message_and_session_ids);
    };
}

#[test]
fn acp_repository_provenance_and_release_ownership_agree_with_real_source() {
    let root = root();
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(root.join("registry/agent.json")).unwrap()).unwrap();
    let registry = MAPPINGS
        .iter()
        .find(|m| m.surface == Surface::Acp && m.source == "registry/agent.json")
        .unwrap();
    assert_eq!(manifest["id"], registry.legacy);
    assert_eq!(registry.planned, Some(PRODUCT.acp_id));
    assert_eq!(
        MAPPINGS
            .iter()
            .find(|m| m.surface == Surface::Credential && m.legacy == "agent-vesper")
            .unwrap()
            .planned,
        Some(PRODUCT.credential_service)
    );
    assert_eq!(
        MAPPINGS
            .iter()
            .find(|m| m.legacy == "agent-vesper-session")
            .unwrap()
            .planned,
        Some(PRODUCT.session_format)
    );
    assert_eq!(
        MAPPINGS
            .iter()
            .find(|m| m.legacy == "vesper-release-controller")
            .unwrap()
            .planned,
        Some(PRODUCT.controller_name)
    );
    let targets: BTreeSet<_> = manifest["distribution"]["binary"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(targets, DISTRIBUTIONS.iter().map(|d| d.target).collect());
    assert_eq!(PRODUCT.acp_id, PRODUCT.slug);
    assert_eq!(PRODUCT.repository.name, PRODUCT.slug);
    assert_eq!(
        PRODUCT.release_owner,
        format!("{}/{}", PRODUCT.repository.owner, PRODUCT.repository.name)
    );
    assert_ne!(
        PRODUCT.release_owner,
        format!("{}/{}", LEGACY_REPOSITORY.owner, LEGACY_REPOSITORY.name)
    );
    assert_eq!(INSTALLER_REPOSITORY, PRODUCT.release_owner);
    let evidence: Value = serde_json::from_slice(
        &std::fs::read(root.join("docs/foundation/stage1-source-provenance.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(evidence["source_commit"], SOURCE_COMMIT);
    assert_eq!(evidence["tag_object"], SOURCE_TAG_OBJECT);
    assert_eq!(evidence["source_ref"], SOURCE_REFERENCE);
    assert_eq!(manifest["version"], SOURCE_RELEASE.trim_start_matches('v'));
    assert_eq!(SOURCE_RELEASE, "v0.24.14");
}

#[test]
fn all_five_planned_platform_roots_are_disjoint_from_actual_legacy_defaults() {
    use jarvis_config::{JarvisPaths, PathEnvironment, Platform as LegacyPlatform, VesperPaths};
    let inputs = PathInputs {
        home: "/identity-fixture/home".into(),
        config_base: "/identity-fixture/home/.config".into(),
        data_base: "/identity-fixture/home/.local/share".into(),
        cache_base: "/identity-fixture/home/.cache".into(),
        state_base: "/identity-fixture/home/.local/state".into(),
        roaming: "/identity-fixture/roaming".into(),
        local: "/identity-fixture/local".into(),
    };
    let env = PathEnvironment {
        home: Some(inputs.home.clone()),
        xdg_config_home: Some(inputs.config_base.clone()),
        xdg_data_home: Some(inputs.data_base.clone()),
        xdg_cache_home: Some(inputs.cache_base.clone()),
        xdg_state_home: Some(inputs.state_base.clone()),
        app_data: Some(inputs.roaming.clone()),
        local_app_data: Some(inputs.local.clone()),
    };
    let mut archives = BTreeSet::new();
    for d in DISTRIBUTIONS {
        assert!(archives.insert(d.archive));
        let old_platform = match d.platform {
            Platform::LinuxX86 | Platform::LinuxArm => LegacyPlatform::Linux,
            Platform::MacIntel | Platform::MacArm => LegacyPlatform::MacOs,
            Platform::WindowsX86 => LegacyPlatform::Windows,
        };
        let old = VesperPaths::resolve(old_platform, &env).unwrap();
        let new = PlannedPaths::for_platform(d.platform, &inputs);
        let active = JarvisPaths::resolve(old_platform, &env).unwrap();
        assert_eq!(active.config, new.config);
        assert_eq!(active.data, new.data);
        assert_eq!(active.cache, new.cache);
        assert_eq!(active.state, new.state);
        assert_eq!(active.logs, new.logs);
        for planned in [
            &new.config,
            &new.data,
            &new.cache,
            &new.state,
            &new.logs,
            &new.install,
            &new.release_ledger(),
        ] {
            for legacy in [&old.config, &old.data, &old.cache, &old.state, &old.logs] {
                assert!(
                    !planned.starts_with(legacy) && !legacy.starts_with(planned),
                    "overlapping roots"
                );
                assert_ne!(
                    planned.to_string_lossy().to_lowercase(),
                    legacy.to_string_lossy().to_lowercase()
                );
            }
        }
        assert_ne!(new.project_directory, ".agent-vesper");
        assert_ne!(new.scope_stamp, ".vesper-scope-id");
    }
    assert_eq!(DISTRIBUTIONS.len(), 5);
}

#[test]
fn active_source_and_shipped_skill_environment_inputs_are_all_classified() {
    fn collect(directory: &Path, found: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if entry.file_name() != "jarvis-identity" {
                    collect(&path, found);
                }
                continue;
            }
            let extension = path.extension().and_then(|v| v.to_str()).unwrap_or("");
            if ![
                "rs", "md", "sh", "ps1", "py", "js", "mjs", "json", "toml", "yaml", "yml", "bat",
                "cmd", "txt",
            ]
            .contains(&extension)
            {
                continue;
            }
            let source = std::fs::read_to_string(path).unwrap();
            for word in source.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
                let branded = ["AGENT_VESPER_", "VESPER_", "JARVIS_"]
                    .iter()
                    .any(|prefix| {
                        word.strip_prefix(prefix).is_some_and(|suffix| {
                            suffix.starts_with(|c: char| c.is_ascii_uppercase())
                        })
                    });
                if word == "GLM_VENV_PATH"
                    || (branded
                        && word
                            .bytes()
                            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'))
                {
                    found.insert(word.to_owned());
                }
            }
        }
    }
    let mut source = BTreeSet::new();
    for directory in ["apps", "crates", "scripts", "skills"] {
        collect(&root().join(directory), &mut source);
    }
    let mut registered: BTreeSet<_> = MAPPINGS
        .iter()
        .filter(|m| m.surface == Surface::Environment)
        .map(|m| m.legacy.to_owned())
        .collect();
    registered.extend(
        RUNTIME_ENVIRONMENT
            .iter()
            .map(|entry| entry.current.to_owned()),
    );
    // Historical mappings remain provenance even after an active reader moves.
    // Every source identifier, including newly added inputs, still needs a row.
    assert!(
        source.is_subset(&registered),
        "unclassified source environment identifiers: {:?}",
        source.difference(&registered).collect::<Vec<_>>()
    );
}

#[test]
fn active_runtime_environment_registry_cites_real_consumers_without_collisions() {
    let root = root();
    let mut targets = std::collections::BTreeSet::new();
    for entry in RUNTIME_ENVIRONMENT {
        assert!(
            entry
                .current
                .to_ascii_lowercase()
                .starts_with(&format!("{}_", PRODUCT.slug))
        );
        assert!(
            targets.insert(entry.current),
            "duplicate runtime environment target"
        );
        let source = std::fs::read_to_string(root.join(entry.source)).unwrap();
        assert!(
            source.contains(entry.current),
            "stale runtime source evidence: {}",
            entry.source
        );
        assert!(
            ![
                "VESPER_SESSION_FORMAT",
                "VESPER_SESSION_VERSION",
                "VESPER_SECRET_CANARY",
                "VESPER_SYNTHETIC_CANARY"
            ]
            .contains(&entry.legacy)
        );
    }
}
