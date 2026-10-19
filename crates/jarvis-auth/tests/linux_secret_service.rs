//! Opt-in synthetic acceptance on a private D-Bus session, never a desktop bus.
#![cfg(target_os = "linux")]

use jarvis_auth::{CredentialId, CredentialStoreError, SecureCredentialStore, StorageBackend};
use std::{path::PathBuf, process::Command};

fn require_private_bus() {
    assert_eq!(
        std::env::var("NATIVE_SECRET_SERVICE_FIXTURE").as_deref(),
        Ok("unlocked-private-bus")
    );
    let root = PathBuf::from(std::env::var_os("NATIVE_SECRET_SERVICE_FIXTURE_HOME").unwrap());
    assert!(root.join(".native-credential-test-home").is_file());
    let address = std::env::var("DBUS_SESSION_BUS_ADDRESS").unwrap();
    assert_eq!(
        std::env::var("NATIVE_SECRET_SERVICE_FIXTURE_BUS").unwrap(),
        address
    );
    assert!(address.starts_with(&format!("unix:path={}/run/", root.display())));
}

fn fixture_id(account: &str) -> CredentialId {
    assert_eq!(account.len(), 32);
    assert!(account.bytes().all(|byte| byte.is_ascii_hexdigit()));
    CredentialId::new("fixture", Box::leak(account.to_owned().into_boxed_str()))
}

#[test]
#[ignore = "requires the explicit private Secret Service fixture"]
fn linux_native_persistence_namespace_and_restart() {
    require_private_bus();
    let temporary = tempfile::tempdir().unwrap();
    let vault = temporary.path().join("auth").join("credentials.json");
    let store = SecureCredentialStore::new("99percentgrip.jarvis", vault.clone());
    let account = uuid::Uuid::new_v4().simple().to_string();
    let id = fixture_id(&account);
    let foreign_service = format!("vesper-synthetic-{}", uuid::Uuid::new_v4().simple());
    let foreign = dbus_secret_service_keyring_store::Store::new().unwrap();
    let foreign_entry = keyring_core::api::CredentialStoreApi::build(
        foreign.as_ref(),
        &foreign_service,
        &format!("fixture:{account}"),
        None,
    )
    .unwrap();
    foreign_entry
        .set_password("synthetic-other-product-value")
        .unwrap();
    assert!(store.load(id).unwrap().is_none());
    for value in ["synthetic-first-value", "synthetic-replaced-value"] {
        assert_eq!(
            store.store(id, value).unwrap().backend,
            StorageBackend::NativeKeyring
        );
        assert!(!vault.exists());
        assert_eq!(store.load(id).unwrap().unwrap().expose().as_str(), value);
        let child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "linux_native_reload_child",
                "--show-output",
            ])
            .env("NATIVE_CREDENTIAL_FIXTURE_ACCOUNT", &account)
            .env("NATIVE_CREDENTIAL_FIXTURE_VAULT", &vault)
            .env("NATIVE_CREDENTIAL_FIXTURE_PHASE", value)
            .output()
            .unwrap();
        let stdout = String::from_utf8(child.stdout).unwrap();
        assert!(child.status.success());
        assert!(
            stdout
                .lines()
                .any(|line| line.trim() == "test linux_native_reload_child ... ok")
        );
        assert!(stdout.contains("test result: ok. 1 passed; 0 failed; 0 ignored;"));
    }
    store.remove(id).unwrap();
    assert!(store.load(id).unwrap().is_none());
    assert_eq!(
        foreign_entry.get_password().unwrap(),
        "synthetic-other-product-value"
    );
    foreign_entry.delete_credential().unwrap();
    println!(
        "real private GNOME Secret Service: native create/read/replace/restart/delete and namespace isolation verified"
    );
    let connection = dbus::blocking::Connection::new_session().unwrap();
    let proxy = connection.with_proxy(
        "org.freedesktop.secrets",
        "/org/freedesktop/secrets",
        std::time::Duration::from_secs(5),
    );
    let (collection,): (dbus::Path<'static>,) = proxy
        .method_call("org.freedesktop.Secret.Service", "ReadAlias", ("default",))
        .unwrap();
    let (_locked, _prompt): (Vec<dbus::Path<'static>>, dbus::Path<'static>) = proxy
        .method_call(
            "org.freedesktop.Secret.Service",
            "Lock",
            (vec![collection.clone()],),
        )
        .unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "locked_service_never_creates_a_plaintext_vault",
            "--show-output",
        ])
        .env("NATIVE_LOCKED_FIXTURE_COLLECTION", collection.to_string())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    while child.try_wait().unwrap().is_none() {
        if std::time::Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("locked Secret Service operation exceeded the bounded fixture deadline");
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "locked child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("test locked_service_never_creates_a_plaintext_vault ... ok")
    );
    println!("real locked Secret Service: write refused without plaintext fallback");
}

#[test]
#[ignore = "spawned only after locking the private fixture collection"]
fn locked_service_never_creates_a_plaintext_vault() {
    require_private_bus();
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
    let collection = std::env::var("NATIVE_LOCKED_FIXTURE_COLLECTION").unwrap();
    let connection = dbus::blocking::Connection::new_session().unwrap();
    let proxy = connection.with_proxy(
        "org.freedesktop.secrets",
        collection,
        std::time::Duration::from_secs(5),
    );
    let locked: bool = proxy
        .get("org.freedesktop.Secret.Collection", "Locked")
        .unwrap();
    assert!(locked, "fixture must establish the real locked state");
    let temporary = tempfile::tempdir().unwrap();
    let vault = temporary.path().join("auth").join("credentials.json");
    let store = SecureCredentialStore::new("99percentgrip.jarvis", vault.clone());
    assert!(matches!(
        store.store(CredentialId::new("fixture", "locked"), "synthetic-only"),
        Err(CredentialStoreError::Unavailable)
    ));
    assert!(!vault.exists());
}

#[test]
#[ignore = "spawned only by private native persistence acceptance"]
fn linux_native_reload_child() {
    require_private_bus();
    let id = fixture_id(&std::env::var("NATIVE_CREDENTIAL_FIXTURE_ACCOUNT").unwrap());
    let vault = PathBuf::from(std::env::var_os("NATIVE_CREDENTIAL_FIXTURE_VAULT").unwrap());
    let phase = std::env::var("NATIVE_CREDENTIAL_FIXTURE_PHASE").unwrap();
    assert!(matches!(
        phase.as_str(),
        "synthetic-first-value" | "synthetic-replaced-value"
    ));
    let store = SecureCredentialStore::new("99percentgrip.jarvis", vault);
    assert_eq!(store.load(id).unwrap().unwrap().expose().as_str(), phase);
}

#[test]
#[ignore = "requires the explicit private Secret Service fixture"]
fn missing_service_never_creates_a_plaintext_vault() {
    assert_eq!(
        std::env::var("NATIVE_SECRET_SERVICE_FIXTURE").as_deref(),
        Ok("missing-private-bus")
    );
    let address = std::env::var("DBUS_SESSION_BUS_ADDRESS").unwrap();
    assert!(address.starts_with("unix:path="));
    assert!(address.ends_with("/absent-fixture-bus"));
    let temporary = tempfile::tempdir().unwrap();
    let vault = temporary.path().join("auth").join("credentials.json");
    let store = SecureCredentialStore::new("99percentgrip.jarvis", vault.clone());
    let id = CredentialId::new("fixture", "api-key");
    assert!(matches!(
        store.store(id, "synthetic-fixture-key"),
        Err(CredentialStoreError::Unavailable)
    ));
    assert!(
        !vault.exists(),
        "unavailable native storage must not save plaintext"
    );
    assert!(store.load(id).unwrap().is_none());
    store.remove(id).unwrap();
}
