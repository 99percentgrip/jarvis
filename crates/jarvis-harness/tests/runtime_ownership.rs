//! Ordinary production-library paths, including all-features builds, stay isolated.
use jarvis_harness::{MemoryStores, release_recovery};

#[test]
fn ordinary_release_commands_are_dormant_without_creating_ledger_state() {
    assert!(release_recovery::default_release_root().is_none());
    for command in ["status", "start v0.1.0", "resume", "recover", "cancel"] {
        assert!(
            release_recovery::release_command(
                "99percentgrip/jarvis",
                command,
                "fixture-only-unverified-source",
                "main"
            )
            .is_err()
        );
    }
}

#[test]
fn private_memory_does_not_adopt_vesper_and_does_not_restrict_user_project_names() {
    let temporary = tempfile::tempdir().unwrap();
    let project = temporary.path().join("agent-vesper");
    let legacy = project.join(".agent-vesper/memory");
    std::fs::create_dir_all(&legacy).unwrap();
    let canary = legacy.join("profile.md");
    std::fs::write(&canary, "synthetic original private profile").unwrap();
    std::fs::write(
        project.join("README.md"),
        "explicitly selected user project",
    )
    .unwrap();
    let refused = MemoryStores::open_at(&legacy, &legacy);
    assert!(refused.memory().is_none());
    let current = project.join(".jarvis/memory");
    let global = temporary.path().join("independent-data/memory");
    let stores = MemoryStores::open_at(&current, &global);
    assert!(stores.memory().is_some());
    let profile = stores.profile().unwrap();
    assert!(!profile.read().contains("synthetic original"));
    profile
        .append("Preferences", "independent profile")
        .unwrap();
    assert!(profile.read().contains("independent profile"));
    assert_eq!(
        std::fs::read_to_string(&canary).unwrap(),
        "synthetic original private profile"
    );
    assert_eq!(
        std::fs::read_to_string(project.join("README.md")).unwrap(),
        "explicitly selected user project"
    );
}
