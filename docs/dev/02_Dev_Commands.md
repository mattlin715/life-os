---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/dev/01_Environment_Setup.md
referenced_by: []
---

# 02 Dev Commands

## Install

```bash
pnpm install
```

Installs JavaScript dependencies from `pnpm-lock.yaml`.

## Frontend Dev Server

```bash
pnpm run dev
```

Starts Vite for frontend-only development.

## Typecheck

```bash
pnpm run typecheck
```

Runs TypeScript without emitting files.

## Build Frontend

```bash
pnpm run build
```

Runs TypeScript and builds the Vite frontend.

This does not create a desktop installer.

## Tauri Desktop Dev

```bash
pnpm run tauri:dev
```

Starts the native Tauri desktop shell.

Run this on the host OS, not inside Devcontainer.

## Tauri Desktop Build

```bash
pnpm run tauri:build
```

Builds the Tauri desktop app for the current host OS.

This requires native build tools and is intentionally not part of the initial CI skeleton.

## Command Boundary

Use Devcontainer or CI for:

- `pnpm install`
- `pnpm run typecheck`
- `pnpm run build`

Use host OS for:

- `pnpm run tauri:dev`
- `pnpm run tauri:build`
