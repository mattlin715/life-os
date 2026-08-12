# Windows Founder Dogfooding Package R1

## Status and boundary

This Book One runbook covers one Windows-only, unsigned, local, non-distributed
Founder review package for the current schema-v4 Life OS desktop product. It is
not a Private Alpha, deployment, release, signed build, or schema-v5 activation.

The package changes no React product behavior, Rust runtime behavior, SQLite
schema, provider, ContextPacket, consent policy, or persistence contract.
Production `SCHEMA_VERSION` and the supported startup maximum remain 4.

## Identity and data-profile separation

| Surface | Ordinary development app | Founder dogfooding package |
| --- | --- | --- |
| Product name | `Life OS` | `Life OS Founder Dogfood` |
| Tauri identifier | `com.lifeos.app` | `com.lifeos.founderdogfood` |
| Window title | `Life OS` | `Life OS — Founder Dogfood (Private)` |
| Bundle | normal configured targets | NSIS, current user only |

The SQL plugin resolves the relative `sqlite:life-os.db` beneath Tauri's
application config directory. Tauri derives that directory from the bundle
identifier. Therefore the Founder package resolves a different directory from
the ordinary development app without accepting or constructing a database path
in renderer or package configuration.

On Windows, the expected profile roots are under `%APPDATA%`:

- ordinary development: `%APPDATA%\com.lifeos.app`
- Founder dogfooding: `%APPDATA%\com.lifeos.founderdogfood`

The build and verification scripts do not open either directory or database.
The Founder must perform installation and data-retention observations in a
disposable Windows account or profile.

## Build

From the repository root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-dogfood-package.ps1
```

The script:

1. fails unless the package/source contract and exact changed-path allowlist pass;
2. builds the bundled frontend, then supplies the exact Founder override through Tauri's `TAURI_CONFIG` merge while compiling only the final application binary with Tauri's production `custom-protocol` feature;
3. applies the Windows GUI subsystem and Rust-compatible CRT entry-point flags only to that final binary, restores the caller's environment, and verifies both embedded Founder markers and PE subsystem 2;
4. bundles one unsigned current-user NSIS installer with the same override and revalidates the binary;
5. copies it under ignored `.artifacts/windows-founder-dogfood-r1/<git-sha-prefix>/`;
6. writes and revalidates `manifest.json` from the closed installer bytes;
7. does not install, start, stop, uninstall, distribute, deploy, or release it.

An existing review output directory fails closed. Remove that exact ignored
directory explicitly before intentionally rebuilding.

The build command is deterministic in inputs, selection, output naming, and
validation. This document does not claim byte-identical NSIS output across
machines or toolchain versions. The manifest SHA-256 identifies the exact bytes
that are reviewed.

Founder manual Step 3 invalidated an earlier binary that lacked explicit
`TAURI_CONFIG`: it opened as `Life OS` and attempted the development `localhost`
URL. That evidence is preserved in the workflow archive. The current build
contract requires the override during final-binary compilation because bundling
cannot replace an executable's already embedded Tauri configuration.

## Content-free manifest

`manifest.json` has exactly six fields:

- `applicationVersion`
- `gitSha`
- `buildTarget`
- `artifactFilename`
- `artifactSize`
- `sha256`

It contains no user path, database metadata, Life OS content, secret, runtime
state, timestamp, credential, or machine identity. The Git SHA identifies the
repository baseline; an uncommitted review package is not thereby promoted.

## Automated verification

Focused contract tests run inside the canonical verification path:

```powershell
node --test .\scripts\founder-dogfood-package.node-test.mjs
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

The tests cover identity collision, base-config drift, schema drift, Windows
GUI/console/malformed PE classification, forbidden
override surface, exact manifest keys, filename path leakage, extra metadata,
size mismatch, checksum mismatch, ignored output, and the actual repository
allowlist. They do not replace the Founder install/start/restart/uninstall
review.

## Founder manual review

Keep the installer local and use a disposable Windows account or profile.
Review one step at a time:

1. Confirm the installer says `Life OS Founder Dogfood` and does not replace the development app.
2. Install for the disposable current user and acknowledge the unsigned local-build warning without distributing the file.
3. Launch from the installed shortcut with no repository dev server or console.
4. Complete one local-mock Experience → Evidence → Reflection → completion journey; Pattern and history stay optional.
5. Close normally and restart. Confirm saved schema-v4 records reconstruct and session-only panels reopen closed.
6. Confirm Experience-only export is honestly labelled and no Upgrade, migration, backup, restore, updater, telemetry, or release control exists.
7. Check English, Traditional Chinese, and Japanese in a narrow window and by keyboard.
8. Uninstall. Record whether `%APPDATA%\com.lifeos.founderdogfood` remains, and confirm `%APPDATA%\com.lifeos.app` is untouched. Do not delete either directory as part of this review unless separately authorized.

Manual success is evidence for bounded Founder dogfooding only. It does not
authorize promotion, distribution, deployment, release, schema-v5 activation,
Phase 4, or Android work.
