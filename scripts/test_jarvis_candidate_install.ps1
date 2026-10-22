# Native disposable-runner check of the exact already-validated candidate archive.
$ErrorActionPreference = 'Stop'
$root = $env:JARVIS_HOME
if ($env:GITHUB_ACTIONS -ne 'true' -or -not $env:RUNNER_TEMP -or -not $root -or
    -not $env:JARVIS_PACKAGE -or -not $env:JARVIS_PACKAGE_SHA256 -or
    $env:JARVIS_INSTALL_ROOT -ne (Join-Path $root 'installed') -or
    -not ([IO.Path]::GetFullPath($root).StartsWith([IO.Path]::GetFullPath($env:RUNNER_TEMP).TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase))) {
    throw 'candidate installer requires an isolated hosted Windows runner root'
}
$originalUserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$originalProcessPath = $env:Path
try {
    & (Join-Path $PSScriptRoot 'install.ps1')
    $generation = (Get-Content -LiteralPath (Join-Path $env:JARVIS_INSTALL_ROOT 'current') -Raw).Trim()
    $bundle = Join-Path (Join-Path $env:JARVIS_INSTALL_ROOT 'generations') $generation
    foreach ($name in @('jarvis-acp.exe', 'jarvis-tui.exe')) {
        & (Join-Path $bundle $name) --version | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "installed $name version failed: $LASTEXITCODE" }
    }
    & python (Join-Path $PSScriptRoot '../apps/jarvis-tui/tests/first_launch_pty.py') (Join-Path $bundle 'jarvis-tui.exe')
    if ($LASTEXITCODE -ne 0) { throw "installed signed-out first launch failed: $LASTEXITCODE" }
    foreach ($name in @('jarvis-acp.cmd', 'jarvis-tui.cmd')) {
        & (Join-Path $env:JARVIS_INSTALL_ROOT $name) --version | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "installed $name launcher failed: $LASTEXITCODE" }
    }
    & (Join-Path $PSScriptRoot 'uninstall.ps1')
    if (Test-Path -LiteralPath $env:JARVIS_INSTALL_ROOT) {
        throw 'uninstall retained owned install root'
    }
    if ((Get-Content -LiteralPath (Join-Path $root 'unrelated-user-data.txt') -Raw) -ne "preserve`n") {
        throw 'uninstall altered foreign sentinel'
    }
    if ([Environment]::GetEnvironmentVariable('Path', 'User') -ne $originalUserPath) {
        throw 'installer left a changed user PATH'
    }
    Write-Host 'PASS native exact candidate Windows installer and owned cleanup'
} finally {
    # Restore even when an installer or test step fails; never hide the original
    # acceptance failure. Runner is disposable but HKCU PATH must not leak.
    [Environment]::SetEnvironmentVariable('Path', $originalUserPath, 'User')
    $env:Path = $originalProcessPath
}
