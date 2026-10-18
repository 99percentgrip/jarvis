//! Explicit native backends; no global default-store mutation or fallback selection.

use std::sync::{Arc, LazyLock};

use keyring_core::{CredentialStore, Entry, Error, Result};

static STORE: LazyLock<Result<Arc<CredentialStore>>> = LazyLock::new(create_store);

pub(crate) fn entry(service: &str, account: &str) -> Result<Entry> {
    STORE
        .as_ref()
        .map_err(|_| Error::NoDefaultStore)?
        .build(service, account, None)
}

fn create_store() -> Result<Arc<CredentialStore>> {
    #[cfg(target_os = "macos")]
    let store = apple_native_keyring_store::keychain::Store::new()?;
    #[cfg(windows)]
    let store = windows_native_keyring_store::Store::new()?;
    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "ios", target_os = "android"))
    ))]
    let store = dbus_secret_service_keyring_store::Store::new()?;
    #[cfg(all(any(unix, windows), not(any(target_os = "ios", target_os = "android"))))]
    {
        Ok(store)
    }
    #[cfg(not(all(any(unix, windows), not(any(target_os = "ios", target_os = "android")))))]
    Err(Error::NoDefaultStore)
}
