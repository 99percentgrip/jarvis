# Native disposable-runner check of the exact already-validated candidate archive.
$ErrorActionPreference = 'Stop'
$root = $env:JARVIS_HOME
if ($env:GITHUB_ACTIONS -ne 'true' -or -not $env:RUNNER_TEMP -or -not $root -or
    -not $env:JARVIS_PACKAGE -or -not $env:JARVIS_PACKAGE_SHA256 -or
    $env:JARVIS_INSTALL_ROOT -ne (Join-Path $root 'installed') -or
    -not ([IO.Path]::GetFullPath($root).StartsWith([IO.Path]::GetFullPath($env:RUNNER_TEMP).TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase))) {
    throw 'candidate installer requires an isolated hosted Windows runner root'
}
. (Join-Path $PSScriptRoot 'jarvis_candidate_sentinel.ps1')
$sentinel = Join-Path $root 'unrelated-user-data.txt'
Assert-JarvisCandidateSentinel -Path $sentinel -Message 'invalid initial sentinel fixture'
$pathKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
try {
    $originalPathPresent = $pathKey.GetValueNames() -contains 'Path'
    $originalUserPath = if ($originalPathPresent) { [string]$pathKey.GetValue('Path', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames) } else { $null }
    $originalPathKind = if ($originalPathPresent) { $pathKey.GetValueKind('Path') } else { $null }
} finally { $pathKey.Dispose() }
$originalProcessPath = $env:Path
$originalFailure = $null
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
    Assert-JarvisCandidateSentinel -Path $sentinel -Message 'uninstall altered foreign sentinel'
    $pathKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
    try {
        $present = $pathKey.GetValueNames() -contains 'Path'
        if ($present -ne $originalPathPresent -or ($present -and
            ([string]$pathKey.GetValue('Path', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames) -cne $originalUserPath -or
             $pathKey.GetValueKind('Path') -ne $originalPathKind))) {
            throw 'installer left a changed user PATH'
        }
    } finally { $pathKey.Dispose() }
    Write-Host 'PASS native exact candidate Windows installer and owned cleanup'
} catch {
    $originalFailure = $_
    throw
} finally {
    # Restore the raw HKCU value and registry kind without masking the original
    # acceptance failure if emergency restoration itself fails.
    try {
        $pathKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
        try {
            if ($originalPathPresent) { $pathKey.SetValue('Path', $originalUserPath, $originalPathKind) }
            else { $pathKey.DeleteValue('Path', $false) }
        } finally { $pathKey.Dispose() }
    } catch {
        if ($null -eq $originalFailure) { throw }
        Write-Warning 'emergency user PATH restoration failed after the original candidate failure'
    } finally { $env:Path = $originalProcessPath }
}
