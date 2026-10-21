# Installation scripts

## Purpose

Own offline Jarvis installers and uninstallers, the package builder, and the
read-only release-prerequisite helper. Public registry publication stays a
deferred script in this directory and is not part of Stage 6 installation.

## Ownership

- `release_gate.py` verifies the latest exact-SHA main-push run and its complete
  attempt-specific successful job set for all four release workflows; pagination,
  changed attempts and missing/skipped/running evidence fail closed.
  `test_release_gate.py` supplies offline structured GitHub fixtures.
  It also checks unfiltered push/PR trigger coverage in all four workflows,
  thirteen required context producers and refusal to tag missing evidence.
  Stage 7B reuses this read-only gate for the manually dispatched candidate;
  `test_release_gate.py` additionally guards nonpublishing workflow identity
  and the dormant independent registration descriptor.

- `install.sh` — POSIX entrypoint. It runs `jarvis_package.py` and refuses to
  start without `JARVIS_PACKAGE` and `JARVIS_PACKAGE_SHA256`. It does not
  download a release or a `uv` binary.
- `test_install_windows.ps1` is the offline Windows x86_64 regression: run under
  powershell.exe (Desktop 5.1) and pwsh.exe (7+); tests both invocation forms,
  SHA-256 mismatch/invalid failure, unsupported and simulated WOW64 metadata,
  and installed launcher version checks. A real release/laptop test remains required.
- `windows_package_audit.py` reads normal/delay x86_64 PE imports for all four
  release executables and refuses external VC/CRT runtime dependencies;
  `test_windows_package_audit.py` owns offline parser/refusal regressions.
- `test_install_windows_release.ps1` exercises the exact-candidate shipping
  executables through direct and piped installers in private state under both
  PowerShell hosts; it does not use synthetic hosts or call providers.
  Capture both CLI output streams and native exit codes: ACP metadata uses stderr;
  PowerShell 5.1 NativeCommandError records alone never imply a failed process.
- `install.ps1` — Windows installer for `%LOCALAPPDATA%\Programs\Jarvis`.
  It reads `JARVIS_PACKAGE` and does not call the network.
- `uninstall.sh` and `uninstall.ps1` remove only files recorded in
  `.jarvis-installer-owned.json`. Memory and foreign launchers stay.
- `publish_to_acp_registry.sh` — VRO-11.2 publishing helper for the public
  ACP Registry (`agentclientprotocol/registry`). Clones the registry to a
  temporary directory, writes `registry/agent.json` from this repo into the
  `agent-vesper/` directory, reuses the branch tracked by the open PR, and
  updates that Pull Request in place. CONTINUOUS-UPDATE CONTRACT
  (maintainer-mandated 2026-08-18):
  there is exactly ONE long-lived open `agent-vesper` PR. Never close it to
  open a new one; never comment that it is superseded. For every version
  bump: PUT the updated `agent.json` to the SAME branch the open PR tracks
  (force-push if rebased), then edit the PR title/body in place via
  `gh api repos/agentclientprotocol/registry/pulls/<N> -X PATCH`. The local installers
  (`install.sh` / `install.ps1`) consume `registry/agent.json` from this
  repo, NOT this script. Replaces the deleted `acp_pr_439.md` payload
  (VRO-11.2: that file was stale — it claimed to be the payload for PR
  #439, but PR #439 is the `native-glm-acp` Python project's PR and is
  intentionally left untouched; `agent-vesper` is a separate agent entry).

## Local Contracts

- Stage 2 compiled-target PE audit uses an explicit new four-binary inventory.
  Legacy release-payload audit remains the default. Native installer regression
  maps new build targets to old names in private fixtures only; existing installer
  locations, launchers, download routes and checksum requirements are unchanged.


- Stage 6 installers do not download from a release URL. A missing archive
  or digest fails closed. SHA-256 proves byte match only, not publisher identity.
- Installed launchers invoke their bundled binary verbatim and add no behavior.
- Until the public ACP Registry PR is merged, installer completion guidance
  directs Zed users to register the installed launcher with `type: custom`;
  it must not imply that the agent is already discoverable from the registry.
- Credentials are not stored by the installer. First-run guidance leads with
  the TUI welcome screen and Settings → Providers for adapter-owned sign-in.
  It also documents optional selected-provider `agent-vesper-acp --setup` and
  the optional `ZAI_API_KEY` environment override.
- Uninstallers never remove provider credentials. OS-keyring entries and the
  private-vault fallback are outside the installer-owned artifact set.
- The installers call both binaries with `--version` to confirm success.
  Windows first validates downloaded hosts before replacing payloads or seeding
  user state. A 0xC0000135 loader failure names the missing-DLL class and preserves
  the existing installation; it must never be reported as successful setup.
- Complete packages include the native `sandbox_init` supervisor beside the host
  binaries (inert off Linux), replaced as an installer-owned payload.
- Complete packages include `web-driver/` (archive, checksum, immutable ID).
  Both installers call the bundled ACP binary's `--setup-web-driver` to
  verify/import it. Missing or stopped Docker/Podman leaves the archive
  installed and gives the in-app Set up features / repair route. Native guided
  setup may install an engine after separate confirmation; installers themselves
  do not authorize OS package or VM installation. The flow must not require
  users to search for driver assets. Setup never enables workspace web tools.
  Older packages without the archive receive an explicit upgrade notice.
- POSIX upgrades remove legacy launcher symlinks before writing replacement
  launchers; launcher creation must never follow a symlink into the freshly
  installed bundle and overwrite its binaries.
- Upgrades replace only the named binaries, seed library, and driver payloads,
  never the bundle root. Co-located cognition databases, voice environments,
  and other user state retain their paths and inodes in both installers.
- Skill seeds, when the package contains them, copy into `JARVIS_MEMORY_ROOT`
  or `~/.jarvis/memory`. Existing files win, and slugs already listed in
  `.seed-manifest` are not resurrected. Uninstall does not delete that tree.
  The library includes `work-unit-reporting` (95 skills), which implements
  the completion-reporting behavior both hosts also inject via the shared
  `COMPLETION_REPORTING_INSTRUCTION` system-prompt mandate.

## Verification

- `python3 scripts/test_release_gate.py` verifies the publishing workflow gate.
- `python3 scripts/test_windows_package_audit.py` verifies the PE import audit.
- Pre-tag Windows CI runs the actual release-package audit and real installer
  acceptance under PowerShell 5.1 and 7; publication repeats the staged audit.

- `sh scripts/test_install_upgrade.sh` runs the offline installer suite in a disposable home.
- Shellcheck-clean `install.sh` (POSIX `sh`, no bashisms).
- Shellcheck-clean `uninstall.sh` (POSIX `sh`, no bashisms).
- `install.ps1` runs under Windows PowerShell 5.1+ and PowerShell 7.
- `uninstall.ps1` runs under Windows PowerShell 5.1+ and PowerShell 7.
- After install, both `--version` commands print the workspace version and both
  binaries are reachable on `PATH`.
- Installer completion output identifies custom Zed registration as the
  pre-registry path.

## Child DOX Index

No children.
