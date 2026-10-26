//! Ordinary ACP startup must preflight explicit credential ownership without imports.
use std::process::Command;

fn launch(fixture: &tempfile::TempDir, name: &str, path: &std::path::Path) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jarvis-acp"));
    command
        .env_clear()
        .current_dir(fixture.path())
        .env("HOME", fixture.path())
        .env("USERPROFILE", fixture.path())
        .env("APPDATA", fixture.path().join("roaming"))
        .env("LOCALAPPDATA", fixture.path().join("local"))
        .env("XDG_CONFIG_HOME", fixture.path().join("config"))
        .env(name, path);
    for key in ["SystemRoot", "WINDIR", "COMSPEC", "PATH"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command.output().unwrap()
}

fn write_private(path: &std::path::Path, bytes: &[u8]) {
    jarvis_security::write_private_atomic(path, bytes).unwrap();
}

#[test]
fn foreign_credential_overrides_are_refused_before_private_root_initialization() {
    for name in [
        "JARVIS_CREDENTIALS_PATH",
        "JARVIS_OPENAI_CREDENTIALS_PATH",
        "JARVIS_XAI_CREDENTIALS_PATH",
        "JARVIS_LMSTUDIO_CREDENTIALS_PATH",
    ] {
        let fixture = tempfile::tempdir().unwrap();
        let foreign = fixture.path().join("foreign-credentials.json");
        let canary = br#"{"application":"99percentgrip.agent-vesper","credentials":{"openai":{"native-auth":"SYNTHETIC-FOREIGN-CREDENTIAL-DO-NOT-IMPORT"}}}"#;
        write_private(&foreign, canary);
        let output = launch(&fixture, name, &foreign);
        assert!(
            !output.status.success(),
            "{name} accepted a foreign credential path"
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("ownership refused"));
        assert_eq!(std::fs::read(&foreign).unwrap(), canary);
        assert!(
            !fixture.path().join("config").exists(),
            "{name} created private roots despite refusal"
        );
    }
}

#[test]
fn malformed_private_blocker_stays_inert_and_is_not_adopted() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("invalid.json");
    let blocker = br#"{"credentials":null}"#;
    write_private(&path, blocker);
    let output = launch(&fixture, "JARVIS_CREDENTIALS_PATH", &path);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(std::fs::read(&path).unwrap(), blocker);
    let store = jarvis_auth::SecureCredentialStore::new("99percentgrip.jarvis", path);
    assert!(matches!(
        store.load(jarvis_auth::CredentialId::new("zai", "api-key")),
        Err(jarvis_auth::CredentialStoreError::Serialize)
    ));
}

#[test]
fn owned_credential_override_allows_clean_startup_without_rewriting_vault() {
    let fixture = tempfile::tempdir().unwrap();
    let path = fixture.path().join("credentials.json");
    let canary = br#"{"application":"99percentgrip.jarvis","credentials":{}}"#;
    write_private(&path, canary);
    let output = launch(&fixture, "JARVIS_OPENAI_CREDENTIALS_PATH", &path);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(std::fs::read(path).unwrap(), canary);
}
