---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/07
depends:
  - docs/dev/01_Environment_Setup.md
  - docs/dev/03_Tauri_Runtime_Verification.md
referenced_by: []
---

# 04 Dependency Installation Log

## Purpose

This document records the development dependencies installed for the Life OS MVP runtime verification.

The goal is reproducibility without global version confusion.

Where possible, dependencies are installed project-locally under `.tools/`.

System-level dependencies are used only when required by Windows native desktop development.

## Installation Summary

| Dependency | Version / Status | Install Location | Scope | Notes |
| --- | --- | --- | --- | --- |
| Node.js | `v24.18.0` | `.tools/node-v24.18.0-win-x64` | Project-local | Portable Node.js archive. Does not modify global PATH. |
| pnpm | `11.10.0` | `.tools/pnpm` | Project-local | Installed through project-local Node/npm. |
| Rust | `rustc 1.96.1` stable MSVC | `.tools/rustup` | Project-local | Uses project-local `RUSTUP_HOME`. |
| Cargo | `cargo 1.96.1` | `.tools/cargo` | Project-local | Uses project-local `CARGO_HOME`. |
| Microsoft Edge WebView2 Runtime | `149.0.4022.98` | Windows system runtime | System | Already installed before this sprint. |
| Visual Studio Build Tools | Installed | `C:\BuildTools` | System | Required by Rust MSVC and Tauri on Windows. |
| Windows SDK | `10.0.22621.0`, `10.0.26100.0` | Windows Kits | System | Installed with Visual Studio Build Tools. |
| Tauri placeholder icon | `icon.ico` | `src-tauri/icons/icon.ico` | Repository | Required by Tauri Windows resource generation. |

## Project-local Dependencies

These dependencies are used only when the project-local dev environment is loaded:

```powershell
.\scripts\use-local-dev-env.ps1
```

The script temporarily sets:

- `PATH`
- `CARGO_HOME`
- `RUSTUP_HOME`

It does not permanently change system environment variables.

## System-level Dependencies

The following dependencies are installed at the Windows system level because Tauri desktop development depends on native OS tooling:

- Microsoft Edge WebView2 Runtime
- Visual Studio Build Tools
- Windows SDK

These are not project-local, but they do not pin application-level JavaScript or Rust versions.

The project still controls Node, pnpm, Rust, and Cargo versions through `.tools/`.

## Verification Commands

Use a PowerShell session from the repository root:

```powershell
.\scripts\use-local-dev-env.ps1
pnpm install
pnpm run typecheck
pnpm run build
pnpm run tauri:dev
```

## Verification Result

Date: `2026/07/07`

Machine: Windows host

Result:

| Command | Result |
| --- | --- |
| `pnpm install` | Passed |
| `pnpm run typecheck` | Passed |
| `pnpm run build` | Passed |
| `pnpm run tauri:dev` | Passed |

Observed runtime result:

- Vite started on `http://localhost:1420/`.
- Rust crates downloaded and compiled.
- `tauri-plugin-sql` compiled successfully.
- Tauri launched `target\debug\life-os.exe`.

Additional generated files:

- `src-tauri/Cargo.lock`
- `src-tauri/gen/schemas/*`
- `src-tauri/icons/icon.ico`

## Notes

An earlier attempt to install Node.js through the MSI installer returned exit code `1603`.

To avoid global version changes, the project now uses portable Node.js under `.tools/`.

Rust was installed with `--no-modify-path` and project-local `CARGO_HOME` / `RUSTUP_HOME`.

Visual Studio Build Tools was installed with the C++ workload because Rust MSVC and Tauri require native Windows compiler tools.
