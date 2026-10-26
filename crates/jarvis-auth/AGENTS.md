# Secure credential storage

## Purpose

Own provider-neutral native credential persistence, retained private-vault reads
and explicitly scoped permission-verified file stores. Production Linux writes
require Secret Service rather than automatic plaintext fallback.

## Ownership

- `src/lib.rs` owns credential identifiers, validation, native-store access,
  strict private-file persistence, secret-free Windows cross-process leases,
  and secret-safe errors/receipts. Its read-only production override preflight
  validates explicit vault ownership before either host initializes private roots.
- `src/native_records.rs` owns Windows size-bounded native records, immutable
  generations, readback/integrity, interruption recovery and sign-out tombstones.
- `src/native_store.rs` selects explicit `keyring-core` native stores without
  mutating the process-global default. Linux uses encrypted libdbus Secret Service;
  macOS and Windows retain their existing native backends and versions.
- `tests/linux_secret_service.rs` owns opt-in real private-bus GNOME persistence,
  namespace/restart, locked-service and missing-service/no-plaintext regressions.
- `tests/native_persistence.rs` owns explicit hosted synthetic save/restart/
  rotation/sign-out acceptance through the public production store. Every
  reload requires a named one-case passing child receipt; missing/renamed/ignored
  or zero-match selections fail, including an offline parser regression.

## Local Contracts

- The production application service is `99percentgrip.jarvis`. Before native or fallback access, refuse legacy/redirected paths and existing unmarked credential records. New vault writes carry the application owner; no Vesper token read, refresh, deletion or plaintext export is authorized. Generic path-explicit fixture stores do not establish production adoption authority.

- Depend only on `jarvis-security` among workspace crates.
- Never include credential values in formatting, errors, logs, or serialized
  metadata other than the explicitly authorized private fallback vault.
- Prefer the native OS credential manager. A fallback vault must be created
  atomically with directory mode `0700` and file mode `0600` on Unix.
- Production Linux writes require Secret Service and fail unavailable rather than
  creating a plaintext fallback. Existing explicitly owned retained-vault reads
  keep their precedence; path-explicit fixture stores are not native acceptance.
- Linux links distro-owned libdbus dynamically; build prerequisites are pkg-config
  and libdbus development files. Do not enable vendored libdbus, the retired zbus
  backend, keyutils, sample stores or plaintext replacements through features.
- A successful native write removes that credential's older fallback. A
  retained fallback is authoritative on reads, preventing an older keyring
  value from resurfacing after a newer fallback rotation or logout.
- Fail closed instead of creating a permission-unverified fallback on Windows.
- Ordinary tests use path-explicit private stores or the Windows-sized in-memory
  backend and never access user OS keyrings. ADR 0032 permits explicitly opted-in
  hosted acceptance using only UUID namespaced synthetic entries, fresh-process
  reload and cleanup on disposable GitHub runners.
- Windows records exceed the native blob limit only through bounded immutable
  chunks, verified primary descriptors and a native transaction journal. Never
  add a plaintext Windows fallback. The secret-free file lease is bounded and
  every process for one native identity must use the same configured vault path.
- Existing explicit vault reads keep retained-value precedence. Windows must
  refuse a write/delete before native mutation when a retained vault credential
  cannot be safely retired; startup and private signed-out fixtures remain read-only.
- Composite records are bounded to 256 KiB; serialized Unix vaults to 1 MiB.
  Oversized writes must preserve the previous readable vault.

## Work Guidance

- Keep provider identity data-driven; this crate does not register providers.
- Preserve bounded inputs and atomic replacement for every fallback write.

## Verification

- Run `cargo test -p jarvis-auth --all-features`.
- Run `.github/test_credential_backend.py` for actual all-feature dependency graph
  coverage. Real Linux credential acceptance uses only the private bus/home helper;
  KDE interoperability is not established by a GNOME result or API documentation.
- Run `cargo xtask architecture` and strict workspace Clippy.

## Child DOX Index

No children.
