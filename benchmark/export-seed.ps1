param(
    [Parameter(Mandatory = $true)][string]$Destination,
    [string]$Revision = "benchmark/implementation-seed"
)
$ErrorActionPreference = "Stop"
$root = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$destinationPath = [System.IO.Path]::GetFullPath($Destination)
if (Test-Path -LiteralPath $destinationPath) { throw "目标目录已存在，拒绝覆盖样本：$destinationPath" }
if (-not (Test-Path -LiteralPath ([System.IO.Path]::GetDirectoryName($destinationPath)) -PathType Container)) {
    throw "目标父目录不存在"
}
if ($destinationPath.StartsWith($root + [System.IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
    throw "评测副本必须在维护仓库之外"
}
$commit = & git -C $root rev-parse --verify "$Revision^{commit}"
if ($LASTEXITCODE -ne 0) { throw "无法解析种子提交" }
$marker = & git -C $root show "$($commit.Trim()):benchmark/seed.json"
if ($LASTEXITCODE -ne 0 -or (($marker -join [Environment]::NewLine) | ConvertFrom-Json).state -ne "declarations-only") {
    throw "指定提交不是已标记的评测起点"
}
$archive = Join-Path ([System.IO.Path]::GetTempPath()) ("fixed-scale-seed-" + [guid]::NewGuid().ToString("N") + ".tar")
try {
    & git -C $root archive --format=tar --output=$archive $commit.Trim()
    if ($LASTEXITCODE -ne 0) { throw "导出种子失败" }
    $sha256 = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    [void][System.IO.Directory]::CreateDirectory($destinationPath)
    & tar -xf $archive -C $destinationPath
    if ($LASTEXITCODE -ne 0) { throw "解包失败，保留目录供检查" }
    & git -C $destinationPath init --quiet --initial-branch benchmark-run
    if ($LASTEXITCODE -ne 0) { throw "初始化独立工作区失败" }
    [pscustomobject]@{ sourceCommit = $commit.Trim(); archiveSha256 = $sha256; destination = $destinationPath } | ConvertTo-Json
} finally {
    if (Test-Path -LiteralPath $archive) { Remove-Item -LiteralPath $archive -Force }
}
