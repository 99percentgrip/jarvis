# Stage 6 offline distribution

## Objective

Give the product its own offline package layout, installer, upgrade, rollback, and uninstall behavior on the five native targets, without publishing a release or reusing Agent Vesper install paths.

## Starting point

- Worktree: `/home/Alex/Projects/jarvis-stage6`
- Branch: `feat/stage6-installers-distribution`
- Base: `b1c45f1def5222e13c03d37d59e88454c6d63d41`
- Tree at that commit: `18417623bed7deb09bdeb2e2ccd254a4b1155a66`
- GitHub `main` matched that commit. PR #7 was merged. All 13 required checks passed. The four post-merge workflows passed (`five-target-foundation` on attempt 2). The `v*` tag ruleset remained active. No other Stage 6 branch existed.
- The dirty pre-existing checkout was not switched or cleaned.

## What changed

- `scripts/jarvis_package.py` packs and installs POSIX packages. `install.sh` and `uninstall.sh` are entrypoints.
- `scripts/install.ps1` and `scripts/uninstall.ps1` are the Windows counterparts. They do not download.
- Archives use the Stage 1 names and a schema-1 manifest. Generations plus `.jarvis-installer-owned.json` separate package files from memory.
- `scripts/test_jarvis_package.py` drives the real POSIX installer in a temporary home.
- Windows fixture and real-binary scripts build `jarvis-windows-x86_64.zip` and call the real PowerShell installer.
- `scripts/test_native_package_install.py` is the five-target CI step. It does not upload artifacts.
- Voice collection now treats stall as an idle wait, so queued progress is not discarded after preemption. The 100 ms and 5 ms test budgets are unchanged.
- `.github/workflows/release.yml` was not enabled. Its retired package names are a Stage 7 replacement, documented in `docs/architecture/stage6-distribution.md`.

## Local evidence

Command:

```text
python3 scripts/test_jarvis_package.py
```

Result:

```text
Installer upgrade preserves user state and database inode: PASS
```

`sh -n` accepted the POSIX entrypoints. `python3 -m py_compile` accepted the package tools.

This machine does not have `dbus-1.pc` (`pkg-config --exists dbus-1` failed). A from-source build was not run here, and system packages were not installed. Header packages are a compile-time requirement, not something the installer tells end users to install. Native package install of real binaries is the five-target CI step, not a local claim.

Windows PowerShell is not installed here, so `install.ps1` was not executed locally. The Windows CI jobs run it.

## Voice investigation

`voice_progress_extends_stall_budget_without_a_whole_recording_deadline` uses a 100 ms stall while the producer sleeps 10 ms between 20 chunks. The old collector checked wall-clock stall before reading the channel. A preempted thread could report "no progress" even when a progress message was already queued. That matches an intermittent macOS failure without proving the exact runner schedule. The fix moves the check to the timeout path. Assertions and the production five-minute message were not weakened. This workstation did not execute the Rust test, because the TUI crate was not built without `dbus-1.pc`. CI remains the native execution.

## Not done in this candidate

- Pull-request head, merge SHA, and the four post-merge workflow conclusions do not exist until GitHub runs them. They are not filled in with guesses.
- Signing, notarization, tag creation, GitHub Releases, RRC, and ACP registry publication are out of scope.
- The container tag `vesper-web-driver:validated` is unchanged so the existing harness and web-driver job stay consistent.

## Stage 7 prerequisites

Replace `.github/workflows/release.yml` before any publication. It still names retired packages and assets, is `workflow_dispatch`-reachable, and must not be treated as current release tooling. Add signing and a real update channel only in that later stage.
