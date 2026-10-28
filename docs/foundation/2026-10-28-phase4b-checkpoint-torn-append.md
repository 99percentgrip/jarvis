# Phase 4B — checkpoint crash recovery (bounded implementation)

## Objective and scope

Continue from `main` `1e01c4edd7bcda876b6b9c72f6888f0782fd39a7`, independently of draft PRs #21, #24 and #25, without using personal credentials or Vesper state. Test an actual checkpoint consumer's recovery after an interrupted append, correct only a demonstrated defect, and retain incomplete Phase 4B acceptance explicitly. [Coverage matrix](phase4b-runtime-recovery-matrix.json) separates credential, checkpoint and remaining session/coexistence scenarios.

## Production route and methods

- TUI `apps/jarvis-tui/src/main.rs`: `CheckpointStores::open_at` → `jarvis_security::prepare_owned_directory` → `CheckpointsLedger::open/create` → shared `append_line` → `CheckpointsLedger::open` after restart. `JARVIS_CHECKPOINT_ROOT` or `.jarvis/checkpoints` selects this private store; application-private checks do not constrain arbitrary project files.
- Reused `ledger::tests::create_persists_across_reopen`, TUI `private_store_bundles_never_open_unowned_roots`, auth `native_persistence` (hosted cases ignored locally), and `jarvis-sessions --test writer`. The two pre-existing checkpoint tests cover intact records and unowned roots but not an unterminated suffix followed by an acknowledged append.
- Injected a truncated JSONL suffix into a disposable owned store after a valid checkpoint, reopened the public ledger, created a new checkpoint, dropped the store and reopened. Before repair the regression **failed**: `assertion left == right failed: acknowledged checkpoint must survive restart; left: 1 right: 2`. Cause: `append_line` concatenated the new valid record onto the prior unterminated line; reload skipped the combined invalid line. A successful append therefore did not imply recoverability.
- Repaired the shared append helper by reading the previous final byte on the same opened file and inserting a record separator only when needed. It still syncs before returning, retains existing JSONL format and process-local mutex discipline. This is not a claim of multi-process transactional append safety, sector-level crash atomicity, or recovery of invalid UTF-8.
- Added a TUI composition regression using disposable owned and foreign-profile roots. It exercises publication, interruption, reopening and repeat publication via `CheckpointStores::open_at`, checks both acknowledged IDs after restart, restores the owned workspace file through the public ledger, and verifies the synthetic foreign profile's content and entry count did not change. This does not prove native ACL metadata, reparse-point behavior or all consumer paths.

## Files

- `crates/jarvis-checkpoints/src/io.rs`, `src/ledger.rs`, `AGENTS.md` — shared bounded recovery fix, red-first checkpoint regression, contract.
- `apps/jarvis-tui/src/main.rs` — host composition/foreign-profile regression; `apps/jarvis-tui/AGENTS.md` remains unchanged because no host contract changed.
- `docs/foundation/phase4b-runtime-recovery-matrix.json`, this report, `docs/foundation/evidence-index.md`, `docs/foundation/AGENTS.md`, `docs/runtime-identity-stage3-prd.md` — scope, evidence, indexes.

## Exact local execution receipts

- Before repair: `cargo test -p jarvis-checkpoints acknowledged_checkpoint_after_torn_append_survives_restart -- --nocapture` → **FAILED**, `left: 1 right: 2` (1 selected, 0 ignored). An earlier `--exact` selector selected **0 tests** and was not counted as proof.
- After repair: same selected regression → `1 passed; 0 failed; 0 ignored`; `cargo test -p jarvis-checkpoints --quiet` → `48 passed; 0 failed; 0 ignored`.
- `cargo test -p jarvis-tui --bin jarvis-tui owned_checkpoint_bundle_recovers_after_torn_record_without_adopting_foreign_state -- --nocapture` → `1 passed; 0 failed; 0 ignored; 206 filtered out` on Linux.
- Baseline tests: `cargo test -p jarvis-auth --all-features --test native_persistence` → `1 passed; 2 ignored` (not native credential acceptance); `cargo test -p jarvis-sessions --test writer --quiet` → `10 passed; 0 failed`.
- `cargo xtask architecture` → `architecture boundaries validated for 32 packages`; `cargo xtask naming-guard` → `clean (36 hits, all frozen in baseline)`; targeted all-features strict Clippy on `jarvis-checkpoints` and `jarvis-tui`, workspace format, and `git diff --check` returned zero. Full workspace, native Secret Service/keyring, Windows and macOS source-head execution remain pending.

## Deviations, unresolved gates and readiness

- Independent separate-context review was attempted once and failed *before assessment*: `Selected OpenAI model is not in the current account model list`; no independent approval claimed.
- No owner profiles, OS keyring, paid provider, Vesper installation, migration, installer, merge or release was accessed. Draft PRs #21/#24/#25 and Issue #23 remain separate.
- At report creation this is local Linux-only proof. Candidate SHA, remote HEAD, exact-head CI, native Windows/macOS execution and mandatory check conclusions must be added to the **local report** after publication; the tested release candidate includes this report, matrix and owning docs. Do not push a follow-up documentation-only commit solely to record post-publication receipts.
- Phase 4B remains **PENDING** until exact-head/native verification and broader required credential, session, checkpoint and coexistence acceptance are performed; Phase 4, migration, alpha and production remain incomplete. A focused defect correction is not the whole milestone.
