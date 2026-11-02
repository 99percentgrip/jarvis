# Phase 5B — credential reliability candidate

## Objective and baseline

From integrated `main` `4db508421346ac20c14fbb9a328e587ae851a7b7`, audit production credential consumers, repair demonstrated corruption behavior and preserve previous unpublished work. This is **not** full Phase 5B acceptance or a native-platform closeout.

## Methods and production trace

- Refreshed `origin/main` and GitHub: `gh pr list --state open` returned `[]`; `gh api .../commits/main` returned `4db508421346ac20c14fbb9a328e587ae851a7b7`. Created isolated `audit/phase5b-credential` worktree; owner checkout unchanged. The merged PR #28 worktree has modified and untracked unpublished reports, so it was retained rather than removed. The investigation worktree with unclassified state was also retained.
- Traced ordinary ACP `apps/jarvis-acp/src/main.rs:main` and TUI `apps/jarvis-tui/src/main.rs:run` preflight into `validate_production_credential_overrides`, and provider adapters into `SecureCredentialStore`. GLM uses `zai/api-key`, OpenAI `openai/native-auth`, xAI `xai/native-auth`, and both host LM Studio compositions `lmstudio/api-key`; the application service is `99percentgrip.jarvis`. `crates/jarvis-auth/src/native_store.rs` selects OS backends. This source trace is not an interactive sign-in or OS-manager test.
- Reviewed the existing `phase4-auth-persistence-coverage.json` and native fixture tests. Reconciled its base with integrated main while retaining the historical proof SHAs; corrected its obsolete Issue #23 status using actual merge-run receipts, without claiming retrospective knowledge of the old syscall.

## Confirmed defect and correction

An owned JSON vault containing an invalid *targeted credential value* passed owner preflight. `PrivateFileCredentialStore::load` used `validate_record(value).ok()` and returned `None` for an invalid value. `SecureCredentialStore::load` then attempted native lookup; `store` could attempt native mutation before the fallback removal reported failure, and `remove` could attempt native deletion first. The security contract requires a corrupt retained record to fail closed, not become an absent credential.

`crates/jarvis-auth/src/lib.rs` now propagates targeted-value validation errors with `transpose()` and checks a retained record before native write/delete. A UUID-scoped synthetic `openai` record containing a control character and a healthy `xai` neighbor live in a disposable owned vault. The regression checks targeted load/store/remove refusal, neighbor readability and byte-for-byte vault preservation. No production provider request or real credential write is made.

**Red first:** `CARGO_BUILD_JOBS=2 cargo test -p jarvis-auth --test credential_corruption -- --nocapture` → `running 1 test`, `... FAILED`, `assertion failed: matches!(store.load(OPENAI), Err(CredentialStoreError::InvalidSecret))`. This initial red iteration used the default OpenAI account identifier, and therefore could have attempted a native *read* in the local user's credential manager. No value was logged, copied, changed or used; this was a fixture-isolation deviation. The retained regression was corrected to use a UUID-scoped account before further execution. No native manager observation of the initial lookup is claimed.

**Green:** `CARGO_BUILD_JOBS=2 cargo test -p jarvis-auth --test credential_corruption` → `1 passed; 0 failed`. `CARGO_BUILD_JOBS=2 cargo test -p jarvis-auth --all-features` → 18 unit passed, corruption integration 1 passed, receipt parser 1 passed, hosted Linux fixture 4 ignored, hosted manager fixture 2 ignored. An initial `cargo fmt --all --check` failed due to formatting of the new test; `cargo fmt --all` and the final check passed.

Additional Linux-local receipts: `cargo test -p jarvis-acp --test phase4_credential_boundary` → 3 passed (actual process startup, ownership override; **not** provider login); `cargo test -p jarvis-tui --test runtime_credentials` → 1 passed (fixture through production reader; **not** a real PTY sign-in); `python3 .github/test_credential_backend.py` → 5 passed; `cargo clippy -p jarvis-auth --all-targets --all-features -- -D warnings` → success; `cargo xtask architecture` → 32 packages validated; `cargo xtask naming-guard` → 36 frozen hits, clean.

## Changed paths

`crates/jarvis-auth/src/lib.rs`, `crates/jarvis-auth/tests/credential_corruption.rs`, `crates/jarvis-auth/AGENTS.md`, `docs/foundation/phase4-auth-persistence-coverage.json`, this report, `docs/foundation/evidence-index.md`, `docs/foundation/AGENTS.md`, `docs/AGENTS.md`, `docs/runtime-identity-stage3-prd.md`, `xtask/src/identity_naming.rs` and `xtask/AGENTS.md`. The new report initially failed naming-guard (six unregistered technical receipt lines); registering only its technical path and a negative guard test restored the unchanged 36-hit baseline.

## Deviations, open items and readiness

- The initially executed red fixture did not fully isolate a native read. Subsequent tests use UUID-scoped identities. No real-account authentication or paid provider calls were authorized or executed.
- Real Linux locked/missing service, macOS/Windows native rotation and failed-manager behavior, actual signed-out TUI PTY and ACP provider operations, malformed-state native manager behavior, complete provider/host suites, cross-platform exact-head CI and independent review are **NOT TESTED on this candidate**. Previous PR #25 native receipts remain historical and do not prove this change.
- No new PR, exact-head commit, CI run, merge, release or tag is claimed in this checkpoint. Merge readiness is **NO** until code review and mandatory exact-head native CI, with remaining Phase 5B coverage independently classified. The old Issue #23 exact-merge verification is separate.
