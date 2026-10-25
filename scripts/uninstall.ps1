# Jarvis uninstaller. Removes installer-owned payloads and recorded launchers only.
$ErrorActionPreference = "Stop"
$installRoot = if ($env:JARVIS_INSTALL_ROOT) { $env:JARVIS_INSTALL_ROOT } else { Join-Path $env:LOCALAPPDATA "Programs\Jarvis" }
function Fail([string]$Message) { throw "jarvis uninstaller: $Message" }
function Send-EnvironmentChange {
    # Persistent registry writes bypass the .NET User setter's shell notification.
    if (-not ('JarvisEnvironmentBroadcast' -as [type])) {
        Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class JarvisEnvironmentBroadcast {
    [DllImport("user32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    public static extern IntPtr SendMessageTimeout(IntPtr window, uint message,
        IntPtr wParam, string lParam, uint flags, uint timeout, out IntPtr result);
}
'@
    }
    $result = [IntPtr]::Zero
    $sent = [JarvisEnvironmentBroadcast]::SendMessageTimeout([IntPtr]0xffff, 0x1a,
        [IntPtr]::Zero, 'Environment', 0x2, 5000, [ref]$result)
    if ($sent -eq [IntPtr]::Zero -and [Runtime.InteropServices.Marshal]::GetLastWin32Error() -ne 0) {
        Fail 'unable to notify Windows of the User PATH change'
    }
}
$text = $installRoot.Replace("\", "/").ToLowerInvariant()
if ($text.Contains("agent-vesper") -or $text.Contains("agentvesper")) { Fail "refusing Vesper-owned or ambiguous path: $installRoot" }
if (-not (Test-Path -LiteralPath $installRoot)) {
    Write-Host "Jarvis uninstall found no installer-owned root."
    return
}
$item = Get-Item -LiteralPath $installRoot -Force
if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { Fail "refusing symlink redirection: $installRoot" }
$ownershipPath = Join-Path $installRoot ".jarvis-installer-owned.json"
if (-not (Test-Path -LiteralPath $ownershipPath -PathType Leaf)) { Fail "refusing to uninstall a root this installer does not own" }
$ownership = Get-Content -LiteralPath $ownershipPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($ownership.application -ne "jarvis") { Fail "refusing to uninstall a root this installer does not own" }
foreach ($name in @("jarvis-acp.cmd", "jarvis-tui.cmd")) {
    $path = Join-Path $installRoot $name
    if (-not (Test-Path -LiteralPath $path)) { continue }
    $launcher = Get-Item -LiteralPath $path -Force
    if ($launcher.Attributes -band [IO.FileAttributes]::ReparsePoint) {
        Remove-Item -LiteralPath $path -Force
        continue
    }
    $head = @(Get-Content -LiteralPath $path -TotalCount 2)
    if ($head.Count -lt 2 -or $head[0] -ne "@echo off" -or $head[1] -ne "rem jarvis-launcher") {
        Fail "refusing to remove unowned launcher: $path"
    }
    Remove-Item -LiteralPath $path -Force
}
foreach ($generation in @($ownership.current_generation, $ownership.previous_generation)) {
    if (-not $generation) { continue }
    if ($generation -match '[\\/]|\\.\\.') { Fail "invalid generation name" }
    $path = Join-Path $installRoot "generations\$generation"
    if (Test-Path -LiteralPath $path -PathType Container) {
        $gen = Get-Item -LiteralPath $path -Force
        if ($gen.Attributes -band [IO.FileAttributes]::ReparsePoint) { Fail "refusing symlink redirection: $path" }
        Remove-Item -LiteralPath $path -Recurse -Force
    }
}
if ($ownership.path_entry_added -eq $true) {
    $pathKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
    try {
        if ($pathKey.GetValueNames() -contains 'Path') {
            $userPath = [string]$pathKey.GetValue('Path', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
            $kind = $pathKey.GetValueKind('Path')
            if ($kind -ne [Microsoft.Win32.RegistryValueKind]::String -and $kind -ne [Microsoft.Win32.RegistryValueKind]::ExpandString) {
                Fail 'unsupported User PATH registry value kind'
            }
            # A new ownership record identifies the exact inserted token's offset.
            # Never filter every equivalent entry: another entry may be user-owned.
            $offset = $ownership.path_prefix_length -as [int]
            $separator = [string]$ownership.path_separator
            $hasOffset = $null -ne $ownership.path_prefix_length -and $null -ne $offset -and
                $offset -ge 0 -and $separator -in @('', ';')
            if ($hasOffset) {
                $owned = $separator + $installRoot
                if ($offset + $owned.Length -le $userPath.Length -and
                    $userPath.Substring($offset, $owned.Length) -ceq $owned -and
                    ($offset + $owned.Length -eq $userPath.Length -or $userPath[$offset + $owned.Length] -eq ';')) {
                    $kept = $userPath.Remove($offset, $owned.Length)
                    if (-not $ownership.path_value_present -and $kept.Length -eq 0) {
                        $pathKey.DeleteValue('Path', $false)
                    } else {
                        $pathKey.SetValue('Path', $kept, $kind)
                    }
                    Send-EnvironmentChange
                }
            } else {
                # Compatibility with prior ownership records: remove only an exact
                # trailing installer token, never normalize other PATH entries.
                $suffix = ';' + $installRoot
                if ($userPath.EndsWith($suffix, [StringComparison]::Ordinal)) {
                    $pathKey.SetValue('Path', $userPath.Substring(0, $userPath.Length - $suffix.Length), $kind)
                    Send-EnvironmentChange
                } elseif ($userPath -ceq $installRoot) {
                    $pathKey.DeleteValue('Path', $false)
                    Send-EnvironmentChange
                }
            }
        }
    } finally { $pathKey.Dispose() }
}
foreach ($name in @("current", "transaction.json", ".jarvis-installer-owned.json")) {
    $path = Join-Path $installRoot $name
    if (Test-Path -LiteralPath $path) { Remove-Item -LiteralPath $path -Force }
}
$generations = Join-Path $installRoot "generations"
if ((Test-Path -LiteralPath $generations -PathType Container) -and -not (Get-ChildItem -LiteralPath $generations -Force)) {
    Remove-Item -LiteralPath $generations -Force
}
$remaining = @(Get-ChildItem -LiteralPath $installRoot -Force | Where-Object { $_.Name -ne ".install.lock" })
if ($remaining.Count -eq 0) {
    Remove-Item -LiteralPath $installRoot -Recurse -Force
}
Write-Host "Jarvis uninstall removed installer-owned payloads only."
Write-Host "Configuration, credentials, sessions, and memory were preserved."
