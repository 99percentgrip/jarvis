# Tokio I/O-safety advisory remediation — execution record

## Objective and boundary

Phase A of the owner-authorized nonpublishing recovery: remove RustSec `RUSTSEC-2026-0386` from the pinned dependency graph without changing application code, security policy, unrelated dependencies, or the disabled candidate workflow. Phase B (existing PR #17) is independently gated on Phase A's protected merge and four exact-merge workflows.

## Starting evidence and methods

GitHub `main` and the new isolated branch began at `26dd400e7aa2b92a61baa5c42e6d86b8958ff6b9`. PR #17 remained open at `5c7dea6ec95d26eb5e1d466e0d2d4ce135ed1cea`; release-candidate workflow `383935829` was read as `disabled_manually`. The active solo-maintainer ruleset requires thirteen named CI contexts, zero approving reviews, and resolved review threads. Canonical checkout dirty documentation, untracked historical artifacts, five stashes, and the existing PR #17 worktree were left intact; Phase A uses `.agent/tokio-security-worktree`.

The official RustSec database entry names the vulnerable safe `AsyncFd` constructors, states patched ranges `>= 1.51.6, < 1.52.0` and `>= 1.53.3`, and describes file-descriptor validity and lifetime hazards. Crates.io version metadata for 1.53.3 returned `yanked: false`, `rust_version: 1.71`, checksum `ce3335fa71841cda333a58d7615b03901380ecf09d59b3296d21f8bbac0dde4e`. This compiler floor is below the workspace's Rust 1.88 MSRV. A bounded first-party search across `crates/` and `apps/` for `AsyncFd`, `try_with_interest`, and `register_with_interest` found no direct calls; this is **not** a transitive dependency source audit or a proof about arbitrary descriptor handling.

## Correction and files

`Cargo.toml` keeps the same feature list and `default-features = false`, changing only Tokio `=1.52.0` to `=1.53.3`. `cargo update -p tokio --precise 1.53.3` changed exactly the Tokio package entry in `Cargo.lock`: `1.52.0`, checksum `a91135f59b1cbf38c91e73cf3386fca9bb77915c45ce2771460c9d92f0f3d776`, to `1.53.3`, checksum `ce3335fa71841cda333a58d7615b03901380ecf09d59b3296d21f8bbac0dde4e`; registry source remains unchanged. No transitive resolver entries changed; `tokio-macros 2.7.2`, `tokio-util 0.7.17` and `tokio-tungstenite 0.28.0` remain as pinned. `cargo metadata --locked --format-version 1` succeeded; `cargo tree --locked -i tokio` resolved `tokio v1.53.3` across direct and transitive users. The prior Tokio version was covered by the advisory; the replacement falls within the officially patched range and is not yanked according to version metadata.

## Verification and protected integration

Local receipts in the isolated worktree: `cargo metadata --locked --format-version 1` succeeded; `cargo audit` exited 0 after scanning 542 lockfile dependencies; `cargo deny --all-features check` exited 0 with `advisories ok, bans ok, licenses ok, sources ok` (pre-existing duplicate warnings); `cargo xtask naming-guard` exited 0 with `naming-guard: clean (36 hits, all frozen in baseline)`; a locked Rust 1.88 check selecting the runtime library and both native host packages exited 0; the locked runtime-library test suite exited 0 with `4 passed; 0 failed`; and `git diff --check` exited 0. Exact PR-head CI and protected merge are still pending. Local Linux checks do not establish native cross-platform acceptance. No Phase B integration or new candidate is authorized by this report.

## Deviations, unresolved items and readiness

The first PR #18 head `6ffc1175bab6ca3af39ea0ac4be632beca8c6465` failed canonical quality job `120719147506` (run `40043309345`) at the naming guard: the report's newly added line 19 contained a non-exempt technical package identifier in prose. Its initial local naming check preceded that report edit; this sequencing was incorrect. The failing report wording has been replaced by a plain-language description of the same checks without changing the naming policy. The first head's supply-chain and contained-browser jobs succeeded; other jobs were still running at diagnosis. The failed head is retained as evidence, not treated as green. The correction requires a fresh exact-head matrix before merge. Local validation and hosted cross-platform acceptance are separate; Phase A is not complete until protected integration and four exact-merge workflows succeed. Historical candidate run `39994107258` remains failed with no accepted Windows artifact. No release, candidate rerun, installer execution on the owner's machine, or production mutation was performed.
