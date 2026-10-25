# Offline Jarvis installer for 64-bit Windows.
# There is no public release download. Set JARVIS_PACKAGE and JARVIS_PACKAGE_SHA256.
$ErrorActionPreference = "Stop"

function Fail([string]$Message) {
    throw "jarvis installer: $Message"
}

if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
    Fail "Windows is required"
}
if (-not [Environment]::Is64BitOperatingSystem) {
    Fail "64-bit Windows is required"
}
$nativeArchitecture = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
if (-not $nativeArchitecture) {
    Fail "unable to determine Windows architecture"
}
if ($nativeArchitecture -ne "AMD64") {
    Fail "unsupported Windows architecture: $nativeArchitecture (only x86_64 packages are supported)"
}

$action = if ($env:JARVIS_INSTALL_ACTION) { $env:JARVIS_INSTALL_ACTION } elseif ($args -contains "--rollback") { "rollback" } else { "install" }
$target = if ($env:JARVIS_TARGET) { $env:JARVIS_TARGET } else { "windows-x86_64" }
if ($target -ne "windows-x86_64") { Fail "wrong target: $target" }

function Test-Foreign([string]$Path) {
    $text = $Path.Replace("\", "/").ToLowerInvariant()
    # Never select the retired Programs\AgentVesper location.
    if ($text.Contains("agent-vesper") -or $text.Contains("agentvesper") -or $text.Contains("programs/agentvesper")) {
        Fail "refusing Vesper-owned or ambiguous path: $Path"
    }
}
function Test-Reparse([string]$Path) {
    if (Test-Path -LiteralPath $Path) {
        $item = Get-Item -LiteralPath $Path -Force
        if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) {
            Fail "refusing symlink redirection: $Path"
        }
    }
}
function Read-JsonFile([string]$Path) {
    Test-Reparse $Path
    $utf8 = New-Object System.Text.UTF8Encoding $false
    return (Get-Content -LiteralPath $Path -Raw -Encoding UTF8 | ConvertFrom-Json)
}
function Write-JsonFile([string]$Path, $Value) {
    Test-Reparse $Path
    $json = $Value | ConvertTo-Json -Compress -Depth 6
    $utf8 = New-Object System.Text.UTF8Encoding $false
    [IO.File]::WriteAllText($Path, $json, $utf8)
}
function Get-ExpectedDigest([string]$Value, [string]$Archive) {
    $text = $Value.Trim()
    if (Test-Path -LiteralPath $text -PathType Leaf) {
        $parts = ((Get-Content -LiteralPath $text -Raw).Trim() -split "\s+")
        if ($parts.Count -lt 1 -or [string]::IsNullOrWhiteSpace($parts[0])) { Fail "invalid SHA-256 checksum file" }
        if ($parts.Count -gt 1 -and $parts[1].TrimStart("*") -ne [IO.Path]::GetFileName($Archive)) {
            Fail "checksum filename does not match the archive"
        }
        $text = $parts[0]
    }
    if ($text -notmatch '^[a-fA-F0-9]{64}$') { Fail "invalid SHA-256 checksum file" }
    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $Archive).Hash
    if ($actual.ToUpperInvariant() -ne $text.ToUpperInvariant()) { Fail "SHA-256 verification failed" }
    return $actual.ToLowerInvariant()
}
function Test-SafeName([string]$Name) {
    if ([string]::IsNullOrWhiteSpace($Name) -or $Name.StartsWith("/") -or $Name.StartsWith("\") -or $Name.Contains(":") -or $Name.Contains("\")) {
        Fail "unsafe archive path: $Name"
    }
    foreach ($part in ($Name -split "/")) {
        if ($part -eq "" -or $part -eq "." -or $part -eq "..") { Fail "unsafe archive path: $Name" }
    }
}
function Test-Allowed([string]$Relative) {
    $allowed = @("jarvis-acp.exe", "jarvis-tui.exe", "jarvis-web-fetch.exe", "jarvis-sandbox-init.exe")
    if ($Relative -notmatch "/") {
        if ($allowed -notcontains $Relative) { Fail "unexpected package file: $Relative" }
        return
    }
    if ($Relative.StartsWith("skills/")) { Test-SafeName $Relative.Substring(7); return }
    if ($Relative.StartsWith("web-driver/")) {
        $rest = $Relative.Substring(11)
        if (@("image.tar.gz", "image-id", "image.sha256") -notcontains $rest) { Fail "unexpected driver file: $Relative" }
        return
    }
    Fail "unexpected package file: $Relative"
}

$homeRoot = if ($env:JARVIS_HOME) { $env:JARVIS_HOME } else { $env:USERPROFILE }
$installRoot = if ($env:JARVIS_INSTALL_ROOT) { $env:JARVIS_INSTALL_ROOT } else { Join-Path $env:LOCALAPPDATA "Programs\Jarvis" }
$memoryRoot = if ($env:JARVIS_MEMORY_ROOT) { $env:JARVIS_MEMORY_ROOT } else { Join-Path $homeRoot ".jarvis\memory" }
Test-Foreign $installRoot
Test-Foreign $memoryRoot
Test-Reparse $installRoot

$known = @(".jarvis-installer-owned.json", ".install.lock", "transaction.json", "current", "generations", "jarvis-acp.cmd", "jarvis-tui.cmd")
function Assert-Adoptable {
    if (-not (Test-Path -LiteralPath $installRoot)) { return }
    Test-Reparse $installRoot
    $unknown = @(Get-ChildItem -LiteralPath $installRoot -Force | Where-Object { $known -notcontains $_.Name })
    if ($unknown.Count -gt 0) { Fail "refusing foreign-owned or ambiguous install root" }
    $ownershipPath = Join-Path $installRoot ".jarvis-installer-owned.json"
    if (Test-Path -LiteralPath $ownershipPath) {
        $record = Read-JsonFile $ownershipPath
        if ($record.application -ne "jarvis" -or $record.schema -ne 1) { Fail "refusing foreign-owned or ambiguous install root" }
    }
}
function Acquire-Lock {
    New-Item -ItemType Directory -Path $installRoot -Force | Out-Null
    Test-Reparse $installRoot
    $lock = Join-Path $installRoot ".install.lock"
    while ($true) {
        try {
            New-Item -ItemType Directory -Path $lock -ErrorAction Stop | Out-Null
            Set-Content -LiteralPath (Join-Path $lock "pid") -Value $PID -Encoding ascii
            return
        } catch {
            $pidFile = Join-Path $lock "pid"
            $owner = 0
            if (Test-Path -LiteralPath $pidFile) { [void][int]::TryParse((Get-Content -LiteralPath $pidFile -Raw).Trim(), [ref]$owner) }
            $alive = $false
            if ($owner -gt 0) { $alive = $null -ne (Get-Process -Id $owner -ErrorAction SilentlyContinue) }
            if ($alive -and $owner -ne $PID) { Fail "concurrent installation holds this install root" }
            Remove-Item -LiteralPath $lock -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}
function Release-Lock {
    $lock = Join-Path $installRoot ".install.lock"
    $pidFile = Join-Path $lock "pid"
    if ((Test-Path -LiteralPath $pidFile) -and ((Get-Content -LiteralPath $pidFile -Raw).Trim() -eq "$PID")) {
        Remove-Item -LiteralPath $lock -Recurse -Force -ErrorAction SilentlyContinue
    }
}
function Expand-SafeZip([string]$Archive, [string]$Destination) {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [System.IO.Compression.ZipFile]::OpenRead($Archive)
    try {
        $seen = @{}
        foreach ($entry in $zip.Entries) {
            $name = $entry.FullName.Replace("\", "/")
            if ($name.EndsWith("/")) {
                if (-not $name.StartsWith("jarvis/")) { Fail "unexpected bundle root: $name" }
                $dir = $name.Substring(7).TrimEnd("/")
                if ($dir) { Test-SafeName $dir }
                continue
            }
            if (-not $name.StartsWith("jarvis/")) { Fail "unexpected bundle root: $name" }
            $relative = $name.Substring(7)
            Test-SafeName $relative
            $key = $relative.ToLowerInvariant()
            if ($seen.ContainsKey($key)) { Fail "case-insensitive path collision: $relative" }
            $seen[$key] = $true
            if ($entry.Length -gt 2GB) { Fail "payload exceeds size limit: $relative" }
            if ($relative -ne "manifest.json") { Test-Allowed $relative }
            $output = Join-Path $Destination ($relative.Replace("/", [IO.Path]::DirectorySeparatorChar))
            $parent = Split-Path -Parent $output
            New-Item -ItemType Directory -Path $parent -Force | Out-Null
            $stream = $entry.Open()
            try {
                $file = [IO.File]::Open($output, [IO.FileMode]::CreateNew)
                try { $stream.CopyTo($file) } finally { $file.Dispose() }
            } finally { $stream.Dispose() }
        }
    } finally { $zip.Dispose() }
}
function Test-Manifest([string]$Root) {
    $path = Join-Path $Root "manifest.json"
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { Fail "malformed package manifest" }
    $manifest = Read-JsonFile $path
    foreach ($key in @("schema", "layout", "application", "target", "archive", "version", "source_commit", "payloads")) {
        if ($null -eq $manifest.$key -and $key -ne "source_commit") { Fail "malformed package manifest" }
    }
    if ($manifest.schema -ne 1 -or $manifest.layout -ne 1 -or $manifest.application -ne "jarvis") { Fail "malformed package manifest" }
    if ($manifest.target -ne "windows-x86_64" -or $manifest.archive -ne "jarvis-windows-x86_64.zip") { Fail "wrong target: $($manifest.target)" }
    $required = @("jarvis-acp.exe", "jarvis-tui.exe")
    $seen = @{}
    foreach ($item in @($manifest.payloads)) {
        $relative = [string]$item.path
        Test-Allowed $relative
        $key = $relative.ToLowerInvariant()
        if ($seen.ContainsKey($key)) { Fail "case-insensitive path collision: $relative" }
        $seen[$key] = $true
        $file = Join-Path $Root ($relative.Replace("/", [IO.Path]::DirectorySeparatorChar))
        if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { Fail "missing payload: $relative" }
        $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $file).Hash
        if ($actual.ToUpperInvariant() -ne ([string]$item.sha256).ToUpperInvariant()) { Fail "payload checksum mismatch: $relative" }
    }
    foreach ($name in $required) { if (-not $seen.ContainsKey($name)) { Fail "missing executable: $name" } }
}
function Invoke-Smoke([string]$Bundle) {
    $bundle = $Bundle
    $preference = $ErrorActionPreference
    try {
        $ErrorActionPreference = "Continue"
        foreach ($name in @("jarvis-acp.exe", "jarvis-tui.exe")) {
            $exe = Join-Path $bundle $name
            $output = @(& $exe --version 2>&1 | ForEach-Object { "$_" })
            $code = $LASTEXITCODE
            if ($code -eq -1073741515) {
                Fail "downloaded $name cannot start (0xC0000135: required DLL missing). Existing installation was preserved."
            }
            if ($code -ne 0 -or -not $output) {
                Fail "downloaded $name version preflight failed (exit $code). Existing installation was preserved."
            }
        }
    } finally { $ErrorActionPreference = $preference }
}
function Write-Launchers([string]$Generation) {
    foreach ($pair in @(@("jarvis-acp","jarvis-acp.exe"), @("jarvis-tui","jarvis-tui.exe"))) {
        $path = Join-Path $installRoot ($pair[0] + ".cmd")
        if (Test-Path -LiteralPath $path) {
            $item = Get-Item -LiteralPath $path -Force
            if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { Remove-Item -LiteralPath $path -Force }
        }
        if (Test-Path -LiteralPath $path -PathType Leaf) {
            $first = @(Get-Content -LiteralPath $path -TotalCount 1)
            $head = @(Get-Content -LiteralPath $path -TotalCount 2)
            if ($head.Count -lt 2 -or $head[0] -ne "@echo off" -or $head[1] -ne "rem jarvis-launcher") {
                Fail "refusing to replace unowned launcher: $path"
            }
        }
        $exePath = Join-Path $installRoot "generations\$Generation\$($pair[1])"
        $body = "@echo off`r`nrem jarvis-launcher`r`n`"$exePath`" %*`r`n"
        $utf8 = New-Object System.Text.UTF8Encoding $false
        [IO.File]::WriteAllText($path, $body, $utf8)
    }
}
function Update-Seed([string]$Generation) {
    $seedRoot = Join-Path $installRoot "generations\$Generation\skills\skills"
    if (-not (Test-Path -LiteralPath $seedRoot -PathType Container)) { return 0 }
    Test-Foreign $memoryRoot
    New-Item -ItemType Directory -Path (Join-Path $memoryRoot "skills"), (Join-Path $memoryRoot "bundles") -Force | Out-Null
    $manifest = Join-Path $memoryRoot ".seed-manifest"
    if (-not (Test-Path -LiteralPath $manifest)) { Set-Content -LiteralPath $manifest -Value "" -Encoding ascii }
    $listed = @(Get-Content -LiteralPath $manifest -ErrorAction SilentlyContinue)
    $count = 0
    Get-ChildItem -LiteralPath $seedRoot -Filter "*.md" -File | ForEach-Object {
        if ($listed -contains $_.BaseName) { return }
        $dest = Join-Path $memoryRoot "skills\$($_.Name)"
        if (Test-Path -LiteralPath $dest) { return }
        Copy-Item -LiteralPath $_.FullName -Destination $dest
        Add-Content -LiteralPath $manifest -Value $_.BaseName
        $script:count++
    }
    return $count
}
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
function Complete-Generation([string]$Generation, $Previous) {
    Write-Launchers $Generation
    $seeded = Update-Seed $Generation
    $ownedPath = Join-Path $installRoot ".jarvis-installer-owned.json"
    $keptPathOwnership = $false
    if (Test-Path -LiteralPath $ownedPath) {
        $prior = Read-JsonFile $ownedPath
        if ($prior.path_entry_added -eq $true) { $keptPathOwnership = $true }
    }
    $ownership = [ordered]@{
        schema = 1
        application = "jarvis"
        install_root = $installRoot
        current_generation = $Generation
        previous_generation = $Previous
        path_entry_added = $keptPathOwnership
        path_value_present = $(if ($keptPathOwnership) { $prior.path_value_present } else { $null })
        path_prefix_length = $(if ($keptPathOwnership) { $prior.path_prefix_length } else { $null })
        path_separator = $(if ($keptPathOwnership) { $prior.path_separator } else { $null })
        memory_root_recorded_not_owned = $memoryRoot
    }
    # Read the unexpanded registry value. Splitting/rejoining PATH discards empty
    # fields and trailing semicolons; .NET's User setter also loses its value kind.
    $pathKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
    try {
        $present = $pathKey.GetValueNames() -contains 'Path'
        $userPath = if ($present) { [string]$pathKey.GetValue('Path', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames) } else { '' }
        $kind = if ($present) { $pathKey.GetValueKind('Path') } else { [Microsoft.Win32.RegistryValueKind]::String }
        if ($kind -ne [Microsoft.Win32.RegistryValueKind]::String -and $kind -ne [Microsoft.Win32.RegistryValueKind]::ExpandString) {
            Fail 'unsupported User PATH registry value kind'
        }
        $already = $false
        foreach ($entry in ($userPath -split ';')) {
            if ($entry -and [string]::Compare($entry.TrimEnd('\'), $installRoot.TrimEnd('\'), $true) -eq 0) { $already = $true }
        }
        if (-not $already) {
            $separator = if ($userPath -and -not $userPath.EndsWith(';')) { ';' } else { '' }
            $pathKey.SetValue('Path', ($userPath + $separator + $installRoot), $kind)
            Send-EnvironmentChange
            $ownership.path_entry_added = $true
            $ownership.path_value_present = $present
            $ownership.path_prefix_length = $userPath.Length
            $ownership.path_separator = $separator
        }
    } finally { $pathKey.Dispose() }
    $generations = Join-Path $installRoot "generations"
    if (Test-Path -LiteralPath $generations) {
        $keep = @($Generation)
        if ($Previous) { $keep += [string]$Previous }
        Get-ChildItem -LiteralPath $generations -Force | ForEach-Object {
            if ($keep -notcontains $_.Name) {
                Remove-Item -LiteralPath $_.FullName -Recurse -Force
            }
        }
    }
    if (($env:Path -split ";") -notcontains $installRoot) { $env:Path = "$installRoot;$env:Path" }
    Write-JsonFile (Join-Path $installRoot ".jarvis-installer-owned.json") $ownership
    Write-JsonFile (Join-Path $installRoot "transaction.json") ([ordered]@{ state = "complete"; generation = $Generation; previous = $Previous })
    Write-Host "Seeded $seeded skill(s) into $memoryRoot"
    Write-Host "Installed Jarvis launchers under $installRoot"
    Write-Host "No public release was downloaded."
}

if ($action -eq "rollback") {
    Assert-Adoptable
    Acquire-Lock
    try {
        $transactionPath = Join-Path $installRoot "transaction.json"
        if (Test-Path -LiteralPath $transactionPath) {
            $transaction = Read-JsonFile $transactionPath
            if ($transaction.state -ne "complete") { Fail "installation transaction is incomplete; rerun install before rollback" }
        }
        $ownership = Read-JsonFile (Join-Path $installRoot ".jarvis-installer-owned.json")
        if (-not $ownership.previous_generation) { Fail "no verified previous Jarvis package is available to restore" }
        $restored = [string]$ownership.previous_generation
        Write-Launchers $restored
        $utf8 = New-Object System.Text.UTF8Encoding $false
        [IO.File]::WriteAllText((Join-Path $installRoot "current"), ($restored + "`n"), $utf8)
        $ownership.previous_generation = $ownership.current_generation
        $ownership.current_generation = $restored
        Write-JsonFile (Join-Path $installRoot ".jarvis-installer-owned.json") $ownership
        Write-Host "Restored local Jarvis package $($ownership.current_generation)"
    } finally { Release-Lock }
    return
}

if (-not $env:JARVIS_PACKAGE -or -not $env:JARVIS_PACKAGE_SHA256) {
    Fail "no public package is available. Set JARVIS_PACKAGE to a local archive and JARVIS_PACKAGE_SHA256 to its expected digest."
}
if (-not (Test-Path -LiteralPath $env:JARVIS_PACKAGE -PathType Leaf)) { Fail "package archive is missing" }
Get-ExpectedDigest $env:JARVIS_PACKAGE_SHA256 $env:JARVIS_PACKAGE | Out-Null
$temporary = Join-Path ([IO.Path]::GetTempPath()) ("jarvis-stage-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $temporary | Out-Null
try {
    Expand-SafeZip $env:JARVIS_PACKAGE $temporary
    Test-Manifest $temporary
    Invoke-Smoke $temporary
    Assert-Adoptable
    Acquire-Lock
    try {
        $generation = "g" + [guid]::NewGuid().ToString("N").Substring(0, 16)
        $previous = $null
        $currentPath = Join-Path $installRoot "current"
        if (Test-Path -LiteralPath $currentPath -PathType Leaf) { $previous = (Get-Content -LiteralPath $currentPath -Raw).Trim() }
        $destination = Join-Path $installRoot "generations\$generation"
        New-Item -ItemType Directory -Path $destination -Force | Out-Null
        Copy-Item -Path (Join-Path $temporary "*") -Destination $destination -Recurse -Force
        Write-JsonFile (Join-Path $installRoot "transaction.json") ([ordered]@{ state = "staged"; generation = $generation; previous = $previous })
        if ($env:JARVIS_INSTALL_FAIL_AFTER -eq "staged") { Fail "injected failure after staging; installation was not activated" }
        $utf8 = New-Object System.Text.UTF8Encoding $false
        [IO.File]::WriteAllText($currentPath, ($generation + "`n"), $utf8)
        Write-JsonFile (Join-Path $installRoot "transaction.json") ([ordered]@{ state = "activated"; generation = $generation; previous = $previous })
        if ($env:JARVIS_INSTALL_FAIL_AFTER -eq "activated") { Fail "injected failure after activation; rerun to finish launchers" }
        Complete-Generation $generation $previous
    } finally { Release-Lock }
} finally {
    Remove-Item -LiteralPath $temporary -Recurse -Force -ErrorAction SilentlyContinue
}
