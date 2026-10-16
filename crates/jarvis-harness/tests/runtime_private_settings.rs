//! Actual saved formats must not activate from an unowned application store.
use std::{collections::BTreeMap, path::Path};

use jarvis_harness::{
    acceptance_settings::AcceptanceSettings, skill_routing_settings, web_settings,
};

fn records(root: &Path) -> BTreeMap<String, Vec<u8>> {
    std::fs::read_dir(root.join(".jarvis"))
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (
                entry.file_name().to_str().unwrap().to_owned(),
                std::fs::read(entry.path()).unwrap(),
            )
        })
        .collect()
}

#[test]
fn unowned_project_settings_refuse_without_import_or_mutation() {
    let source = tempfile::tempdir().unwrap();
    AcceptanceSettings {
        enabled: true,
        prd: String::new(),
    }
    .save(source.path())
    .unwrap();
    skill_routing_settings::save(source.path(), &Default::default()).unwrap();
    web_settings::save(source.path(), &Default::default()).unwrap();
    #[cfg(feature = "bridge")]
    jarvis_harness::bridge_settings::BridgeSettings { enabled: true }
        .save(source.path())
        .unwrap();
    #[cfg(feature = "swarm")]
    jarvis_harness::swarm_settings::save(source.path(), &Default::default()).unwrap();
    jarvis_security::write_private_atomic(
        &source.path().join(".jarvis/config.toml"),
        b"[web]\nenabled = true\n",
    )
    .unwrap();

    // Establish that the generated records are valid before withholding ownership.
    assert!(AcceptanceSettings::load(source.path()).unwrap().enabled);
    assert!(skill_routing_settings::load(source.path()).is_ok());
    assert!(web_settings::load(source.path()).is_ok());
    assert!(jarvis_config::read_sandbox_scope(source.path()).is_ok());
    #[cfg(feature = "bridge")]
    assert!(jarvis_harness::bridge_settings::BridgeSettings::load(source.path()).enabled);
    #[cfg(feature = "swarm")]
    assert!(jarvis_harness::swarm_settings::load(source.path()).is_ok());

    let foreign = tempfile::tempdir().unwrap();
    std::fs::create_dir(foreign.path().join(".jarvis")).unwrap();
    for (name, bytes) in records(source.path()) {
        if name != ".jarvis-owner" {
            std::fs::write(foreign.path().join(".jarvis").join(name), bytes).unwrap();
        }
    }
    let before = records(foreign.path());
    assert!(AcceptanceSettings::load(foreign.path()).is_err());
    assert!(skill_routing_settings::load(foreign.path()).is_err());
    assert!(web_settings::load(foreign.path()).is_err());
    assert!(jarvis_config::read_sandbox_scope(foreign.path()).is_err());
    #[cfg(feature = "bridge")]
    assert!(!jarvis_harness::bridge_settings::BridgeSettings::load(foreign.path()).enabled);
    #[cfg(feature = "swarm")]
    assert!(jarvis_harness::swarm_settings::load(foreign.path()).is_err());
    assert!(AcceptanceSettings::default().save(foreign.path()).is_err());
    assert_eq!(records(foreign.path()), before);
    assert!(!foreign.path().join(".jarvis/.jarvis-owner").exists());
}
