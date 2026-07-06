---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/adr/ADR-0006-mvp-tech-stack.md
  - docs/dev/00_Local_Development.md
referenced_by: []
---

# 01 Environment Setup

## Purpose

This document defines the minimum reproducible development environment for Life OS MVP.

The goal is to make Work Mac and Windows 11 development predictable without pretending that Tauri desktop GUI development can be fully containerized.

Tauri depends on OS GUI, WebView, and native toolchains.

Devcontainer and CI are used only for non-GUI tasks:

- `pnpm install`
- `pnpm run typecheck`
- `pnpm run build`

They do not replace local desktop runtime verification.

## Pinned Versions

| Tool | Version |
| --- | --- |
| Node.js | `24.18.0` |
| pnpm | `11.10.0` |
| Rust | `stable` |

Node is pinned in `.nvmrc`.

pnpm is pinned in `package.json` through `packageManager`.

Rust is pinned in `rust-toolchain.toml` through the stable channel.

`pnpm-workspace.yaml` pins the expected Node version for pnpm checks and explicitly allows the `esbuild` install script required by Vite.

## Windows 11 Setup

Install Node.js using a version manager or the official installer.

Recommended:

```powershell
winget install OpenJS.NodeJS.LTS
corepack enable
corepack prepare pnpm@11.10.0 --activate
```

Install Rust:

```powershell
winget install --id Rustlang.Rustup
rustup default stable-msvc
```

Install Microsoft C++ Build Tools.

During installation, select:

- Desktop development with C++
- MSVC toolchain
- Windows SDK

Install or verify Microsoft Edge WebView2 Runtime.

Windows 11 usually already includes WebView2, but install the Evergreen Runtime if Tauri reports missing WebView2.

Restart the terminal after installing Node, Rust, or Visual Studio Build Tools.

## macOS Setup

Install Node.js through a version manager.

Example with `nvm`:

```bash
nvm install
nvm use
corepack enable
corepack prepare pnpm@11.10.0 --activate
```

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup default stable
```

Install Xcode Command Line Tools:

```bash
xcode-select --install
```

If full Xcode is installed, open it once after installation so macOS can finish setup.

## Node And pnpm Setup

Check versions:

```bash
node --version
pnpm --version
```

Expected:

```text
v24.18.0
11.10.0
```

Install dependencies:

```bash
pnpm install
```

Use `pnpm` as the primary package manager because the repository contains `pnpm-lock.yaml`.

## Rust Setup

The repository uses `rust-toolchain.toml`.

When you run Cargo inside the repo, rustup should automatically use the stable toolchain.

Check:

```bash
rustc --version
cargo --version
rustup show
```

On Windows, confirm the MSVC toolchain is active.

## Tauri Prerequisites

Tauri desktop development must run on the host OS.

It requires:

- Node.js and pnpm for the React frontend.
- Rust and Cargo for the Tauri backend.
- Native build tools for the host OS.
- WebView runtime for the host OS.

Do not rely on Devcontainer for `pnpm run tauri:dev`.

Use Devcontainer only for non-GUI checks.

## Run Frontend Dev

```bash
pnpm run dev
```

This starts the Vite dev server.

It does not start the native desktop shell.

## Run Tauri Desktop Dev

```bash
pnpm run tauri:dev
```

This starts the Tauri desktop shell.

Run this on Windows 11 or macOS host, not inside Devcontainer.

## Common Troubleshooting

### `node` or `pnpm` is not recognized

Restart the terminal.

Then check:

```bash
node --version
pnpm --version
```

If pnpm is missing, run:

```bash
corepack enable
corepack prepare pnpm@11.10.0 --activate
```

### `cargo` or `rustc` is not recognized

Restart the terminal after installing Rust.

On Windows, confirm Rust was installed with the MSVC host triple.

### Windows build fails with C++ compiler errors

Install Visual Studio Build Tools and select Desktop development with C++.

Then restart the terminal.

### Windows WebView2 error

Install Microsoft Edge WebView2 Evergreen Runtime.

Windows 11 usually includes it, but development machines can still be missing or misconfigured.

### macOS compiler or signing prompt

Run:

```bash
xcode-select --install
```

If Xcode is installed, open Xcode once and accept required prompts.

### Devcontainer cannot open the Tauri desktop window

This is expected.

The Devcontainer exists for install, typecheck, build, and CI-like checks.

Tauri GUI runtime verification must happen on the host OS.
