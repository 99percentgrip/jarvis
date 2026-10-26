# Jarvis interrupted-turn identity correction (2026-10-26)

## Objective and root cause

The Stage 7B merged source still displayed `[Agent Vesper: ...]` to users when a provider stream interrupted after partial output. This is a migration defect, not a compatibility token. The three owning paths were ACP `apps/jarvis-acp/src/lib.rs:2843`, native TUI `apps/jarvis-tui/src/main.rs:14353-14357`, and shared hosted-worker output `crates/jarvis-harness/src/lib.rs:1582` at base `59d174ef032a7cd6cb82edfa2fe093c30d4fa138`.

## Methods, files and evidence

- Used a separate clean worktree from the exact main source; the owner's dirty checkout and original Agent Vesper were not changed. Before the correction, an explicit Python check for legacy user-visible interruption labels failed with `AssertionError: apps/jarvis-acp/src/lib.rs` (exit 1).
- Replaced only four displayed prefix literals with `[Jarvis:`. Cause, partial assistant output, tool-call ambiguity/no replay and cancellation behavior are unchanged. Added direct output assertions to existing ACP/TUI tests and a new shared harness output test. Corrected the two owning app DOX titles, which still described their binaries as Agent Vesper. No wire markers or persisted records were changed.
- `CARGO_BUILD_JOBS=2 cargo test -p jarvis-harness --lib interrupted_worker_result_uses_jarvis_identity_and_retains_partial_text --locked`: **1 passed, 0 failed** (403 filtered).
- `CARGO_BUILD_JOBS=2 cargo test -p jarvis-acp --lib interrupted_outcome_preserves_partial_text_and_safe_cause --locked`: **1 passed, 0 failed** (61 filtered).
- `CARGO_BUILD_JOBS=2 cargo test -p jarvis-tui --bin jarvis-tui apply_agent_event_preserves_interrupted_partial_text_and_plan --locked`: **1 passed, 0 failed** (205 filtered).
- `cargo fmt --all -- --check` and `git diff --check` passed after the correction. Static pre-fix failure is not a substitute for a behavior-level red/green test; host regressions should also be run with the old literals to demonstrate red before PR review.

## Deviations, unresolved and readiness

- No protected merge, exact-head CI/security matrix, cross-platform built archive or real provider interruption acceptance is claimed. This scoped label repair is not a release-candidate verification or evidence that the full migration is complete. No user credentials, registry, release, installer, candidate dispatch or public tag were touched.
