[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

$repositoryRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repositoryRoot

try {
  . (Join-Path $PSScriptRoot 'use-local-dev-env.ps1')

  function Invoke-VerificationStep {
    param(
      [Parameter(Mandatory = $true)][string]$Name,
      [Parameter(Mandatory = $true)][scriptblock]$Command
    )

    Write-Host "`n==> $Name" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
      throw "$Name failed with exit code $LASTEXITCODE."
    }
  }

  Invoke-VerificationStep 'pnpm toolchain' { pnpm --version }
  Invoke-VerificationStep 'AI workflow contract tests' { pnpm run test:workflow }
  Invoke-VerificationStep 'AI workflow state validation' { pnpm run verify:workflow }
  Invoke-VerificationStep 'Vitest' { pnpm run test:run }
  Invoke-VerificationStep 'TypeScript typecheck' { pnpm run typecheck }
  Invoke-VerificationStep 'Frontend build' { pnpm run build }
  Invoke-VerificationStep 'SQLite Rust tests' { cargo test --manifest-path src-tauri/Cargo.toml sqlite::tests -- --nocapture }
  Invoke-VerificationStep 'Rust check' { cargo check --manifest-path src-tauri/Cargo.toml }
  Invoke-VerificationStep 'Unstaged whitespace errors' { git diff --check }
  Invoke-VerificationStep 'Staged whitespace errors' { git diff --cached --check }

  Write-Host "`n==> UTF-8 and replacement-character scan" -ForegroundColor Cyan
  $utf8 = [System.Text.UTF8Encoding]::new($false, $true)
  $repositoryFiles = @(git ls-files --cached --others --exclude-standard)
  if ($LASTEXITCODE -ne 0) { throw 'Unable to list repository files.' }
  $repositoryFiles = @($repositoryFiles | Where-Object { $_ } | Sort-Object -Unique)
  $encodingFailures = [System.Collections.Generic.List[string]]::new()
  $replacementFailures = [System.Collections.Generic.List[string]]::new()
  foreach ($relativePath in $repositoryFiles) {
    if (-not (Test-Path -LiteralPath $relativePath -PathType Leaf)) { continue }
    $extension = [System.IO.Path]::GetExtension($relativePath).ToLowerInvariant()
    if ($extension -notin @('.css', '.html', '.json', '.md', '.ps1', '.rs', '.toml', '.ts', '.tsx', '.yaml', '.yml') -and
      $relativePath -notin @('.env.example', '.gitignore', '.nvmrc')) {
      continue
    }
    try {
      $content = $utf8.GetString([System.IO.File]::ReadAllBytes((Join-Path $repositoryRoot $relativePath)))
    } catch {
      $encodingFailures.Add($relativePath)
      continue
    }
    if ($content.Contains([char]0xFFFD) -or $content -match '\?{4,}') {
      $replacementFailures.Add($relativePath)
    }
  }
  if ($encodingFailures.Count -gt 0) { throw "Invalid UTF-8: $($encodingFailures -join ', ')" }
  if ($replacementFailures.Count -gt 0) { throw "Replacement-character or four-question-mark marker: $($replacementFailures -join ', ')" }
  Write-Host 'UTF-8 and replacement-character scan passed.'

  Write-Host "`n==> Repository secret-file check" -ForegroundColor Cyan
  $secretFiles = $repositoryFiles | Where-Object {
    $_ -match '(^|/)(\.env($|\.)|.*\.(pem|key|p12|pfx|crt)$|id_rsa$|credentials\.json$)' -and
    $_ -notmatch '(^|/)\.env\.example$'
  }
  if ($secretFiles) { throw "Repository secret-like files: $($secretFiles -join ', ')" }
  Write-Host 'No tracked or non-ignored untracked secret-like files found.'

  Write-Host "`n==> Local Markdown-link sanity" -ForegroundColor Cyan
  $linkFailures = [System.Collections.Generic.List[string]]::new()
  Get-ChildItem -Path $repositoryRoot -Recurse -File -Filter *.md |
    Where-Object { $_.FullName -notmatch '[\\/]node_modules[\\/]|[\\/]\.git[\\/]|[\\/]\.tools[\\/]' } |
    ForEach-Object {
      $document = $_
      $matches = [regex]::Matches((Get-Content -LiteralPath $document.FullName -Raw -Encoding UTF8), '\]\(([^)]+)\)')
      foreach ($match in $matches) {
        $target = $match.Groups[1].Value.Trim()
        if ($target -match '^(https?://|mailto:|#)' -or $target -notmatch '\.md(?:#.*)?$') { continue }
        $pathOnly = $target.Split('#')[0]
        $resolved = Join-Path $document.DirectoryName $pathOnly
        if (-not (Test-Path -LiteralPath $resolved -PathType Leaf)) {
          $linkFailures.Add("$($document.FullName): $target")
        }
      }
    }
  if ($linkFailures.Count -gt 0) { throw "Broken local Markdown links: $($linkFailures -join '; ')" }
  Write-Host 'Local Markdown-link sanity passed.'

  Write-Host "`n==> Constitution diff report" -ForegroundColor Cyan
  $unstagedConstitutionDiff = git diff --name-only -- docs/00_Constitution.md
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect Constitution diff.' }
  $stagedConstitutionDiff = git diff --cached --name-only -- docs/00_Constitution.md
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect staged Constitution diff.' }
  if ($unstagedConstitutionDiff -or $stagedConstitutionDiff) {
    Write-Warning 'Constitution has staged or unstaged content changes. Explicit founder authorization is required.'
  } else {
    Write-Host 'No staged or unstaged Constitution content diff.'
  }

  Write-Host "`nAll deterministic verification checks passed." -ForegroundColor Green
} finally {
  Pop-Location
}
