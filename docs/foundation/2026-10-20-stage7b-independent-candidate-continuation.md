# Stage 7B independent candidate continuation — execution report

## Objective and status

Continue the in-progress, **nonpublishing** Stage 7B candidate from the existing dirty branch without resetting earlier work or authorizing release. Align the real ACP governor regression with the pending single repository `target/` policy. This is an intermediate checkpoint, **not** Stage 7B completion or release readiness.

## Methods and files

Read the root and owning `.github/`, `scripts/`, `registry/`, `crates/`, `crates/jarvis-harness/`, `apps/`, `apps/jarvis-acp/`, `docs/` and `docs/foundation/` contracts; inspected the working-tree diff, candidate workflow, package builder, storage registry and governor. Changed `apps/jarvis-acp/tests/release_resource_governor.rs`: the process test now requires the exact repository `target/` for check, metadata and gate, and rejects the former controller-owned target directory. No release workflow was dispatched, no installer run, and no state from the existing dirty worktree was discarded.

## Exact verification receipts

- `python3 scripts/test_release_gate.py -q`: `Ran 15 tests in 0.015s`, `OK`.
- `cargo test --locked -p jarvis-harness --lib host_storage::tests --no-default-features --quiet`: `9 passed; 0 failed`.
- `cargo test --locked -p jarvis-harness --lib host_resources::tests:: --no-default-features --quiet`: `26 passed; 0 failed`.
- `cargo test --locked -p jarvis-acp --test release_resource_governor acp_process_release_status_uses_governor_for_every_cargo_path -- --nocapture`: **failed before compiling the test**. `libdbus-sys v0.2.7` could not find `dbus-1.pc` (`Package dbus-1 was not found in the pkg-config search path`). This is not a passing ACP process receipt.
- The first `cargo fmt --all -- --check` reported a line-wrap diff in the changed ACP assertion. After correction, `cargo fmt --all -- --check` passed. `git diff --check` and the new Markdown-link existence check passed.

## Deviations and unresolved items

The existing dirty Stage 7B changes to `.github/workflows/release.yml`, `host_resources.rs`, `host_storage.rs`, `release_executor.rs` and the dormant independent registration descriptor predate this checkpoint. They have not been certified by this report. The old ACP test had asserted the inverse cache policy; its source assertion is updated, but native execution is blocked here by missing system development files. No five-target candidate packaging, full canonical verification, `cargo xtask acceptance`, MSRV, native release controller progression, publication, tag, registry submission or production installation was performed. The storage lifecycle and deterministic worktree reuse require a further safety audit and current exact-head CI before readiness can be claimed.

## DOX pass

Updated the nearest ACP, harness, workflow, scripts, registry and documentation ownership contracts to reflect the current bounded Stage 7B state. Root, `crates/` and `apps/` parent indexes were reviewed and left unchanged: no new child boundary or parent-level ownership was introduced. The Stage 6 architecture page is the closest existing phased specification; no independent Stage 7B PRD exists here, so the report is linked from that specification and the evidence index rather than falsely claiming PRD acceptance.

## Readiness effect

One stale process assertion is brought into line with the proposed single-target cache policy. Stage 7B remains in progress and nonpublishing; the failed ACP compilation and absent platform/candidate gates are explicit open evidence.
