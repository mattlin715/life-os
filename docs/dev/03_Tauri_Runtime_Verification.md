---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/07
depends:
  - docs/dev/01_Environment_Setup.md
  - docs/dev/02_Dev_Commands.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
referenced_by:
  - docs/dev/00_Local_Development.md
---

# 03 Tauri Runtime Verification

## Purpose

This document defines how to verify that the Life OS Tauri desktop runtime can start on a real host machine.

This is not a feature test.

This is not an MVP behavior test.

This verification only answers one question:

Can the current Tauri + React + SQLite skeleton launch as a desktop app on Windows 11 and macOS host machines?

Do not run this verification inside Docker or Devcontainer.

Tauri desktop runtime depends on OS GUI, native WebView, and native toolchains.

## Verification Boundary

This sprint verifies:

- Node and pnpm can install frontend dependencies.
- TypeScript can typecheck the scaffold.
- Vite can build the frontend.
- Tauri can start the desktop shell.
- The initial desktop window opens.
- The minimal Life OS page renders.

This sprint does not verify:

- SQLite schema.
- Local evidence store.
- AI provider calls.
- Product workflow.
- Desktop release packaging.
- Cross-machine data portability.

## Windows 11 Runtime Checklist

Before running commands, confirm:

- Node.js matches `.nvmrc`.
- pnpm matches `package.json`.
- Rust is installed through rustup.
- Cargo is available in the terminal.
- Rust uses the MSVC toolchain.
- Visual Studio Build Tools are installed.
- Windows SDK is installed.
- Microsoft Edge WebView2 Runtime is installed.
- The command is running on the Windows host, not inside Devcontainer.

Recommended checks:

```powershell
node --version
pnpm --version
rustc --version
cargo --version
rustup show
```

Expected environment:

- Node: `v24.18.0`
- pnpm: `11.10.0`
- Rust: `stable`
- Cargo: available

## macOS Runtime Checklist

Before running commands, confirm:

- Node.js matches `.nvmrc`.
- pnpm matches `package.json`.
- Rust is installed through rustup.
- Cargo is available in the terminal.
- Xcode Command Line Tools are installed.
- macOS WebKit runtime is available through the OS.
- The command is running on the macOS host, not inside Devcontainer.

Recommended checks:

```bash
node --version
pnpm --version
rustc --version
cargo --version
rustup show
xcode-select -p
```

Expected environment:

- Node: `v24.18.0`
- pnpm: `11.10.0`
- Rust: `stable`
- Cargo: available

## Required Commands

Run commands in this order from the repository root.

```bash
pnpm install
pnpm run typecheck
pnpm run build
pnpm run tauri:dev
```

Do not skip `typecheck` or `build`.

They separate frontend problems from Tauri runtime problems.

## Expected Result

Expected command results:

| Command | Expected Result |
| --- | --- |
| `pnpm install` | Dependencies install from `pnpm-lock.yaml` without lockfile changes. |
| `pnpm run typecheck` | TypeScript completes without errors. |
| `pnpm run build` | Vite builds the frontend into `dist/`. |
| `pnpm run tauri:dev` | Tauri starts and opens a desktop window. |

Expected desktop window:

- Shows `Life OS`.
- Shows `We Build Mirrors, Not Oracles.`
- Shows a textarea with placeholder `Write one experience you want to understand.`

No save behavior is expected.

No SQLite data should be created by this verification.

No AI provider call should happen.

## How To Record Verification Result

For every verification attempt, record:

- OS.
- Machine.
- Node version.
- pnpm version.
- Rust version.
- Cargo version.
- Each command result.
- Error message if any command fails.
- Whether the desktop window opened.
- Whether the expected text rendered.

If `pnpm run tauri:dev` fails, keep the full terminal error.

Do not summarize native toolchain errors too early.

The exact compiler or WebView message is often the useful part.

## Common Failure Cases

### `cargo` Or `rustc` Not Found

Cause:

Rust is not installed, or the terminal has not reloaded PATH after installation.

Check:

```bash
rustc --version
cargo --version
rustup show
```

Fix:

Install Rust through rustup.

Restart the terminal.

On Windows, verify the active toolchain is MSVC.

### Visual Studio Build Tools Missing

Windows symptom:

Tauri or Rust compilation fails with linker, MSVC, Windows SDK, or C++ build tool errors.

Fix:

Install Visual Studio Build Tools.

Select:

- Desktop development with C++.
- MSVC toolchain.
- Windows SDK.

Restart the terminal after installation.

### WebView2 Missing

Windows symptom:

Tauri starts compiling, but the desktop runtime reports WebView2 is missing or cannot initialize.

Fix:

Install Microsoft Edge WebView2 Evergreen Runtime.

Then rerun:

```powershell
pnpm run tauri:dev
```

### pnpm Version Mismatch

Symptom:

Install behavior differs across machines, lockfile changes unexpectedly, or pnpm warns about package manager mismatch.

Check:

```bash
pnpm --version
```

Expected:

```text
11.10.0
```

Fix:

```bash
corepack enable
corepack prepare pnpm@11.10.0 --activate
```

Then rerun:

```bash
pnpm install
```

### Tauri CLI Error

Symptom:

`pnpm run tauri:dev` fails before opening the app.

Check:

```bash
pnpm exec tauri --version
pnpm run build
```

If frontend build fails, fix frontend first.

If frontend build passes, inspect the Tauri CLI error and native toolchain output.

### SQL Plugin Compile Issue

Symptom:

Rust compilation fails around `tauri-plugin-sql`, SQLite, or plugin initialization.

Check:

```bash
cargo --version
rustc --version
```

Then inspect the first Rust compiler error, not only the final failure line.

This project currently includes the SQL plugin as a runtime dependency only.

No schema or migrations should exist yet.

### Vite Dev Server Starts But Desktop Window Does Not Open

Symptom:

Vite starts successfully, but no Tauri window appears.

Check:

- Is the command `pnpm run tauri:dev`, not only `pnpm run dev`?
- Is the command running on the host OS?
- Did Tauri compilation fail after Vite started?
- Did the OS block the app window or show a security prompt?
- On Windows, is WebView2 installed?
- On macOS, are Xcode Command Line Tools installed?

Keep the full terminal output.

The Vite server alone does not prove Tauri runtime works.

## Debug Order If Verification Fails

Use this order:

1. Confirm `node --version` and `pnpm --version`.
2. Confirm `rustc --version` and `cargo --version`.
3. Run `pnpm install`.
4. Run `pnpm run typecheck`.
5. Run `pnpm run build`.
6. Run `pnpm run tauri:dev`.
7. Record the first failing command.
8. Record the exact error message.

Do not change application code until the failing layer is identified.

## Verification Log

```md
## Verification Log

Date:
Machine:
OS:
Node:
pnpm:
Rust:
Cargo:
Result:
Notes:
```
