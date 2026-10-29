# PR #25 → #27 protected integration — execution record

## Objective and scope

Integrate the existing #25, #26 and #27 branches sequentially without release, replacement PR, owner-checkout edits, source-history rewrite or private user-state access. This record distinguishes a passing bounded repair from complete Phase 4 acceptance. Post-publication receipts may be appended locally without a documentation-only push.

## Methods and exact evidence

- Remote `main` at initial readback: `3aa536306d9a7f63d5ae7a22bb08769fb4f64210`. PR #25 candidate `ee74cb6dbfd2c5022015fa22dd2d166c0182b733` passed all 13 required exact-head statuses, including five native foundation jobs (run `40816670420`); synthetic signed-out TUI/ACP, foreign/owned override and Windows/macOS native keyring tests were selected and passed. The four guarded override names are `JARVIS_CREDENTIALS_PATH`, `JARVIS_OPENAI_CREDENTIALS_PATH`, `JARVIS_XAI_CREDENTIALS_PATH`, `JARVIS_LMSTUDIO_CREDENTIALS_PATH`. Source review confirmed malformed-vault bypass is limited to read-only host preflight: credential load and write reject it. No review threads and zero required human approvals per active ruleset `25049950`; independent review NOT COMPLETED. `gh pr ready 25` and `gh pr merge 25 --merge --match-head-commit ee74cb6dbfd2c5022015fa22dd2d166c0182b733` resulted in merge `fb9c027832afb63e66ceb4d0b402ae968e154afe`, also read back as `main`. Exact-main four workflow outcomes: PENDING.
- PR #26 original head `d47e79329ee5204664d40c5c332d8d87d8a44f55`, prior five native job pass run `40665446147`, with selected acknowledged-checkpoint-after-torn-append and owned checkpoint bundle tests on every platform. Original proof and bounded durability exceptions are in `2026-10-28-phase4b-checkpoint-torn-append.md`. Reconciliation uses a detached integration worktree, retaining #25 credential and migration history, #26 checkpoint repair, naming registration and post-publication scope receipts. New final candidate SHA, local commands, exact-head CI and merge: PENDING.
- PR #27 original head `2c72516c93da673d0b750800a7fbab143787450e`. Reconciliation, effective diff, local/native tests, exact-head CI and merge: PENDING.

## Files, verification, deviations and readiness

- Owns this report, its evidence-index and PRD links, and conflict resolution in `docs/foundation/AGENTS.md`, `docs/foundation/evidence-index.md`, `xtask/src/identity_naming.rs` and the scoped checkpoint documentation. Existing historical audit, Issue #23 and credential evidence must remain present. Re-run naming, architecture, formatting, fixture and scoped tests on each reconciled candidate; require all 13 exact-head statuses and four successful workflows on each actual merge SHA.
- Original agent worktrees contain unique unpublished local report receipts. Do not delete without exact inventory and retrievable preservation. Owner primary checkout HEAD `ef227fd0de2b25d812a4e19ea087460362426ba9` remains untouched by this task.
- Independent review is NOT COMPLETED. Issue #23 cause, untested credential consumers, multiprocess JSONL atomicity and power-loss durability remain unresolved. No complete Phase 4 or release claim follows from integrating bounded repairs. Final post-merge disposition and remaining risks: PENDING.
