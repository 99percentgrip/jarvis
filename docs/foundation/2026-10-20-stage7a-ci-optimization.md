# Stage 7A CI optimization

## Objective

Shorten wasted pull-request runner time and the time to a cheap failure, without removing a required check, a native test, or the exact-head gate.

## Baseline

Source: pull request 8 head `4404719ddf11ab97c6c0bf5fa47193d45f413426` and merge `6df962208e1c4dee84c1638425ba16471a3dff9e` (tree `3045cdbd98b96306f6038d5b607d48198a9a48b8`).

Final-head workflow durations (created to completed):

| Workflow | Pull request | Post-merge push |
|---|---|---|
| five-target-foundation | 1:48:12 | 1:25:31 |
| matrix verification | 1:48:37 | not a push workflow |
| msrv | 0:32:22 | 0:24:20 |
| pull-request-validation | 0:23:47 | 0:34:35 |
| web-driver | 0:08:04 | 0:07:49 |

Queue time before the Windows job started was about 4 seconds. The final-head critical path is the Windows lane, not runner queue.

Windows job `118546890146` step times, longest first:

| Step | Duration |
|---|---|
| Run eligible foundational tests | 1:27:46 |
| Validate fixtures and architecture | 0:12:26 |
| Verify clean native first launch | 0:03:24 |

Inside that test step, `frozen_quality_matrix_records_every_prediction_and_does_not_hide_failed_gates` in `crates/jarvis-memory/tests/routing_quality_eval.rs` took 3781.54 seconds. The next largest suite in the same step took 480.47 seconds and finished before the long test started. sccache on that Windows job logged `sccache unavailable (status 2)` and compiled directly. Cache permissions stay `contents: read`. This change does not grant `actions: write`.

The same branch had 13 pull-request heads and 65 pull-request workflow runs. Successful final-head workflow time was 281 minutes. Other pull-request runs on that branch accounted for 1853 minutes. Those minutes are workflow wall clock, not a sum of every parallel job. Many of them overlapped a newer head because nothing cancelled them.

## Changes

The five required verification workflows now use one concurrency group per workflow and pull request. A newer run of that same workflow and pull request cancels the older run, including a repeated event for the same head. Push and other non-pull-request events use `run-<run id>`, so two `main` pushes do not share a group. `cancel-in-progress` is true only for `pull_request`.

The exact-head admission check still admits only a completed success for the current head, repository, event, workflow path, and expected jobs. A cancelled or stale run is not success. If a late stale event cancels the current head, the gate waits and then refuses until a current-head success exists.

The documentation and shell-syntax check in `quality` now runs immediately after checkout. A passing run still executes the full workspace verification after it.

Windows tests stay on `--test-threads=1`. The 3781-second routing matrix is unchanged.

Release and RRC workflow files were not given this cancellation group and were not enabled.

## Cancellation check

The first pull-request runs for `d3f0ddc10c28b5e46198f28d85f93ffa2acc4443` were started before this note. The next commit on this pull request is expected to supersede those same-workflow runs. Their conclusions are recorded from GitHub after the fact; a cancelled run is not merge evidence.

## What this does not claim

This does not make the final-head critical path 25 percent shorter. The Windows routing matrix still has to run on the head that merges. Cancellation reduces time spent on superseded heads. One later measurement of this pull request is not a warm-cache study.

## Local checks

`python3 .github/test_ci_concurrency.py` and `python3 .github/test_verify_pr_matrix.py` passed in the Stage 7A worktree. This workstation still has no `dbus-1.pc`, so the native workspace suite was not run here.
