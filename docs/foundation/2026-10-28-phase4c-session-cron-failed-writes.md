# Phase 4C — session and cron failed-write consistency

## Objective

Continue the bounded Phase 4 runtime-isolation investigation from `d47e79329ee5204664d40c5c332d8d87d8a44f55` in a separate worktree. Check whether production session and scheduled-job consumers publish unacknowledged state after a durable write fails. Preserve the owner's checkout, foreign profiles and existing draft PRs.

## Methods and exact evidence

- Inspected the public `SessionLineage` and `CronRegistry` implementations and shared JSONL write helpers. Used only temporary owned directories; replaced each log path with a directory after saving the original log to induce a portable atomic-rename failure, then restored the original log and reopened it. No real credentials, user profiles, native services or foreign filesystem data were read or modified.
- Before the correction, `CARGO_BUILD_JOBS=2 cargo test -p jarvis-checkpoints --test failed_mutations -- --nocapture` returned `FAILED. 0 passed; 2 failed` before the third lease-settlement regression was added. The cron case reached `assertion failed: cron.claim_due(std::time::SystemTime::now(), true).is_err()` because an earlier failed pause changed the cache; the session case returned `left: "after" right: "before"` after a failed rename. This is direct evidence of an acknowledged error exposing unpersisted state.
- Rewrites now operate on staged copies under the existing mutex, publishing the cache only after successful durable replacement. Session creation commits parent supersession and child in one atomic rewrite rather than first appending a child and then attempting a second rewrite. Append-based branch and registration advance IDs only after append success. This does **not** make multi-process operations atomic or prove power-loss durability of the directory entry.
- After correction, `CARGO_BUILD_JOBS=2 cargo test -p jarvis-checkpoints --quiet` returned `48 passed; 0 failed` plus `3 passed; 0 failed` in `failed_mutations`; `CARGO_BUILD_JOBS=2 cargo clippy -p jarvis-checkpoints --all-targets -- -D warnings` and `git diff --check` returned zero. `cargo xtask naming-guard` initially failed with `3 new upstream-brand hit(s)` on exact technical command/path receipts; the report-only technical-document registration and a scoped allow/deny regression corrected this without changing the baseline or upstream patterns. `cargo test -p xtask phase4c_report_admits_technical_receipts_only --quiet` returned `1 passed; 0 failed`; rerun `cargo xtask naming-guard` returned `clean (36 hits, all frozen in baseline)` and `cargo xtask architecture` returned `architecture boundaries validated for 32 packages`. Format, JSON parsing, report/PRD/index links and whitespace checks passed. The store regressions assert live cache and reopened file invariants and retry IDs; they are same-process Linux tests, not native cross-platform acceptance.

## Files

- `crates/jarvis-checkpoints/src/cron.rs`, `src/sessions.rs`, `tests/failed_mutations.rs`, and its owning `AGENTS.md`.
- `xtask/src/identity_naming.rs`, `xtask/AGENTS.md` — scoped technical-receipt registration and deny regression.
- This report, `phase4b-runtime-recovery-matrix.json`, `evidence-index.md`, foundation `AGENTS.md`, and the owning runtime-identity PRD.

## Deviations, open items and readiness

- This focused follow-up does not execute OS credential stores, hosted wrappers, actual separate-process restart, Windows/macOS native CI, ACL or reparse-negative cases, or comprehensive interrupted-append recovery; those remain open. The directory fault fixture is deliberately narrower than a power failure. No independent review, Phase 4C draft PR, exact-head CI or merge is claimed here yet.
- Independent reviewer delegation was attempted once and returned `Selected OpenAI model is not in the current account model list`; no independent review or approval is claimed. PR #26 at `d47e79329ee5204664d40c5c332d8d87d8a44f55` has 13 successful exact-head checks and named checkpoint/TUI tests in Linux x86_64, Windows x86_64 and both macOS native jobs; those are historical Phase 4B source receipts, **not** native evidence for this Phase 4C change. Its owning report records the job IDs.
- Existing Phase 4B report and its receipts remain historical. Phase 4B and Phase 4 are **incomplete**; alpha/production clearance is not implied. A new exact-head candidate with native verification and independent review is required before cross-platform claims.
