# Jarvis uninstaller. Removes installer-owned payloads and recorded launchers only.
$ErrorActionPreference = "Stop"
$installRoot = if ($env:JARVIS_INSTALL_ROOT) { $env:JARVIS_INSTALL_ROOT } elseif ($env:JARVIS_INSTALL_DIR) { $env:JARVIS_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "Programs\Jarvis" }
function Fail([string]$Message) { throw "jarvis uninstaller: $Message" }
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
    if ((Get-Content -LiteralPath $path -TotalCount 1) -ne "rem jarvis-launcher") {
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
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath) {
        $kept = @($userPath -split ";" | Where-Object { $_ -and ([string]::Compare($_.TrimEnd("\"), $installRoot.TrimEnd("\"), $true) -ne 0) })
        [Environment]::SetEnvironmentVariable("Path", ($kept -join ";"), "User")
    }
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
