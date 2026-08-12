[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$configPath = Join-Path $repositoryRoot 'src-tauri\tauri.founder-dogfood.conf.json'
$contractScript = Join-Path $PSScriptRoot 'founder-dogfood-package.mjs'
$reviewRoot = Join-Path $repositoryRoot '.artifacts\windows-founder-dogfood-r1'

Push-Location $repositoryRoot
try {
  . (Join-Path $PSScriptRoot 'use-local-dev-env.ps1')

  if ($env:OS -ne 'Windows_NT') {
    throw 'Windows Founder Dogfooding Package R1 can only be built on Windows.'
  }

  & node $contractScript verify-source --root $repositoryRoot
  if ($LASTEXITCODE -ne 0) { throw 'Founder package source contract verification failed.' }

  $applicationVersion = (Get-Content -Raw -Encoding UTF8 (Join-Path $repositoryRoot 'package.json') | ConvertFrom-Json).version
  $gitSha = (git rev-parse HEAD).Trim()
  if ($LASTEXITCODE -ne 0 -or $gitSha -notmatch '^[0-9a-f]{40}$') { throw 'Unable to resolve the exact Git SHA.' }
  $rustVersion = @(rustc -vV)
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect the Rust build target.' }
  $targetLine = $rustVersion | Where-Object { $_ -match '^host:\s+' } | Select-Object -First 1
  if (-not $targetLine) { throw 'Rust host target was not reported.' }
  $buildTarget = ($targetLine -replace '^host:\s+', '').Trim()

  & pnpm run build
  if ($LASTEXITCODE -ne 0) { throw 'Founder package frontend build failed.' }

  $founderTauriConfig = Get-Content -Raw -Encoding UTF8 $configPath
  $previousTauriConfig = $env:TAURI_CONFIG
  try {
    $env:TAURI_CONFIG = $founderTauriConfig
    & cargo rustc --manifest-path (Join-Path $repositoryRoot 'src-tauri\Cargo.toml') --release --bin life-os --features 'tauri/custom-protocol' -- -C 'link-arg=/SUBSYSTEM:WINDOWS' -C 'link-arg=/ENTRY:mainCRTStartup'
    if ($LASTEXITCODE -ne 0) { throw 'Founder package binary build failed.' }
  } finally {
    if ($null -eq $previousTauriConfig) {
      Remove-Item Env:TAURI_CONFIG -ErrorAction SilentlyContinue
    } else {
      $env:TAURI_CONFIG = $previousTauriConfig
    }
  }

  $applicationBinary = Join-Path $repositoryRoot 'src-tauri\target\release\life-os.exe'
  & node $contractScript verify-founder-binary --binary $applicationBinary
  if ($LASTEXITCODE -ne 0) { throw 'Founder package binary identity or Windows subsystem is invalid.' }

  $startedAt = [DateTime]::UtcNow.AddSeconds(-5)
  & pnpm exec tauri bundle --config $configPath --bundles nsis --ci --no-sign
  if ($LASTEXITCODE -ne 0) { throw 'Tauri Founder package bundling failed.' }
  & node $contractScript verify-founder-binary --binary $applicationBinary
  if ($LASTEXITCODE -ne 0) { throw 'Tauri bundling changed the Founder binary contract.' }

  $bundleDirectory = Join-Path $repositoryRoot 'src-tauri\target\release\bundle\nsis'
  $candidates = @(
    Get-ChildItem -LiteralPath $bundleDirectory -File -Filter '*.exe' |
      Where-Object { $_.Name -like '*Founder*Dogfood*' -and $_.LastWriteTimeUtc -ge $startedAt }
  )
  if ($candidates.Count -ne 1) {
    throw "Expected exactly one fresh Founder NSIS installer, found $($candidates.Count)."
  }

  $shortSha = $gitSha.Substring(0, 12)
  $finalDirectory = Join-Path $reviewRoot $shortSha
  if (Test-Path -LiteralPath $finalDirectory) {
    throw "Review output already exists: $finalDirectory. Remove that ignored directory explicitly before rebuilding."
  }
  $stagingDirectory = Join-Path $reviewRoot ".building-$shortSha-$PID"
  New-Item -ItemType Directory -Path $stagingDirectory -Force | Out-Null

  try {
    $safeTarget = $buildTarget -replace '[^A-Za-z0-9_.-]', '_'
    $artifactName = "Life-OS-Founder-Dogfood-R1_${applicationVersion}_${safeTarget}-setup.exe"
    $artifactPath = Join-Path $stagingDirectory $artifactName
    $manifestPath = Join-Path $stagingDirectory 'manifest.json'
    Copy-Item -LiteralPath $candidates[0].FullName -Destination $artifactPath

    & node $contractScript write-manifest --artifact $artifactPath --manifest $manifestPath --version $applicationVersion --git-sha $gitSha --target $buildTarget
    if ($LASTEXITCODE -ne 0) { throw 'Founder package manifest creation failed.' }
    & node $contractScript verify-manifest --artifact $artifactPath --manifest $manifestPath --version $applicationVersion --git-sha $gitSha --target $buildTarget
    if ($LASTEXITCODE -ne 0) { throw 'Founder package manifest verification failed.' }

    Move-Item -LiteralPath $stagingDirectory -Destination $finalDirectory
    $finalArtifact = Join-Path $finalDirectory $artifactName
    $finalManifest = Join-Path $finalDirectory 'manifest.json'
    Write-Host "`nFounder-only review package created." -ForegroundColor Green
    Write-Host "Artifact: $finalArtifact"
    Write-Host "Manifest: $finalManifest"
    Write-Host 'This unsigned local package has not been installed, distributed, deployed, or released.'
  } catch {
    if (Test-Path -LiteralPath $stagingDirectory) {
      Remove-Item -LiteralPath $stagingDirectory -Recurse -Force
    }
    throw
  }
} finally {
  Pop-Location
}
