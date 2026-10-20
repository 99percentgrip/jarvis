param([Parameter(Mandatory = $true)][string]$BinaryDir)
$ErrorActionPreference = 'Stop'
$root = Join-Path ([IO.Path]::GetTempPath()) ('jarvis-real-package-' + [guid]::NewGuid().ToString('N'))
$oldUserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$oldPath = $env:Path
try {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $package = Join-Path $root 'stage'
    New-Item -ItemType Directory -Path $package -Force | Out-Null
    $payloads = @()
    foreach ($name in @('jarvis-acp.exe', 'jarvis-tui.exe', 'jarvis-web-fetch.exe', 'jarvis-sandbox-init.exe')) {
        $source = Join-Path $BinaryDir $name
        if (-not (Test-Path -LiteralPath $source -PathType Leaf)) { throw "missing built executable: $source" }
        Copy-Item -LiteralPath $source -Destination (Join-Path $package $name)
        $role = if ($name -like 'jarvis-acp.exe' -or $name -like 'jarvis-tui.exe') { 'user-facing' } else { 'helper' }
        $required = $role -eq 'user-facing'
        $hash = (Get-FileHash -Algorithm SHA256 (Join-Path $package $name)).Hash.ToLowerInvariant()
        $payloads += [ordered]@{ path = $name; type = 'file'; role = $role; required = $required; sha256 = $hash }
    }
    $manifest = [ordered]@{
        schema = 1; layout = 1; application = 'jarvis'; target = 'windows-x86_64'
        archive = 'jarvis-windows-x86_64.zip'; version = '0.24.14'; source_commit = 'candidate'
        payloads = $payloads
    }
    $utf8 = New-Object System.Text.UTF8Encoding $false
    [IO.File]::WriteAllText((Join-Path $package 'manifest.json'), ($manifest | ConvertTo-Json -Depth 6), $utf8)
    $archive = Join-Path $root 'jarvis-windows-x86_64.zip'
    $zip = [System.IO.Compression.ZipFile]::Open($archive, 'Create')
    try {
        foreach ($file in Get-ChildItem -LiteralPath $package -File) {
            [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $file.FullName, ('jarvis/' + $file.Name)) | Out-Null
        }
    } finally { $zip.Dispose() }
    $env:JARVIS_PACKAGE = $archive
    $env:JARVIS_PACKAGE_SHA256 = (Get-FileHash -Algorithm SHA256 $archive).Hash
    $env:JARVIS_INSTALL_ROOT = Join-Path $root 'installed'
    $env:JARVIS_MEMORY_ROOT = Join-Path $root 'memory'
    $env:JARVIS_HOME = $root
    New-Item -ItemType Directory -Path $env:JARVIS_MEMORY_ROOT -Force | Out-Null
    Set-Content -LiteralPath (Join-Path $env:JARVIS_MEMORY_ROOT 'keep.txt') -Value 'keep'
    $installer = Join-Path $PSScriptRoot 'install.ps1'
    & $installer | Out-Null
    Get-Content -LiteralPath $installer -Raw | Invoke-Expression | Out-Null
    foreach ($hostName in @('jarvis-acp', 'jarvis-tui')) {
        $launcher = Join-Path $env:JARVIS_INSTALL_ROOT "$hostName.cmd"
        $checkPreference = $ErrorActionPreference
        try {
            $ErrorActionPreference = 'Continue'
            $observedVersion = @(& $launcher --version 2>&1 | ForEach-Object { "$_" })
            $versionExit = $LASTEXITCODE
            & $launcher --help 2>&1 | Out-Null
            $helpExit = $LASTEXITCODE
        } finally { $ErrorActionPreference = $checkPreference }
        if ($versionExit -ne 0 -or -not $observedVersion) { throw "$hostName --version failed (exit $versionExit)" }
        if ($helpExit -ne 0) { throw "$hostName --help failed (exit $helpExit)" }
        Write-Host "PASS real exact-candidate Windows package: $observedVersion"
    }
    if ((Get-Content (Join-Path $env:JARVIS_MEMORY_ROOT 'keep.txt')) -ne 'keep') { throw 'real install changed memory sentinel' }
} finally {
    [Environment]::SetEnvironmentVariable('Path', $oldUserPath, 'User')
    $env:Path = $oldPath
    Remove-Item -LiteralPath $root -Recurse -Force -ErrorAction SilentlyContinue
}
