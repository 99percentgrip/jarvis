# Stage 7B native acceptance reverification — execution report

## Objective and status

Reverify the retained dirty, nonpublishing Stage 7B candidate after the storage-safety fixes and the earlier native D-Bus build failure. The native acceptance blocker is resolved in this workspace; **Stage 7B as a whole remains unaccepted** because no exact-head hosted five-target matrix or candidate workflow run has been produced.

## Methods, files and exact evidence

Re-read the root, documentation/foundation, crate/harness, app/ACP, workflow, script and registry DOX chains; inspected the candidate workflow, storage boundary and earlier reports. Verified the feature-gated ACP process case with `--all-features` (without that flag the Linux integration fixture compiles zero tests). No source or packaging behavior was changed in this follow-up; this report, the evidence index and the staged architecture specification are the changed files. Existing uncommitted Stage 7B source and the unrelated `.agent/` file were retained.

- `CARGO_TARGET_DIR=/home/Alex/Projects/jarvis/target cargo test --locked -p jarvis-acp --all-features --test release_resource_governor acp_process_release_status_uses_governor_for_every_cargo_path -- --nocapture`: `running 1 test`, `test acp_process_release_status_uses_governor_for_every_cargo_path ... ok`, `1 passed; 0 failed`.
- `CARGO_TARGET_DIR=/home/Alex/Projects/jarvis/target cargo xtask acceptance`: `Acceptance regression gate: 165 exact cases passed in 221598 ms. Offline fixture model cost: zero; live-model effectiveness is not measured.` Exit 0. This supersedes the earlier local D-Bus failure as the **latest** local acceptance result; it does not change that the earlier attempt failed before completing.
- `python3 scripts/test_release_gate.py -q`: `Ran 15 tests in 0.022s`, `OK`.
- `cargo fmt --all -- --check` and `git diff --check`: exit 0 before the prose-only report update; whitespace and link checks were repeated after updating the report.
- `registry/independent-registration.json`: parsed with `json.loads`; workflow YAML parsed with `yaml.safe_load`, yielding `candidate-gate, package`. Parsing is not a hosted execution receipt.

## Deviations, unresolved items and readiness effect

The ACP fixture requires `--all-features` / `integration-test-harness`; an earlier no-feature invocation selected zero cases, which is **not** evidence. Native prerequisites were present for the successful rerun; no installer was run and no user installation was replaced. The previous storage hardening report records its own red-first regressions and library receipts; this report does not reclassify those older receipts as exact-head CI. The known historic broken evidence-index link to `2026-10-02-agent-process-status-check.md` remains unresolved. No live-provider effectiveness test, five-platform exact-PR-head CI/security run, candidate workflow dispatch, uploaded archive inspection, installer acceptance, publication, tag or registry mutation occurred. This is improved Linux-local confidence only, not completion or cross-platform release readiness. DOX owning contracts were rechecked and intentionally left unchanged: this verification changes no scope, ownership, or operational rule.
