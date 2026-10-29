# Phase 3 exact-head GitHub CI settlement (2026-10-26)

## Objective

Correct the prematurely delivered Phase 3 checkpoint by waiting for all 13 required GitHub checks to settle on BOTH current PR heads. This report supplements [the migration Phase 3 execution report](2026-10-26-migration-phase3-execution.md); it is not an independent review, native release candidate, merge authorization or migration completion.

## Methods and files

- Re-read live PR head SHAs and GitHub `gh pr checks 21/22`, active ruleset `25049950`, commit check-runs and each referenced workflow run/attempt. Waited while jobs were genuinely in progress; no reruns, cancels or branch mutations. Saved [the final exact-head snapshot](2026-10-26-migration-ci-snapshot-final.json) with every required check's name, workflow, run ID, job/check-run ID, attempt, head/run SHA, status, conclusion and URL. The 03:15 and 03:24 interim snapshots remain as historical observations rather than being silently overwritten.
- Read current Windows job log and the older failed Windows log; read live GitHub PR review/draft/merge metadata. No paid provider calls, installs, fixture writes or source modifications were performed.
- Changed files for this closeout: this report, final CI snapshot, the Phase 3 report correction, evidence index, owning audit plan and owning documentation DOX. No PR-head commit/push occurred; these receipts exist only after the PR heads were published and remain local until a separately authorized update.

## Exact receipts

At `2026-10-26T04:49:54Z` (snapshot timestamp, not a prediction):

| PR | SHA | All required checks | Workflow runs (all attempt 1) | Pending / failed / skipped / cancelled |
| --- | --- | --- | --- | --- |
| #21 | `cdc588889009f85c0fd61cf46ef5000d52be6b9a` | **13/13 completed success** | `40261508676`, `40261508677`, `40261508679`, `40261508690`, `40261508691` | **0 / 0 / 0 / 0** |
| #22 | `c66bd2e77aaa4109e5be7c9a2e633725e615d6a1` | **13/13 completed success** | `40261216011`, `40261216014`, `40261216049`, `40261216084`, `40261216149` | **0 / 0 / 0 / 0** |

All 13 names match ruleset `25049950`; each current-head check's own `head_sha`, workflow `run_head_sha`, and PR SHA are equal in the linked JSON. GitHub reports each five-target workflow `completed/success` at attempt 1. PR #21 current-head Windows job `121375045184`, run `40261508677`, attempt 1: `test held_pipe_fixture_allows_slow_startup_before_settlement ... ok` at `03:06:22Z` under foundational tests, `14 passed; 0 failed` at `03:06:51Z`; it passed again under `Run shared command-settlement behavior` at `04:43:10Z`, `14 passed; 0 failed` at `04:43:39Z`. This is relevant current-head Windows evidence, NOT an explanation of why the earlier head failed. PR #22 Windows job `121374052054` also completed success. The earlier PR #21 head `9e756a24a7ec1f3f2fb8195b057aba556e063031` still has the real failed Windows job `121368991248`, run `40259554518` (eventual workflow conclusion `cancelled` after supersession): readiness at 6.49 seconds followed by post-readiness three-second timeout. Its cause remains unknown; a later pass does not retroactively erase the failure, and no timeout/test was weakened.

## Deviations and unresolved items

The previous delivery reported a completion checkpoint before GitHub produced these results. That was incorrect; the subsequent wait and final snapshot now close only the **current-head CI reconciliation** requirement. Independent adversarial reviewer remains **NOT TESTED**: the isolated worker failed to start because its configured model was unavailable. Both PRs are still **draft**, have zero recorded reviews and no independent approval; GitHub `mergeStateStatus=CLEAN` and `mergeable=MERGEABLE` are technical eligibility signals, not permission to merge. Protected merge is separately unauthorized. No reviewed native *candidate* derived from a merged final migration source exists; signed-out/private-store native acceptance, 1,067 UNKNOWN text files, 110 unreviewed binary leads, version `0.24.14` and `DISPLAY-01` pending PR #22 integration remain open.

## Readiness effect

- Exact-head mandatory CI: **PASS for the two stated draft heads only**. New commits invalidate these receipts and require all 13 checks anew. Older red/aborted attempts remain historical evidence.
- PR #21: **BLOCKED for merge/acceptance** (draft audit, migration findings unresolved; no merge authority). PR #22: **BLOCKED for merge/acceptance** (draft, independent review NOT TESTED; do not confuse 13 green checks with adversarial acceptance).
- Migration **FAIL**, experimental alpha **FAIL**, production **FAIL**; accepted final release commit **none**. Stage 7B run `40217349974` remains only historical acceptance of `59d174ef`; it is not validation of the two PRs or a release source.
