# First-party identity specification

## Purpose

Own Stage 1 planned identity, package ownership and compatibility policy for
J.A.R.V.I.S.; remain separate from active runtime composition.

## Ownership

- `src/lib.rs`, `packages.rs`, `mappings.rs`, `platform.rs` own typed declarations
  and pure validation. Production dependencies and side effects are forbidden.
- `tests/contracts.rs` checks actual Cargo metadata, source evidence, platform
  defaults and negative registry mutations using isolated verification inputs.

## Local Contracts

- No filesystem, environment, process, network or credential access in `src/`.
- No existing package, executable, protocol or persisted identity is renamed here.
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
