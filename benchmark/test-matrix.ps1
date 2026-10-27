param(
    [switch]$BuildOnly,
    [string]$OutputDirectory
)
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$root = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$frozen = [System.IO.File]::ReadAllText((Join-Path $PSScriptRoot "frozen-files.json")) | ConvertFrom-Json
foreach ($entry in $frozen.files) {
    $file = Join-Path $root $entry.path
    if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { throw "冻结文件缺失：$($entry.path)" }
    # Git 在不同工作区可能转换换行；仅归一 CRLF，BOM 和其他内容都参与校验。
    $content = [System.Text.Encoding]::UTF8.GetString([System.IO.File]::ReadAllBytes($file)).Replace("`r`n", "`n")
    $algorithm = [System.Security.Cryptography.SHA256]::Create()
    try {
        $digest = [BitConverter]::ToString($algorithm.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($content))).Replace("-", "").ToLowerInvariant()
    } finally { $algorithm.Dispose() }
    if ($digest -ne $entry.sha256) { throw "冻结文件已变更：$($entry.path)" }
}
if (-not $OutputDirectory) { $OutputDirectory = Join-Path $root (".benchmark-results/" + [guid]::NewGuid().ToString("N")) }
$output = [System.IO.Path]::GetFullPath($OutputDirectory)
if (Test-Path -LiteralPath $output) { throw "结果目录已存在，拒绝覆盖：$output" }
[void][System.IO.Directory]::CreateDirectory($output)
$configurations = @("default", "math", "serde", "fory", "math,serde", "math,fory", "serde,fory", "math,serde,fory")
$rows = @()
$failed = $false
Push-Location -LiteralPath $root
try {
    foreach ($configuration in $configurations) {
        $arguments = @("test", "--locked", "--offline", "--no-fail-fast")
        if ($configuration -ne "default") { $arguments += @("--features", $configuration) }
        if ($BuildOnly) { $arguments += "--no-run" } else { $arguments += @("--", "--test-threads=2") }
        $log = Join-Path $output ($configuration.Replace(",", "-") + ".log")
        $watch = [System.Diagnostics.Stopwatch]::StartNew()
        # PowerShell 5.1 将原生 stderr 包装成错误记录；Cargo 正常进度也写 stderr。
        $previousErrorAction = $ErrorActionPreference
        try {
            $ErrorActionPreference = "Continue"
            & cargo @arguments > $log 2>&1
            $code = $LASTEXITCODE
        } finally { $ErrorActionPreference = $previousErrorAction }
        $watch.Stop()
        $text = [System.IO.File]::ReadAllText($log)
        $passed = 0; $testsFailed = 0; $ignored = 0
        foreach ($match in [regex]::Matches($text, "test result: .*? (\d+) passed; (\d+) failed; (\d+) ignored;")) {
            $passed += [int]$match.Groups[1].Value
            $testsFailed += [int]$match.Groups[2].Value
            $ignored += [int]$match.Groups[3].Value
        }
        $rows += [pscustomobject]@{
            features = $configuration; exitCode = $code; elapsedMs = $watch.ElapsedMilliseconds
            passed = $passed; failed = $testsFailed; ignored = $ignored
            buildOnly = [bool]$BuildOnly; log = $log
        }
        if ($code -ne 0 -or (-not $BuildOnly -and ($passed + $testsFailed -eq 0))) { $failed = $true }
    }
} finally { Pop-Location }
[pscustomobject]@{ buildOnly = [bool]$BuildOnly; configurations = $rows } |
    ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $output "summary.json") -Encoding UTF8
$rows | Format-Table -AutoSize
"结果目录：$output"
if ($failed) { exit 1 }
