$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$NodeDir = Join-Path $RepoRoot ".tools\node-v24.18.0-win-x64"
$PnpmBin = Join-Path $RepoRoot ".tools\pnpm\node_modules\.bin"
$CargoHome = Join-Path $RepoRoot ".tools\cargo"
$RustupHome = Join-Path $RepoRoot ".tools\rustup"

if (!(Test-Path $NodeDir)) {
  throw "Project-local Node.js not found at $NodeDir"
}

if (!(Test-Path $PnpmBin)) {
  throw "Project-local pnpm not found at $PnpmBin"
}

if (!(Test-Path $CargoHome)) {
  throw "Project-local Cargo home not found at $CargoHome"
}

$env:CARGO_HOME = $CargoHome
$env:RUSTUP_HOME = $RustupHome

function Add-ProjectToolPaths {
  $toolPaths = @($NodeDir, $PnpmBin, "$CargoHome\bin")
  $existingPaths = $env:Path -split [IO.Path]::PathSeparator
  $pathsToAdd = $toolPaths | Where-Object { $existingPaths -notcontains $_ }
  if ($pathsToAdd.Count -gt 0) {
    $env:Path = (($pathsToAdd + $existingPaths) -join [IO.Path]::PathSeparator)
  }
}

$VsWhere = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $VsWhere) {
  $VsInstall = & $VsWhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
  if ($VsInstall) {
    $VsDevCmd = Join-Path $VsInstall "Common7\Tools\VsDevCmd.bat"
    if (Test-Path $VsDevCmd) {
      cmd /c "`"$VsDevCmd`" -arch=x64 -host_arch=x64 >nul && set" | ForEach-Object {
        if ($_ -match "^(.*?)=(.*)$") {
          Set-Item -Path "Env:$($matches[1])" -Value $matches[2]
        }
      }
    }
  }
}

Add-ProjectToolPaths

Write-Host "Life OS local dev environment loaded."
Write-Host "Repo: $RepoRoot"
Write-Host "Node: $(& node --version)"
Write-Host "pnpm: $(& pnpm --version)"
Write-Host "Rust: $(& rustc --version)"
Write-Host "Cargo: $(& cargo --version)"
