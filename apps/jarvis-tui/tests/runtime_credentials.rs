//! Exercise the exact terminal fixture through the production ownership reader.
use std::{path::Path, process::Command};

#[test]
fn signed_out_terminal_vault_has_readable_private_ownership() {
    let root = tempfile::tempdir().unwrap();
    let vault = root.path().join("config/signed-out.json");
    // The public credential API takes registered static identities. This single
    // bounded fixture never shares an account with another process or user.
    let provider: &'static str =
        Box::leak(format!("fixture-{}", uuid::Uuid::new_v4().simple()).into_boxed_str());
    let helpers = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let output = Command::new(if cfg!(windows) { "python" } else { "python3" })
        .args([
            "-B",
            "-c",
            "import sys; from pathlib import Path; sys.path.insert(0, sys.argv[1]); from settings_auth_pty import signed_out_vault; signed_out_vault(Path(sys.argv[2]), sys.argv[3], 'native-auth')",
        ])
        .arg(helpers)
        .arg(&vault)
        .arg(provider)
        .output()
        .expect("run bounded synthetic fixture setup");
    assert!(
        output.status.success(),
        "synthetic fixture setup failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    jarvis_security::validate_owned_directory(vault.parent().unwrap())
        .expect("terminal fixture directory ownership must validate");
    jarvis_security::validate_private_file(&vault)
        .expect("terminal fixture vault confidentiality must validate");
    let id = jarvis_auth::CredentialId::new(provider, "native-auth");
    let direct = jarvis_auth::PrivateFileCredentialStore::new(vault.clone())
        .load(id)
        .unwrap()
        .expect("matching synthetic fallback must exist before native-first read");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(direct.expose().as_str()).unwrap()["mode"],
        "signed-out"
    );
    let store = jarvis_auth::SecureCredentialStore::new("99percentgrip.jarvis", vault.clone());
    let before = std::fs::read(&vault).unwrap();
    assert_eq!(
        store.load(id).unwrap().unwrap().expose().as_str(),
        direct.expose().as_str()
    );
    assert_eq!(std::fs::read(vault).unwrap(), before);
}
