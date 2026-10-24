# Side-effect-free byte guard shared by Windows candidate acceptance and regressions.
function Assert-JarvisCandidateSentinel {
    param([Parameter(Mandatory=$true)][string]$Path,
          [Parameter(Mandatory=$true)][string]$Message)
    $expected = [byte[]](0x70, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x0a)
    try {
        $item = Get-Item -LiteralPath $Path -Force -ErrorAction Stop
        if ($item.PSIsContainer -or ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -or
            -not ($item -is [IO.FileInfo])) { throw 'not an ordinary file' }
        $actual = [IO.File]::ReadAllBytes($Path)
        if ($actual.Length -ne $expected.Length) { throw 'length mismatch' }
        for ($i = 0; $i -lt $expected.Length; $i++) {
            if ($actual[$i] -ne $expected[$i]) { throw 'byte mismatch' }
        }
    } catch {
        throw "$Message`: $($_.Exception.Message)"
    }
}
