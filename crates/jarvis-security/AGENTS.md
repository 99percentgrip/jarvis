# Security primitives

## Purpose

Own secret-safe values, redaction, environment scrubbing, bounded untrusted
output, path authority descriptors, and honest sandbox capability contracts.

## Local Contracts

- `private_state.rs` owns bounded private-path validation and independent empty-directory initialization. These functions are for application stores, never a blanket constraint on user tool/workspace paths. Refuse traversal, redirected/reparse components, known Vesper-private roots and unmarked nonempty stores; new ownership markers publish atomically. Safe effective-UID checks use the already locked rustix version; temporary staging uses the already locked tempfile version.
  Persistent child initialization establishes/refuses every enclosing `.jarvis`
  application boundary before creating descendants, including deep overrides and case-variant spellings.
  Arbitrary user-project parents are not claimed or restricted.

- `private_temporary_workspace` returns an exclusive RAII allocation, validates its base before creation and secures native ACLs before writes. It leaves copied project trees free of application metadata; its live allocation is ownership evidence, never adoption of an existing path.
- `private_temporary_directory` validates the temporary base before any creation, then allocates unique owned ephemeral directories with automatic cleanup; callers cannot name an existing directory for deletion.
- `private_state_windows.rs` verifies native owner/DACLs and refuses reparse
  points through safe pinned `windows-permissions` wrappers. Verification handles
  share delete access so an overlapping private-record publication is not blocked;
  unrelated handles that prohibit deletion still cause a propagated write failure.
  Windows private-record replacement closes the synced same-directory staging
  handle, clears its temporary attribute, re-arms RAII cleanup, and then uses
  native filesystem rename-over-target. Unix retains its existing persist and
  parent sync. Only fresh or empty owned objects may receive restrictive ACLs;
  parents and Vesper state stay intact.
- This crate depends on no workspace crate. Private-state helpers perform bounded
  explicit filesystem initialization; they never grant tool/process authority.
- Secret values never expose contents through Debug, Display, errors, or serialization.
- General tool sandbox enforcement remains outside these application-private storage guards.
- Unsafe code is forbidden in Stage 1.

## Verification

- The Windows-only `tests/windows_private_replacement.rs` checks 2 × 32 nonhostile same-target writes and 64 whole-record reads, both delete-sharing and non-delete-sharing readers, and barrier-synchronized two-writer denial without retries. A native Windows unit regression holds the production verifier handle through replacement. Unit-test-only Windows stage/error-code diagnostics in `private_state.rs` never log paths or payloads. Cross-compilation does not substitute for native Windows execution or identify Issue #23's historical syscall.
- Run `cargo test -p jarvis-security`.
- Run `cargo xtask architecture`.

## Child DOX Index

No children.
