# Phase 5B PR #31 first exact-head CI and ACP fixture repair

## Objective

Reconcile the first exact-PR-head CI run for the offline authentication candidate and correct the first causal failure without weakening provider acceptance or adopting owner credentials.

## Methods and commands

- `gh pr view 31 --json headRefOid,baseRefOid,mergeable,state,statusCheckRollup` identified candidate `7683285d66550b7b23e674e91815f57e8d51bb19`, base `2f7433df570430f05b88d80b2d287119ef876c34` and open/mergeable status.
- `gh run view 41574689110 --job 125715352392 --log-failed` captured the quality failure in `cargo xtask` workspace verification, specifically the ACP swarm-controls process fixture. `gh run view 41574689110 --json jobs` reported `quality failure`, `windows-installer success`, `supply-chain success`. The other workflow runs were not certified by this observation.
- Inspected the ACP process-harness support module and `tests/swarm_controls.rs`, plus the ACP workspace-root resolver and swarm settings check. The process harness named its temporary roots with wall-clock nanoseconds plus an atomic sequence, without atomically reserving the name. Concurrent fixtures could therefore select the same root; one test got `AlreadyExists`, while the other saw a missing workspace during concurrent cleanup.
- Replaced the time-derived path with `tempfile::Builder::tempdir().keep()` to atomically reserve a unique test-process root while retaining the existing explicit cleanup. This is test harness isolation only; no production provider or credential path changed.
- From the isolated candidate worktree: `CARGO_BUILD_JOBS=2 cargo test --offline -p <ACP package> --all-features --test swarm_controls -- --test-threads=4` with `CARGO_TARGET_DIR` set to the managed Phase 5B cache; the ACP package identity is omitted here only to satisfy the naming guard, not from the actual command.

## Exact evidence

First CI run `41574689110`, job `125715352392`:

> `native_settings_save_cancel_and_refusals_never_dispatch_a_provider ... FAILED`
>
> `called Result::unwrap() on an Err value: Os { code: 17, kind: AlreadyExists, message: "File exists" }`
>
> `acp_swarm_audit_surfaces_the_shared_lifecycle_renderer ... FAILED`
>
> `unexpected audit surface text: Swarm settings require an existing absolute workspace root.`
>
> `test result: FAILED. 1 passed; 2 failed`

Focused Linux repair run:

> `test acp_swarm_audit_surfaces_the_shared_lifecycle_renderer ... ok`
>
> `test native_settings_save_cancel_and_refusals_never_dispatch_a_provider ... ok`
>
> `test result: ok. 3 passed; 0 failed`

This focused result does not prove the entire CI matrix or a Windows/macOS regression result. The first failed candidate remains red historically. Replacement head `8d60143b00a57704e22c75f8c4ff86a1a0d9610c` also failed the quality naming gate in run `41577252673`, before workspace tests: four lines in *this new report* contained the embargoed upstream stem within the ACP package/path spelling. The failure was documentation-only, not a renewed swarm test failure; no baseline or pattern was changed. Reworded those four lines without the stem, then ran `cargo xtask naming-guard`: `naming-guard: clean (36 hits, all frozen in baseline)`. The other jobs on that head do not turn its quality failure green. The successful exact-head rerun is recorded below.

## Files, deviations and unresolved items

Changed the ACP process-harness support module and this report, with links from the foundation evidence index and runtime-identity PRD. The original PR candidate did not pass the mandatory quality gate; no merge, release, installation, or live provider test occurred. The local focused test used the isolated worktree but did not run inside a network namespace; the fixture runs a production ACP process with synthetic signed-out credential records and a loopback listener, but the focused run is not counted as a provider isolation acceptance result. The wider Phase 5B host/platform matrix and independent review remain open as described in the offline execution report. A broad existing evidence-index link sweep failed on unrelated missing `2026-09-29-active-agent-supervision-0845z.md`; focused checks verified both Phase 5B reports' local links and the index/PRD pointers instead. The coverage JSON parsed, `git diff --check` passed, and `cargo xtask naming-guard` reported `clean (36 hits, all frozen in baseline)` after the local receipt update. The later exact-head matrix passed, but separate merge authorization and missing acceptance evidence remain outstanding.

## Exact-head replacement CI closeout (local post-publication receipt)

At PR #31 head `5f86bfa587922c5347b72e4c56cb459544d53037`, `gh pr view 31 --json headRefOid,mergeable,state,statusCheckRollup` reported `state: OPEN`, `mergeable: MERGEABLE`, and **13/13 checks COMPLETED/SUCCESS**. `gh run list --commit 5f86bfa587922c5347b72e4c56cb459544d53037 --json name,databaseId,status,conclusion` reported the following five complete/successful workflows:

| Workflow | Run ID | Conclusion |
| --- | --- | --- |
| web-driver | `41579413673` | `success` |
| pull-request-validation | `41579413743` | `success` |
| msrv | `41579413672` | `success` |
| PR matrix verification (product prefix elided by naming guard) | `41579413702` | `success` |
| five-target-foundation | `41579413686` | `success` |

Exact-head successful check names (product prefix elided by naming guard for the first label): `verification`,  `linux-x86_64`, `rust-1-88`, `quality`, `Contained browser linux-x86_64`, `linux-arm64`, `Contained browser linux-aarch64`, `macos-intel`, `macos-apple-silicon`, `windows-x86_64`, `windows-installer`, `Native namespace Hive`, `supply-chain`. GraphQL `reviewThreads(first:100)` returned `total: 0, unresolved: 0`. CI green is an exact-head gate receipt, **not** execution of the missing scenario × provider × host × platform acceptance matrix or native credential-manager tests. The original two red quality runs remain failures on their respective older heads. This paragraph is a local after-publication receipt; it has not been pushed as a documentation-only commit.

## Readiness effect

Fixture collision has a focused local correction and PR #31 has a successful exact-head mandatory matrix. The PR remains open and unmerged; no release or installation is authorized. Phase 5B offline authentication acceptance remains incomplete because the unexecuted host/platform/security scenarios listed in the offline execution report remain NOT TESTED.
