//! Synthetic owned-vault corruption must not resolve through the native manager.
use jarvis_auth::{CredentialId, CredentialStoreError, SecureCredentialStore};

#[test]
fn malformed_owned_record_fails_closed_without_changing_other_provider() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("credentials.json");
    // UUID-scoped accounts prevent even the red-first test from reading real
    // provider entries in a developer's native credential manager.
    let account: &'static str =
        Box::leak(uuid::Uuid::new_v4().simple().to_string().into_boxed_str());
    let bad = CredentialId::new("openai", account);
    let neighbor = CredentialId::new("xai", account);
    let bytes = serde_json::to_vec(&serde_json::json!({
        "application": "99percentgrip.jarvis",
        "credentials": {
            "openai": {account: "bad\nrecord"},
            "xai": {account: "synthetic-good-record"}
        }
    }))
    .unwrap();
    jarvis_security::write_private_atomic(&path, &bytes).unwrap();
    let store = SecureCredentialStore::new("99percentgrip.jarvis", path.clone());
    assert!(matches!(
        store.load(bad),
        Err(CredentialStoreError::InvalidSecret)
    ));
    assert!(matches!(
        store.store(bad, "synthetic-new-record"),
        Err(CredentialStoreError::InvalidSecret)
    ));
    assert!(matches!(
        store.remove(bad),
        Err(CredentialStoreError::InvalidSecret)
    ));
    assert_eq!(
        store.load(neighbor).unwrap().unwrap().expose().as_str(),
        "synthetic-good-record"
    );
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}
