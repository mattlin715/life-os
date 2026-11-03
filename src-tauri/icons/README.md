# Tauri compile-time icon inputs

This note preserves asset provenance and reproduction steps. It does not change
product identity, packaging policy, or any acceptance record.

## Source and extraction

`icon.ico` is the existing repository design, introduced by
`014ed5d7ce4e18f8908b59a7ea8889665e8f1e89` on 2026-07-07. The 256x256 PNG
is copied byte-for-byte from its sixth directory entry (zero-based entry 5):
offset 13639, length 2239 bytes. No resampling, redesign, or image dependency
is involved. Both files must be tracked as ordinary Git blobs.

From the repository root, using the repository's Node.js 24 toolchain:

```javascript
// Save as a temporary .cjs file or execute with node -e.
const fs = require("node:fs");
const ico = fs.readFileSync("src-tauri/icons/icon.ico");
const entry = 6 + 5 * 16;
const length = ico.readUInt32LE(entry + 8);
const offset = ico.readUInt32LE(entry + 12);
fs.writeFileSync("src-tauri/icons/icon.png", ico.subarray(offset, offset + length));
```

SHA-256:

- ICO: `62d764181ef9137aec875f345345daef4bd398fa828f80cdb35bded563ad2ade`
- PNG: `1605b62ebdffb0aec44b2fcdce15ab1098c09ca91807bebbf7c430804b961542`

The PNG is 256x256, 8-bit RGBA, non-interlaced. The regression checks PNG
signature, chunk boundaries/CRC, IHDR, decompressed scanline size/filter bytes,
and exact equality with the existing embedded ICO payload.

## Why a clean Linux checkout needs it

At baseline `e8db8f0f8e59aed7b321a8792a139b53e7328609`, only the ICO was
tracked. There were no tracked Git attributes/LFS rules or repository ignore
rules affecting these assets. This is a pre-existing input gap, not introduced
by the Reflection correction.

`src-tauri/Cargo.lock` locks Tauri 2.11.5 and tauri-codegen 2.6.3. In that
codegen version, `src/context.rs` selects an ICO for Windows if it exists,
falling back to PNG only if it does not. Other targets select PNG directly.
`find_icon` uses the first matching `bundle.icon` entry or the fallback
`icons/icon.png`. The current config has no `bundle.icon` override.
`src/image.rs` (`CachedIcon::new_png`) requires RGBA output.

Therefore Windows host verification can succeed using the tracked ICO while
Linux fails. This does not establish that any host used an ignored PNG.
`build.rs` only calls `tauri_build::build()`. The Check workflow checks out
source, installs system dependencies, then invokes Cargo directly; no icon
generation step exists or is needed once the input is tracked.

The original [Check run, attempt 1](https://github.com/mattlin715/life-os/actions/runs/41511241254)
failed in Rust tests with exit 101 for the missing PNG. Rust check and legacy
schema-v4 compatibility check were skipped, not passed.

## Regression and verification boundary

```text
pnpm exec vitest run scripts/tauri-icon-input.test.mjs
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

The regression is discovered by the existing `test:run` command in both the
canonical verifier and Frontend Checks; no workflow change is required.
It checks exact-case Git index paths, reads actual Git blobs, and compares
working asset bytes against them. A merely local/untracked PNG cannot pass.
Negative cases cover absent/untracked entries, wrong case, non-PNG/LFS pointer,
truncation, corruption, and non-RGBA data. The index supports pre-commit checks;
after commit, a fresh clone of the exact commit verifies committed inputs.

Windows checks do not establish Ubuntu success. The PR's existing Check run
must execute Frontend Checks, Rust tests, Rust check, and legacy compatibility
successfully. This repair does not rebuild or reaccept the old Windows
candidate, authorize manual journeys, resume Android/036, or authorize release.
No ADR or Book Zero change is required for restoring an existing build input.
