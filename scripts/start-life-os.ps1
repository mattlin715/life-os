param(
  [switch]$NoEnvFile
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

. "$repoRoot\scripts\use-local-dev-env.ps1"

$envFile = Join-Path $repoRoot ".env.local"

if (-not $NoEnvFile -and (Test-Path $envFile)) {
  Get-Content $envFile | ForEach-Object {
    $line = $_.Trim()

    if (-not $line -or $line.StartsWith("#") -or -not $line.Contains("=")) {
      return
    }

    $key, $value = $line.Split("=", 2)
    $key = $key.Trim()
    $value = $value.Trim().Trim('"').Trim("'")

    if ($key) {
      Set-Item -Path "Env:$key" -Value $value
    }
  }
}

Write-Host ""
Write-Host "Life OS is starting." -ForegroundColor Green

$provider = if ($env:AI_PROVIDER) { $env:AI_PROVIDER.ToLowerInvariant() } else { "" }

if (($provider -eq "gemini" -or (-not $provider -and $env:GEMINI_API_KEY)) -and $env:GEMINI_API_KEY) {
  $model = if ($env:GEMINI_MODEL) { $env:GEMINI_MODEL } else { "gemini-3.1-flash-lite" }
  Write-Host "AI: Gemini enabled ($model)" -ForegroundColor Green
} elseif (($provider -eq "openai" -or (-not $provider -and $env:OPENAI_API_KEY)) -and $env:OPENAI_API_KEY) {
  $model = if ($env:OPENAI_MODEL) { $env:OPENAI_MODEL } else { "gpt-5-nano" }
  Write-Host "AI: OpenAI enabled ($model)" -ForegroundColor Green
} else {
  Write-Host "AI: local mirror fallback. Add AI_PROVIDER plus GEMINI_API_KEY or OPENAI_API_KEY to .env.local to enable real AI." -ForegroundColor Yellow
}

Write-Host ""
pnpm run tauri:dev
