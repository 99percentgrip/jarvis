# Windows private replacement: focused syscall investigation

## Objective

Investigate Issue #23 without treating the successful rerun as a root-cause explanation. This is a diagnostic regression, not a reliability repair or migration acceptance.

## Source and methods

Based on main `1e01c4edd7bcda876b6b9c72f6888f0782fd39a7`. The recorded first attempt of foundation run `40271802076`, Windows job `121406020467`, failed at the writer's `write_private_atomic(...).unwrap()` with OS error 5; attempt 2 Windows job `121464404044` passed. The line does not expose the specific failing syscall. Source inspection of `crates/jarvis-security/src/private_state.rs` found same-directory staging, `write_all`, `sync_all` and `NamedTempFile::persist`. Inspection of the locally locked tempfile Windows implementation found `SetFileAttributesW` followed by `MoveFileExW(..., MOVEFILE_REPLACE_EXISTING)`. A reader holding the old target without delete sharing can prevent replacement; this is a hypothesis, not a demonstrated cause of the CI failure. Concurrent writers and ACL verification remain alternative suspects.

## Changed file and exact local receipts

`crates/jarvis-security/tests/windows_private_replacement.rs` adds a Windows-only, no-sleep test. It opens the existing target with read/write sharing but no delete sharing, requires the attempted write to return a Windows access/sharing error, checks the original complete record and owner marker, closes the conflicting handle, and verifies that a separate new write publishes the complete replacement. It does not suppress access-denied errors or automatically retry a failed write. This intentionally tests the platform sharing hypothesis, not the unforced original race.

- `CARGO_BUILD_JOBS=2 cargo check --target x86_64-pc-windows-msvc -p jarvis-security --test windows_private_replacement --quiet`: exit 0; compilation only, not native execution.
- `CARGO_BUILD_JOBS=2 cargo test -p jarvis-security --lib --quiet`: `32 passed; 0 failed`; Linux, not native Windows.
- `git diff --check`: exit 0.

## Deviations, unresolved items and readiness

Native Windows execution of the new test was performed by the five-target foundation run `40332026404` attempt 1 on PR #24 head `3beddd429abbf4f8d5ccd15db9d2da1a12b28a5a`: Windows job `121603620822` completed success; its `Run eligible foundational tests` step completed success and invokes `cargo test --workspace --all-features -- --test-threads=1` on Windows. Four other native platform jobs also succeeded. Other workflow runs: pull-request-validation `40332026279`, MSRV `40332026288` and web-driver `40332026841`, all attempt 1 completed success. Exact-head PR verifier `40332026277` attempt 1 failed after waiting for the still-running Windows job, when a fetch of `pulls/24` returned HTTP 422 at `15:11:31Z`; this failure is retained. After the foundation run settled, one `gh run rerun 40332026277 --failed` produced attempt 2 completed success. All 13 required check runs on that unchanged exact PR #24 head then completed successfully. This native test confirms only the deliberately held non-delete-sharing behavior, not the historical failure's actual syscall or its cause. No production correction is proposed until a failing syscall and its ownership implications are established. The historical failure might occur before the rename (including ACL or temporary-file handling); this test alone cannot prove its cause. A native stress/repeated-run harness with specific syscall diagnostics and concurrent readers/writers remains necessary. Issue #23 remains open; no migration, alpha or production readiness is claimed. No user data, protected branch, credentials or release operations were touched.
