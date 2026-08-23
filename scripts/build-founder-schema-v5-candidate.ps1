[CmdletBinding()]
param(
  [ValidatePattern('^[A-Za-z0-9][A-Za-z0-9.-]*$')]
  [string]$ReviewSuffix
)

$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$configPath = Join-Path $repositoryRoot 'src-tauri\tauri.founder-dogfood-v5-candidate.conf.json'
$contractScript = Join-Path $PSScriptRoot 'founder-schema-v5-candidate-package.mjs'
$manifestScript = Join-Path $PSScriptRoot 'founder-dogfood-package.mjs'
$reviewRoot = Join-Path $repositoryRoot '.artifacts\desktop-schema-v5-founder-candidate-r1'

Push-Location $repositoryRoot
try {
  . (Join-Path $PSScriptRoot 'use-local-dev-env.ps1')
  if ($env:OS -ne 'Windows_NT') { throw 'The Founder schema-v5 candidate can only be built on Windows.' }
  & node $contractScript verify-source $repositoryRoot
  if ($LASTEXITCODE -ne 0) { throw 'Candidate source contract verification failed.' }

  $version = (Get-Content -Raw -Encoding UTF8 package.json | ConvertFrom-Json).version
  $gitSha = (git rev-parse HEAD).Trim()
  $hostLine = @(rustc -vV) | Where-Object { $_ -match '^host:\s+' } | Select-Object -First 1
  $target = ($hostLine -replace '^host:\s+', '').Trim()
  $previousGate = $env:VITE_LIFE_OS_FOUNDER_SCHEMA_V5
  $previousConfig = $env:TAURI_CONFIG
  try {
    $env:VITE_LIFE_OS_FOUNDER_SCHEMA_V5 = '1'
    & pnpm run build
    if ($LASTEXITCODE -ne 0) { throw 'Candidate frontend build failed.' }
    $env:TAURI_CONFIG = Get-Content -Raw -Encoding UTF8 $configPath
    & cargo rustc --manifest-path (Join-Path $repositoryRoot 'src-tauri\Cargo.toml') --release --bin life-os --no-default-features --features 'tauri/custom-protocol,founder-schema-v5' -- -C 'link-arg=/SUBSYSTEM:WINDOWS' -C 'link-arg=/ENTRY:mainCRTStartup'
    if ($LASTEXITCODE -ne 0) { throw 'Candidate binary build failed.' }
  } finally {
    if ($null -eq $previousGate) { Remove-Item Env:VITE_LIFE_OS_FOUNDER_SCHEMA_V5 -ErrorAction SilentlyContinue } else { $env:VITE_LIFE_OS_FOUNDER_SCHEMA_V5 = $previousGate }
    if ($null -eq $previousConfig) { Remove-Item Env:TAURI_CONFIG -ErrorAction SilentlyContinue } else { $env:TAURI_CONFIG = $previousConfig }
  }

  $binary = Join-Path $repositoryRoot 'src-tauri\target\release\life-os.exe'
  & node $contractScript verify-binary $binary
  if ($LASTEXITCODE -ne 0) { throw 'Candidate binary contract verification failed.' }
  $startedAt = [DateTime]::UtcNow.AddSeconds(-5)
  & pnpm exec tauri bundle --config $configPath --bundles nsis --ci --no-sign
  if ($LASTEXITCODE -ne 0) { throw 'Candidate bundling failed.' }
  & node $contractScript verify-binary $binary
  if ($LASTEXITCODE -ne 0) { throw 'Bundling changed the candidate binary contract.' }

  $candidates = @(Get-ChildItem 'src-tauri\target\release\bundle\nsis' -File -Filter '*.exe' | Where-Object { $_.Name -like '*Schema*v5*Candidate*' -and $_.LastWriteTimeUtc -ge $startedAt })
  if ($candidates.Count -ne 1) { throw "Expected one fresh candidate installer, found $($candidates.Count)." }
  $shortSha = $gitSha.Substring(0, 12)
  $reviewLeaf = if ($ReviewSuffix) { "$shortSha-$ReviewSuffix" } else { $shortSha }
  $final = Join-Path $reviewRoot $reviewLeaf
  if (Test-Path -LiteralPath $final) { throw "Review output already exists: $final" }
  New-Item -ItemType Directory -Path $final -Force | Out-Null
  $safeTarget = $target -replace '[^A-Za-z0-9_.-]', '_'
  $artifactLabel = if ($ReviewSuffix) { "-Review-$ReviewSuffix" } else { '' }
  $artifact = Join-Path $final "Life-OS-Founder-Schema-v5-Candidate-R1${artifactLabel}_${version}_${safeTarget}-setup.exe"
  Copy-Item -LiteralPath $candidates[0].FullName -Destination $artifact
  $manifest = Join-Path $final 'manifest.json'
  & node $manifestScript write-manifest --artifact $artifact --manifest $manifest --version $version --git-sha $gitSha --target $target
  if ($LASTEXITCODE -ne 0) { throw 'Candidate manifest creation failed.' }
  & node $manifestScript verify-manifest --artifact $artifact --manifest $manifest --version $version --git-sha $gitSha --target $target
  if ($LASTEXITCODE -ne 0) { throw 'Candidate manifest verification failed.' }
  Write-Host "Candidate review package: $artifact" -ForegroundColor Green
  Write-Host "Manifest: $manifest"
  Write-Host 'This unsigned Founder-only candidate has not been installed, distributed, deployed, or released.'
} finally { Pop-Location }
