# Stage 7B local continuation closeout

## Objective and status

Continue the independent, nonpublishing Stage 7B candidate without treating a blocked workstation or unexecuted CI as success. This closes the locally actionable storage-safety work unit; **Stage 7B as a whole remains unaccepted**. The detailed red/green history is in [storage ownership hardening](2026-10-20-stage7b-storage-ownership-hardening.md), with the earlier [candidate continuation](2026-10-20-stage7b-independent-candidate-continuation.md).

## Methods, files and evidence

Read the root and owning DOX hierarchy, source diffs, candidate workflow, package builder, storage registry and process regression. Changed `crates/jarvis-harness/src/host_storage.rs` and `crates/jarvis-harness/src/release_executor.rs` with red-first owner/path/lease regressions, and aligned the ACP target-cache regression under `apps/jarvis-acp/tests/` in the earlier checkpoint. The owner/metadata/target redirects, stale atomic-write blockage, ambiguous live-lease clearing, cleanup-pending reactivation and foreign resource-record admission are now locally refused. Updated the nearest harness and documentation contracts, Stage 6 publication-boundary description, and evidence index; existing unrelated worktree bytes were not reset.

Exact final commands and receipts:

- `cargo test --locked -p jarvis-harness --lib --no-default-features --quiet`: **400 passed, 0 failed, 2 ignored** of 402.
- `cargo test --locked -p jarvis-harness --all-features --lib --quiet`: **450 passed, 0 failed, 6 ignored** of 456.
- `cargo clippy --locked -p jarvis-harness --lib --all-features -- -D warnings`: passed.
- `python3 scripts/test_release_gate.py -q`: **15 tests, OK**.
- `cargo fmt --all -- --check`, `git diff --check`, candidate-YAML parse, new-report-link existence and independent-registration JSON parse: passed.
- `cargo xtask acceptance`: **failed at named case 7/165**, `tests::native_release_controls_bypass_pending_account_discovery`, during `libdbus-sys v0.2.7` compilation: `dbus-1.pc` not found. First six cases were verified; 159 remaining cases were not executed. The ACP process case from the earlier report is blocked by the same dependency. No OS packages were installed to change the user's machine.
- Whole historical evidence-index link sweep: **failed** on the existing missing `2026-10-02-agent-process-status-check.md`. The new links resolve; the unrelated historic defect remains open.

## Deviations, unresolved items and readiness

No full canonical verification, five-target exact-head PR matrix, dispatch of the nonpublishing candidate workflow, all-target package/install acceptance, local full 165-case acceptance, production release RRC, tag, publication, registry mutation, installer execution or user installation occurred. Source-only and Linux-local checks do not establish Windows/macOS lifecycle, candidate artifact or release readiness. The dirty branch still needs review, an environment with native D-Bus development files, and authorized exact-head CI before a Stage 7B completion claim. No separate Stage 7B PRD exists in this checkout; the closest staged architecture specification and evidence index link this report. Root and parent child indexes were reviewed and intentionally left unchanged because the scoped work added no boundary.
