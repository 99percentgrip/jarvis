# Stage 7B PR #20 Windows environment notification correction

## Objective

Address the P1 review finding on PR #20: after an owned persistent User PATH change, notify the shell of `Environment` changes without normalizing registry bytes or claiming foreign PATH entries. No merge, candidate dispatch, tag or release is authorized.

## Methods and commands

- Inspected existing `scripts/install.ps1`, `scripts/uninstall.ps1`, `scripts/test_install_windows.ps1`, root/scripts/docs DOX and the open PR #20 review thread `PRRT_kwDOVIpdWM6v_XW3`.
- Added a real top-level Win32-window observer to `scripts/test_install_windows.ps1`; the existing Windows 5.1/PowerShell 7 CI invocation checks `WM_SETTINGCHANGE` with `Environment` exactly once for each owned registry mutation, never for unchanged/equivalent PATH, retained upgrades or staged failure. The original raw PATH/kind, ownership and emergency-restoration cases remain unchanged.
- **Red-first receipt:** test-only fast-forward commit `622021a858ea63bdf44e6e71475baed055cef079` on the existing PR #20 head (parent `ea9b3d93`) failed Windows installer run `40181198564`, job `121132448671`, step `Offline install regression (Windows PowerShell 5.1 Desktop)` with `installation Environment notification mismatch: absent` at test line 212. This confirms that the previous direct-registry implementation emitted no message after actually adding the PATH token. PowerShell 7 did not execute on this red head.
- First corrected PR head `4e6c6ad9ee0aa64f28f474cad2fa30e7e6c5b88d` failed Windows installer job `121133270525` in run `40181423587` before its PATH assertions: PowerShell 7 `Add-Type` required `System.Windows.Forms.Primitives` for the forwarded `Message` type (`CS1069`, test line 169). The existing Windows PowerShell 5.1 regression reached the notification checks; the PS7 observer compilation is now conditioned on PowerShell major version. This failure is retained, not treated as repair evidence.
- Corrective implementation introduces a bounded `SendMessageTimeout(HWND_BROADCAST, WM_SETTINGCHANGE, 0, "Environment", SMTO_ABORTIFHUNG, 5000)` P/Invoke in both standalone PowerShell scripts. Install notifies after its owned `SetValue`; uninstall notifies immediately after each recorded-token `SetValue`/`DeleteValue`, including the legacy exact-suffix fallback. Unchanged user PATH and failed staged installation do not notify. A nonzero Win32 error when the broadcast fails is surfaced rather than silently treated as success.

## Files and evidence

- `scripts/install.ps1`, `scripts/uninstall.ps1`, `scripts/test_install_windows.ps1`, `scripts/AGENTS.md`, this report, and owning docs/PRD/index are scoped to the follow-up. Final corrected SHA, exact-head CI and review-resolution receipts: pending; this report is not yet proof of completion.

## Deviations, unresolved items and readiness

- No completed notification/Windows CI proof yet. PR #20 remains open and Stage 7B native candidate acceptance is separate and unexecuted.
