---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/08
depends:
  - docs/dev/01_Environment_Setup.md
  - docs/dev/03_Tauri_Runtime_Verification.md
  - docs/dev/04_Dependency_Installation_Log.md
referenced_by:
  - docs/dev/00_Local_Development.md
---

# 05 Development Agent Runbook

## Purpose

This document explains what was installed, why it was installed that way, and how a development agent should start or stop the Life OS MVP desktop runtime.

The goal is to avoid version confusion.

The project should be runnable without forcing Node, pnpm, Rust, or Cargo versions onto other projects on the same machine.

## Installation Policy

Prefer project-local dependencies when possible.

Use system-level dependencies only when Windows native desktop development requires them.

Project-local dependencies are stored under `.tools/`.

`.tools/` is ignored by Git and should not be committed.

## What Was Installed Project-locally

| Dependency | Version | Location | Why |
| --- | --- | --- | --- |
| Node.js | `v24.18.0` | `.tools/node-v24.18.0-win-x64` | Matches `.nvmrc` without changing global Node. |
| pnpm | `11.10.0` | `.tools/pnpm` | Matches `package.json` without changing global pnpm. |
| Rust | `rustc 1.96.1` stable MSVC | `.tools/rustup` | Gives this repo its own Rust toolchain. |
| Cargo | `cargo 1.96.1` | `.tools/cargo` | Gives this repo its own Cargo home and crate cache. |

Rust was installed with `--no-modify-path`.

The repo controls Rust through:

- `.tools/rustup`
- `.tools/cargo`
- `rust-toolchain.toml`

## What Was Installed System-wide

| Dependency | Location / Version | Why |
| --- | --- | --- |
| Microsoft Edge WebView2 Runtime | `149.0.4022.98` | Required by Tauri WebView on Windows. |
| Visual Studio Build Tools | `C:\BuildTools` | Required by Rust MSVC and Tauri native builds. |
| Windows SDK | `10.0.22621.0`, `10.0.26100.0` | Required by Windows native compilation. |

These dependencies are system-level because Tauri GUI development depends on Windows native tooling.

They do not pin application-level JavaScript or Rust versions.

## Download / Install Notes

Installers were downloaded to:

```text
C:\tmp\life-os-deps
```

An attempt to install Node.js through the MSI returned exit code `1603`.

The chosen solution was portable Node.js under `.tools/`, which avoids global Node changes.

## Environment Loader

Use:

```powershell
.\scripts\use-local-dev-env.ps1
```

This script temporarily sets, for the current PowerShell session:

- `PATH`
- `CARGO_HOME`
- `RUSTUP_HOME`
- Visual Studio Build Tools environment variables, if available

It does not permanently modify system environment variables.

Expected output:

```text
Life OS local dev environment loaded.
Node: v24.18.0
pnpm: 11.10.0
Rust: rustc 1.96.1
Cargo: cargo 1.96.1
```

## Standard Verification Commands

From the repository root:

```powershell
.\scripts\use-local-dev-env.ps1
pnpm install
pnpm run typecheck
pnpm run build
pnpm run tauri:dev
```

Expected results:

| Command | Expected Result |
| --- | --- |
| `pnpm install` | Completes with `pnpm v11.10.0`. |
| `pnpm run typecheck` | TypeScript passes. |
| `pnpm run build` | Vite builds `dist/`. |
| `pnpm run tauri:dev` | Opens the Life OS desktop window. |

## Background Launch Procedure

For Codex or another agent that needs to start the app without occupying the current terminal, use a hidden PowerShell process and write logs under `.tools/logs/`.

The command used during verification:

```powershell
$logDir = Join-Path (Get-Location) ".tools\logs"
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
$out = Join-Path $logDir "tauri-dev.log"
$err = Join-Path $logDir "tauri-dev.err.log"
Set-Content -Path $out -Value ""
Set-Content -Path $err -Value ""
$cmd = ". 'D:\Lin\Project\LifeOperatingSystem\scripts\use-local-dev-env.ps1'; pnpm run tauri:dev"
Start-Process -FilePath powershell.exe `
  -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", $cmd) `
  -WorkingDirectory "D:\Lin\Project\LifeOperatingSystem" `
  -RedirectStandardOutput $out `
  -RedirectStandardError $err `
  -WindowStyle Hidden
```

Logs:

```text
.tools/logs/tauri-dev.log
.tools/logs/tauri-dev.err.log
```

## Runtime Verification Result

The host runtime was verified successfully.

Observed:

- Vite started on `http://localhost:1420/`.
- Cargo compiled the Tauri app.
- `tauri-plugin-sql` compiled successfully.
- `target\debug\life-os.exe` launched.
- Closing the Tauri window also ended the dev session.
- Port `1420` was no longer occupied after the window was closed.

## How To Check Whether The App Is Still Running

Use:

```powershell
Get-CimInstance Win32_Process |
  Where-Object {
    $_.CommandLine -like "*tauri:dev*" -or
    $_.CommandLine -like "*target\debug\life-os.exe*" -or
    $_.Name -like "*life-os*" -or
    $_.CommandLine -like "*localhost:1420*"
  } |
  Select-Object ProcessId,Name,CommandLine
```

Also check port `1420`:

```powershell
Get-NetTCPConnection -LocalPort 1420 -ErrorAction SilentlyContinue
```

If there is no `life-os.exe` and port `1420` is not occupied, the dev session is closed.

## How To Stop The App

Preferred:

Close the Life OS desktop window.

If a process remains, identify it first:

```powershell
Get-CimInstance Win32_Process |
  Where-Object {
    $_.CommandLine -like "*tauri:dev*" -or
    $_.CommandLine -like "*target\debug\life-os.exe*" -or
    $_.Name -like "*life-os*"
  } |
  Select-Object ProcessId,Name,CommandLine
```

Then stop only the matching process.

Do not kill unrelated `node.exe` or `powershell.exe` processes without checking their command line.

## Important Generated Files

Generated or added during runtime verification:

- `src-tauri/Cargo.lock`
- `src-tauri/gen/schemas/*`
- `src-tauri/icons/icon.ico`

`src-tauri/Cargo.lock` should be kept for reproducible Rust app builds.

`src-tauri/icons/icon.ico` is currently a neutral placeholder required by Tauri Windows resource generation.

## Known Shell Noise

Some command outputs include this unrelated line:

```text
conda-script.py: error: argument COMMAND: invalid choice: 'initialize'
```

This appears to come from the surrounding shell/runtime initialization.

It did not block:

- `pnpm install`
- `pnpm run typecheck`
- `pnpm run build`
- `pnpm run tauri:dev`

Do not treat that line as a Life OS app failure unless it becomes the first failing command.

## Agent Rules

Development agents should:

- Load `.\scripts\use-local-dev-env.ps1` before running project commands.
- Use project-local Node, pnpm, Rust, and Cargo.
- Avoid global installs unless explicitly required.
- Avoid modifying Book Zero theory files during runtime work.
- Avoid adding product functionality while verifying environment.
- Record new native dependency changes in `docs/dev/04_Dependency_Installation_Log.md`.
