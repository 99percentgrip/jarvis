# Exercise the exact guard called by the native candidate Windows acceptance path.
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'jarvis_candidate_sentinel.ps1')
$root = Join-Path ([IO.Path]::GetTempPath()) ('jarvis-sentinel-regression-' + [guid]::NewGuid().ToString('N'))
[IO.Directory]::CreateDirectory($root) | Out-Null
$sentinel = Join-Path $root 'foreign.txt'
$canonical = [byte[]](0x70, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x0a)
function Expect-SentinelFailure([string]$label) {
    $rejected = $false
    try {
        Assert-JarvisCandidateSentinel -Path $sentinel -Message $label
    } catch {
        if ($_.Exception.Message -notlike "*$label*") { throw }
        $rejected = $true
    }
    if (-not $rejected) { throw "guard accepted $label" }
    Write-Host "PASS rejects $label"
}
try {
    [IO.File]::WriteAllBytes($sentinel, $canonical)
    Assert-JarvisCandidateSentinel -Path $sentinel -Message 'invalid initial sentinel fixture'
    Write-Host 'PASS canonical LF before install'
    # No installation/uninstallation needed to reproduce the old comparison.
    & python -c "import pathlib,sys; pathlib.Path(sys.argv[1]).write_text('preserve\n', encoding='utf-8')" $sentinel
    if ($LASTEXITCODE -ne 0) { throw 'cannot reproduce Windows Python text-mode fixture' }
    $textModeBytes = [IO.File]::ReadAllBytes($sentinel)
    if ($textModeBytes.Length -ne 10 -or $textModeBytes[8] -ne 0x0d -or $textModeBytes[9] -ne 0x0a) {
        throw 'Windows Python text-mode did not produce CRLF; hypothesis not reproduced'
    }
    Write-Host 'PASS Windows Python text mode produced 70726573657276650d0a'
    if ((Get-Content -LiteralPath $sentinel -Raw) -eq "preserve`n") {
        throw 'historical PowerShell comparison unexpectedly accepted CRLF'
    }
    Write-Host 'PASS old PowerShell raw-text comparison rejects unchanged CRLF'
    foreach ($case in @(
        @{ name='CRLF'; bytes=[byte[]](0x70,0x72,0x65,0x73,0x65,0x72,0x76,0x65,0x0d,0x0a) },
        @{ name='altered'; bytes=[byte[]](0x70,0x72,0x65,0x73,0x65,0x72,0x76,0x66,0x0a) },
        @{ name='truncated'; bytes=[byte[]](0x70,0x72,0x65,0x73) },
        @{ name='appended'; bytes=[byte[]](0x70,0x72,0x65,0x73,0x65,0x72,0x76,0x65,0x0a,0x00) }
    )) {
        [IO.File]::WriteAllBytes($sentinel, $case.bytes)
        Expect-SentinelFailure $case.name
    }
    [IO.File]::Delete($sentinel)
    Expect-SentinelFailure 'missing'
    $other = Join-Path $root 'other'
    [IO.Directory]::CreateDirectory($other) | Out-Null
    & cmd.exe /d /c "mklink /J `"$sentinel`" `"$other`"" | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'cannot create redirected sentinel fixture' }
    Expect-SentinelFailure 'redirected'
    [IO.Directory]::Delete($sentinel)
    [IO.File]::WriteAllBytes($sentinel, $canonical)
    # The same guard runs after the lifecycle; the fixture remains unchanged.
    Assert-JarvisCandidateSentinel -Path $sentinel -Message 'uninstall altered foreign sentinel'
    Write-Host 'PASS unchanged after simulated lifecycle'
    [IO.File]::WriteAllBytes($sentinel, [byte[]](0x78))
    Expect-SentinelFailure 'data modified during simulated uninstall'
} finally {
    [IO.Directory]::Delete($root, $true)
}
