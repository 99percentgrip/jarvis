# First-party identity specification

## Purpose

Own current Stage 2 Rust identity, explicit legacy-to-current ownership and deferred
compatibility policy for
J.A.R.V.I.S.; remain separate from active runtime composition.

## Ownership

- `src/lib.rs`, `packages.rs`, `mappings.rs`, `platform.rs` own typed declarations
  and pure validation. Production dependencies and side effects are forbidden.
- `tests/contracts.rs` checks actual Cargo metadata, source evidence, platform
  defaults and negative registry mutations using isolated verification inputs.
  Its Windows Job Object launcher contract checks the actual Python activation
  key against the registered Rust guard on every platform; native execution is separate.

## Local Contracts

- `src/runtime.rs` records reviewed active runtime environment and path identifiers, independently of legacy/provenance mappings. Serialized format constants and synthetic canaries are not environment variables and remain preserved. Runtime behavior lives in config/security/compositions; this specification crate stays side-effect-free with zero production dependencies and no runtime consumers.

- No filesystem, environment, process, network or credential access in `src/`.
  No custom build target; automatic build scripts remain disabled.
- Package/import/manifest/executable registry entries reflect actual Stage 2 metadata.
  Explicit legacy names/manifests preserve migration provenance; deferred protocol
  and persistence identities remain planned and unwired. `xtask` remains unchanged.
- Every legacy mapping cites inspected source; unknown targets remain `None`.
- Credential and serialization contracts are migration-sensitive; provenance,
  provider identities and third-party names are never mechanical replacements.
- Future import is opt-in, excludes RRC ledgers and cannot replay interrupted work.

## Work Guidance

- Keep new package ownership explicit; preserve legacy readers and attribution.

## Verification

- `cargo test -p` for `jarvis-identity`; `cargo xtask architecture`; full native CI/MSRV.

## Child DOX Index

No children.
