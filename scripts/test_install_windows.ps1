# Offline Jarvis installer regression for Windows PowerShell 5.1 and 7+.
$ErrorActionPreference = 'Stop'
$installer = Join-Path $PSScriptRoot 'install.ps1'
$uninstaller = Join-Path $PSScriptRoot 'uninstall.ps1'
$root = Join-Path ([IO.Path]::GetTempPath()) ('jarvis-install-test-' + [guid]::NewGuid().ToString('N'))
# Preserve the registry value and its kind, not just the expanded .NET view.
function Get-UserPathRecord {
    $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey('Environment')
    try {
        if ($null -eq $key -or $key.GetValueNames() -notcontains 'Path') {
            return @{ present = $false; value = $null; kind = $null }
        }
        return @{ present = $true
                  value = [string]$key.GetValue('Path', $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
                  kind = $key.GetValueKind('Path') }
    } finally { if ($null -ne $key) { $key.Dispose() } }
}
function Set-UserPathFixture($Value, $Kind) {
    $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
    try {
        if ($null -eq $Kind) { $key.DeleteValue('Path', $false) }
        else { $key.SetValue('Path', [string]$Value, $Kind) }
    } finally { $key.Dispose() }
}
function Assert-UserPathRecord($Expected, [string]$Description) {
    $actual = Get-UserPathRecord
    if ($actual.present -ne $Expected.present -or
        ($actual.present -and ($actual.value -cne $Expected.value -or $actual.kind -ne $Expected.kind))) {
        throw "user PATH content or registry type changed: $Description"
    }
}
$oldUserPathRecord = Get-UserPathRecord
$oldArch = $env:PROCESSOR_ARCHITECTURE
$oldWow = $env:PROCESSOR_ARCHITEW6432
$oldPath = $env:Path
$oldTls = [Net.ServicePointManager]::SecurityProtocol

function Invoke-WebRequest { throw 'installer attempted a network download' }
function Expect-Failure {
    param([scriptblock]$Run, [string]$Message)
    try { & $Run | Out-Null } catch {
        if ($_.Exception.Message -notlike "*$Message*") { throw "unexpected failure: $_" }
        return
    }
    throw "expected failure: $Message"
}
function New-FixturePackage {
    param([string]$Directory, [string]$Executable)
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $package = Join-Path $Directory 'stage'
    New-Item -ItemType Directory -Path $package -Force | Out-Null
    Copy-Item -LiteralPath $Executable -Destination (Join-Path $package 'jarvis-acp.exe') -Force
    Copy-Item -LiteralPath $Executable -Destination (Join-Path $package 'jarvis-tui.exe') -Force
    $payloads = @()
    foreach ($name in @('jarvis-acp.exe', 'jarvis-tui.exe')) {
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $package $name)).Hash.ToLowerInvariant()
        $payloads += [ordered]@{ path = $name; type = 'file'; role = 'user-facing'; required = $true; sha256 = $hash }
    }
    $manifest = [ordered]@{
        schema = 1; layout = 1; application = 'jarvis'; target = 'windows-x86_64'
        archive = 'jarvis-windows-x86_64.zip'; version = '0.24.14'; source_commit = 'test'
        payloads = $payloads
    }
    $utf8 = New-Object System.Text.UTF8Encoding $false
    [IO.File]::WriteAllText((Join-Path $package 'manifest.json'), ($manifest | ConvertTo-Json -Depth 6), $utf8)
    $archive = Join-Path $Directory 'package.zip'
    if (Test-Path $archive) { Remove-Item $archive -Force }
    $zip = [System.IO.Compression.ZipFile]::Open($archive, 'Create')
    try {
        foreach ($file in Get-ChildItem -LiteralPath $package -File) {
            [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $file.FullName, ('jarvis/' + $file.Name)) | Out-Null
        }
    } finally { $zip.Dispose() }
    return $archive
}

try {
    New-Item -ItemType Directory -Path $root -Force | Out-Null
    $compiler = Join-Path $env:WINDIR 'Microsoft.NET\Framework\v4.0.30319\csc.exe'
    if (-not (Test-Path -LiteralPath $compiler -PathType Leaf)) { throw 'Windows .NET Framework C# compiler unavailable on test runner' }
    $source = Join-Path $root 'fixture.cs'
    $exe = Join-Path $root 'fixture.exe'
    Set-Content -LiteralPath $source -Value 'public class Fixture { public static int Main(string[] args) { if (args.Length == 1 && args[0] == "--version") { System.Console.WriteLine("fixture 0.0.0"); return 0; } return 1; } }'
    & $compiler /nologo "/out:$exe" $source
    if ($LASTEXITCODE -ne 0) { throw 'fixture compilation failed' }
    $archive = New-FixturePackage -Directory $root -Executable $exe
    $sha = (Get-FileHash -Algorithm SHA256 -LiteralPath $archive).Hash
    $env:JARVIS_PACKAGE = $archive
    $env:JARVIS_PACKAGE_SHA256 = $sha
    $env:JARVIS_INSTALL_ROOT = Join-Path $root 'installed'
    $env:JARVIS_MEMORY_ROOT = Join-Path $root 'memory'
    $env:JARVIS_HOME = $root
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'
    Remove-Item Env:PROCESSOR_ARCHITEW6432 -ErrorAction SilentlyContinue
    Remove-Item Env:JARVIS_INSTALL_FAIL_AFTER -ErrorAction SilentlyContinue
    Remove-Item Env:JARVIS_INSTALL_ACTION -ErrorAction SilentlyContinue

    $env:PROCESSOR_ARCHITECTURE = 'ARM64'
    Expect-Failure { & $installer } 'unsupported Windows architecture: ARM64'
    $env:PROCESSOR_ARCHITECTURE = 'x86'
    Expect-Failure { & $installer } 'unsupported Windows architecture: x86'
    Remove-Item Env:PROCESSOR_ARCHITECTURE
    Expect-Failure { & $installer } 'unable to determine Windows architecture'
    $env:PROCESSOR_ARCHITECTURE = 'AMD64'

    $env:JARVIS_PACKAGE_SHA256 = ('0' * 64)
    Expect-Failure { & $installer } 'SHA-256 verification failed'
    if (Test-Path $env:JARVIS_INSTALL_ROOT) { throw 'checksum failure installed payload' }
    $env:JARVIS_PACKAGE_SHA256 = 'garbage'
    Expect-Failure { & $installer } 'invalid SHA-256 checksum file'
    $env:JARVIS_PACKAGE_SHA256 = $sha

    & $installer | Out-Null
    $acp = Join-Path $env:JARVIS_INSTALL_ROOT 'jarvis-acp.cmd'
    $tui = Join-Path $env:JARVIS_INSTALL_ROOT 'jarvis-tui.cmd'
    $acpOut = @(& $acp --version 2>&1 | ForEach-Object { "$_" }) -join '|'
    $tuiOut = @(& $tui --version 2>&1 | ForEach-Object { "$_" }) -join '|'
    $cmd = Get-Content -LiteralPath $tui -Raw
    if ($acpOut -ne 'fixture 0.0.0' -or $tuiOut -ne 'fixture 0.0.0') { throw "launchers failed acp=[$acpOut] tui=[$tuiOut] cmd=[$cmd]" }
    New-Item -ItemType Directory -Path $env:JARVIS_MEMORY_ROOT -Force | Out-Null
    Set-Content -LiteralPath (Join-Path $env:JARVIS_MEMORY_ROOT 'keep.txt') -Value 'keep'
    $vesper = Join-Path $root 'AgentVesper\secret.txt'
    New-Item -ItemType Directory -Path (Split-Path $vesper) -Force | Out-Null
    Set-Content -LiteralPath $vesper -Value 'vesper'

    $installedExe = Join-Path $env:JARVIS_INSTALL_ROOT 'generations'
    $current = (Get-Content -LiteralPath (Join-Path $env:JARVIS_INSTALL_ROOT 'current') -Raw).Trim()
    $installedHash = (Get-FileHash -Algorithm SHA256 (Join-Path $env:JARVIS_INSTALL_ROOT "generations\$current\jarvis-acp.exe")).Hash
    $brokenSource = Join-Path $root 'broken.cs'
    $brokenExe = Join-Path $root 'broken.exe'
    Set-Content -LiteralPath $brokenSource -Value 'public class Broken { public static int Main() { return unchecked((int)0xC0000135); } }'
    & $compiler /nologo "/out:$brokenExe" $brokenSource
    if ($LASTEXITCODE -ne 0) { throw 'broken fixture compilation failed' }
    $brokenArchive = New-FixturePackage -Directory (Join-Path $root 'broken') -Executable $brokenExe
    $env:JARVIS_PACKAGE = $brokenArchive
    $env:JARVIS_PACKAGE_SHA256 = (Get-FileHash -Algorithm SHA256 $brokenArchive).Hash
    Expect-Failure { & $installer } '0xC0000135: required DLL missing'
    $after = (Get-Content -LiteralPath (Join-Path $env:JARVIS_INSTALL_ROOT 'current') -Raw).Trim()
    if ($after -ne $current) { throw 'failed preflight replaced working generation' }
    if ((Get-FileHash -Algorithm SHA256 (Join-Path $env:JARVIS_INSTALL_ROOT "generations\$current\jarvis-acp.exe")).Hash -ne $installedHash) {
        throw 'failed preflight replaced working executable'
    }

    $env:JARVIS_PACKAGE = $archive
    $env:JARVIS_PACKAGE_SHA256 = $sha
    Get-Content -LiteralPath $installer -Raw | Invoke-Expression | Out-Null
    if ((& $acp --version) -ne 'fixture 0.0.0') { throw 'piped install failed' }

    $env:PROCESSOR_ARCHITECTURE = 'x86'
    $env:PROCESSOR_ARCHITEW6432 = 'AMD64'
    & $installer | Out-Null
    if ((& $tui --version) -ne 'fixture 0.0.0') { throw 'WOW64 install failed' }

    $foreign = Join-Path $root 'foreign'
    New-Item -ItemType Directory -Path $foreign | Out-Null
    Set-Content -LiteralPath (Join-Path $foreign 'notes.txt') -Value 'mine'
    $env:JARVIS_INSTALL_ROOT = $foreign
    Expect-Failure { & $installer } 'foreign-owned'
    if ((Get-Content (Join-Path $foreign 'notes.txt')) -ne 'mine') { throw 'foreign root was modified' }
    $env:JARVIS_INSTALL_ROOT = Join-Path $root 'installed'

    & $uninstaller
    if (Test-Path $acp) { throw 'uninstall left a launcher' }
    if ((Get-Content (Join-Path $env:JARVIS_MEMORY_ROOT 'keep.txt')) -ne 'keep') { throw 'uninstall removed memory' }
    if ((Get-Content $vesper) -ne 'vesper') { throw 'uninstall changed simulated Vesper state' }

    # A real top-level window receives HWND_BROADCAST, unlike message-only windows.
    # Observe the Environment payload, not merely the presence of a source call.
    Add-Type -ReferencedAssemblies System.Windows.Forms -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Windows.Forms;
public sealed class JarvisEnvironmentListener : NativeWindow {
    public int Count { get; private set; }
    public JarvisEnvironmentListener() {
        var parameters = new CreateParams();
        parameters.Style = unchecked((int)0x80000000); // top-level WS_POPUP
        CreateHandle(parameters);
    }
    protected override void WndProc(ref Message message) {
        if (message.Msg == 0x1a && message.LParam != IntPtr.Zero &&
            Marshal.PtrToStringUni(message.LParam) == "Environment") Count++;
        base.WndProc(ref message);
    }
    public void Close() { DestroyHandle(); }
}
'@
    $listener = New-Object JarvisEnvironmentListener
    try {
    # Exercise the real installer and uninstaller against controlled HKCU values.
    # Do not echo real or fixture PATH contents into CI logs.
    $ordinary = [Microsoft.Win32.RegistryValueKind]::String
    $expand = [Microsoft.Win32.RegistryValueKind]::ExpandString
    $cases = @(
        @{ name = 'absent'; value = $null; kind = $null; equivalent = $false },
        @{ name = 'empty'; value = ''; kind = $ordinary; equivalent = $false },
        @{ name = 'populated'; value = 'C:\Other;D:\Tools'; kind = $ordinary; equivalent = $false },
        @{ name = 'leading-consecutive-trailing'; value = ';C:\Other;;D:\Tools;'; kind = $ordinary; equivalent = $false },
        @{ name = 'duplicates'; value = 'C:\Other;C:\Other;D:\Tools'; kind = $ordinary; equivalent = $false },
        @{ name = 'expandable'; value = '%USERPROFILE%\bin;;C:\Other;'; kind = $expand; equivalent = $false },
        @{ name = 'pre-existing-equivalent'; value = ('C:\Other;' + $env:JARVIS_INSTALL_ROOT.ToUpperInvariant() + '\;D:\Tools;'); kind = $ordinary; equivalent = $true },
        @{ name = 'duplicate-equivalent'; value = ($env:JARVIS_INSTALL_ROOT + ';C:\Other;' + $env:JARVIS_INSTALL_ROOT.ToUpperInvariant() + '\;'); kind = $ordinary; equivalent = $true }
    )
    foreach ($case in $cases) {
        Set-UserPathFixture $case.value $case.kind
        $initial = Get-UserPathRecord
        $beforeNotice = $listener.Count
        & $installer | Out-Null
        $installed = Get-UserPathRecord
        $expectedNotices = if ($case.equivalent) { 0 } else { 1 }
        if ($listener.Count -ne $beforeNotice + $expectedNotices) {
            throw ('installation Environment notification mismatch: ' + $case.name)
        }
        $ownership = Get-Content -LiteralPath (Join-Path $env:JARVIS_INSTALL_ROOT '.jarvis-installer-owned.json') -Raw | ConvertFrom-Json
        if ($case.equivalent) {
            Assert-UserPathRecord $initial ("install of " + $case.name)
            if ($ownership.path_entry_added -ne $false) { throw 'claimed a pre-existing equivalent PATH entry' }
        } else {
            $prefix = if ($initial.present) { $initial.value } else { '' }
            $separator = if ($prefix -and -not $prefix.EndsWith(';')) { ';' } else { '' }
            $expected = @{ present = $true; value = $prefix + $separator + $env:JARVIS_INSTALL_ROOT; kind = $(if ($initial.present) { $initial.kind } else { $ordinary }) }
            Assert-UserPathRecord $expected ("install of " + $case.name)
            if ($ownership.path_entry_added -ne $true) { throw 'missing owned PATH entry metadata' }
        }
        if ($case.name -eq 'populated') {
            # Upgrades retain original ownership, and a later user-added suffix
            # stays byte-for-byte while only the owned token is removed.
            $beforeUpgrade = $listener.Count
            & $installer | Out-Null
            if ($listener.Count -ne $beforeUpgrade) { throw 'unchanged upgrade broadcast Environment' }
            Assert-UserPathRecord $expected 'upgrade of populated PATH'
            Set-UserPathFixture ($expected.value + ';E:\Added') $expected.kind
            $initial.value += ';E:\Added'
        }
        $beforeUninstall = $listener.Count
        & $uninstaller | Out-Null
        if ($listener.Count -ne $beforeUninstall + $expectedNotices) {
            throw ('uninstallation Environment notification mismatch: ' + $case.name)
        }
        Assert-UserPathRecord $initial ("uninstall of " + $case.name)
        if (Test-Path -LiteralPath $env:JARVIS_INSTALL_ROOT) { throw 'PATH case retained installer-owned root' }
    }
    # A failed staged transaction cannot change HKCU PATH or mask the failure.
    Set-UserPathFixture ';C:\Other;;' $expand
    $beforeFailure = Get-UserPathRecord
    $env:JARVIS_INSTALL_FAIL_AFTER = 'staged'
    $beforeFailureNotice = $listener.Count
    try { Expect-Failure { & $installer } 'injected failure after staging' }
    finally { Remove-Item Env:JARVIS_INSTALL_FAIL_AFTER -ErrorAction SilentlyContinue }
    if ($listener.Count -ne $beforeFailureNotice) { throw 'failed staged install broadcast Environment' }
    Assert-UserPathRecord $beforeFailure 'failed staged installation'
    & $installer | Out-Null
    & $uninstaller | Out-Null
    if ($listener.Count -ne $beforeFailureNotice + 2) { throw 'recovered install/uninstall missed Environment notification' }
    Assert-UserPathRecord $beforeFailure 'recovered failed installation'
    Write-Host 'PASS: Jarvis PowerShell installer offline direct/piped/WOW64, launchers, checksum, ownership, PATH preservation, Environment broadcast and failure recovery'
    } finally { $listener.Close() }
} finally {
    Set-UserPathFixture $oldUserPathRecord.value $oldUserPathRecord.kind
    $env:Path = $oldPath
    $env:PROCESSOR_ARCHITECTURE = $oldArch
    if ($null -eq $oldWow) { Remove-Item Env:PROCESSOR_ARCHITEW6432 -ErrorAction SilentlyContinue } else { $env:PROCESSOR_ARCHITEW6432 = $oldWow }
    [Net.ServicePointManager]::SecurityProtocol = $oldTls
    Remove-Item -LiteralPath $root -Recurse -Force -ErrorAction SilentlyContinue
}
