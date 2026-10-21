# Stage 7B PR #11 macOS PTY observation repair

## Objective and status

Diagnose the current protected corrective PR #11 head's macOS Intel failure without claiming Stage 7B acceptance. This is a local repair candidate, not an exact-head hosted pass.

## Methods, commands and files

At branch `fix/stage7b-postmerge-windows-settlement`, head `13acb9082b5184a905fda1c62e2c8e7259c485ef`, read `gh pr checks 11` and fetched job `119437025291` using `gh api --allow-escape-sequences repos/99percentgrip/jarvis/actions/jobs/119437025291/logs`. Changed only `apps/jarvis-tui/tests/openai_startup_responsiveness.rs`: the PTY reader now timestamps the first contiguous typed/pasted output and the 1,200 ms assertion checks that timestamp rather than the polling thread's wake-up time. A boundary regression covers 1,199 ms accepted, 1,201 ms rejected, and no observation rejected. No production timeout, startup behavior or acceptance budget was changed.

## Exact evidence

The macOS Intel job log reports `test stalled_openai_discovery_accepts_input_then_applies_catalog ... FAILED` and `typed/pasted input was not visible within 1.2s while discovery was still pending (elapsed 1.218333793s)`. Its diagnostic transcript includes the contiguous `ZQPASTETOKEN` PTY render while the loading status is present; it does not prove whether the frame arrived before the deadline. `cargo test --locked -p jarvis-tui --features integration-test-harness --test openai_startup_responsiveness --quiet` returned `5 passed; 0 failed`; `cargo xtask verify` returned `Acceptance regression gate: 178 exact cases passed in 210176 ms`. `git diff --check` passed. The initial featureless cargo test failed because this target requires `integration-test-harness`; the corrected invocation passed. Initial `cargo fmt --all -- --check` identified formatting differences, followed by `cargo fmt --all`; final `cargo fmt --all -- --check`, `git diff --check` and report-link checks passed.

## Deviations, unresolved items, readiness effect

The prior archival Windows failure at job `119296851037` belongs to a superseded PR #10 head; PR #10 merged at `9592751a`. PR #11 currently has a red macOS Intel job and other pending contexts. Linux-local tests cannot demonstrate that the macOS Intel renderer will meet 1,200 ms or that main-push Windows settlement is healthy. A fresh final-head protected matrix, protected merge, exact-merge push workflows and the disabled candidate-package workflow remain outstanding. No tag, release, registry publication or local installer run is claimed. `.agent/` is unrelated and remains untracked.
