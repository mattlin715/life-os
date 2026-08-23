import { readFile } from "node:fs/promises";
import path from "node:path";
import { validateWindowsGuiBinary } from "./founder-dogfood-package.mjs";

export const IDENTIFIER = "com.lifeos.app";
export const TITLE = "Life OS — Ordinary Schema v5 Review (Disposable Only)";
export const VERSION = "0.3.0";

function assert(condition, message) { if (!condition) throw new Error(message); }

export async function validateOrdinaryReviewSource(root) {
  const [base, review, cargo, pkg, adapter, rust, lib, viteEnv] = await Promise.all([
    readFile(path.join(root, "src-tauri", "tauri.conf.json"), "utf8").then(JSON.parse),
    readFile(path.join(root, "src-tauri", "tauri.ordinary-schema-v5-review.conf.json"), "utf8").then(JSON.parse),
    readFile(path.join(root, "src-tauri", "Cargo.toml"), "utf8"),
    readFile(path.join(root, "package.json"), "utf8").then(JSON.parse),
    readFile(path.join(root, "src", "shared", "storage", "sqlite", "founderSchemaV5.ts"), "utf8"),
    readFile(path.join(root, "src-tauri", "src", "schema_v5_founder_activation.rs"), "utf8"),
    readFile(path.join(root, "src-tauri", "src", "lib.rs"), "utf8"),
    readFile(path.join(root, "src", "vite-env.d.ts"), "utf8"),
  ]);
  assert(base.identifier === IDENTIFIER && review.identifier === IDENTIFIER, "Ordinary identity drifted.");
  assert(base.version === VERSION && pkg.version === VERSION, "Ordinary application version drifted.");
  assert(review.app.windows?.[0]?.title === TITLE, "Disposable review title drifted.");
  assert(cargo.includes('version = "0.3.0"'), "Rust application version drifted.");
  assert(cargo.includes('default = ["desktop-schema-v5"]'), "Ordinary schema-v5 feature must be a default desktop capability.");
  assert(adapter.includes("isOrdinaryDesktopSchemaV5"), "Ordinary frontend build policy is missing.");
  assert(viteEnv.includes("VITE_LIFE_OS_DESKTOP_SCHEMA_V5"), "Ordinary frontend override declaration is missing.");
  assert(rust.includes(IDENTIFIER) && rust.includes("desktop-schema-v5"), "Ordinary Rust identity policy is missing.");
  assert(lib.includes("schema_v5_founder_activation::inspect_founder_schema_v5_startup"), "Shared activation commands are not registered.");
  return { identifier: IDENTIFIER, title: TITLE, version: VERSION };
}

export async function validateOrdinaryReviewBinary(binaryPath) {
  await validateWindowsGuiBinary(binaryPath);
  const bytes = await readFile(binaryPath);
  // Tauri uses the identifier for bundle and app-data configuration but does
  // not guarantee that the literal survives release optimization. Source and
  // bundle contracts verify identity; the binary must retain the review title
  // plus the shared activation/runtime command surface.
  for (const marker of [TITLE, "inspect_founder_schema_v5_startup", "save_founder_v5_artifacts"]) {
    assert(bytes.includes(Buffer.from(marker, "utf8")), `Ordinary review binary marker missing: ${marker}`);
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === path.resolve(new URL(import.meta.url).pathname.replace(/^\/(.:)/, "$1"))) {
  const [command, target] = process.argv.slice(2);
  const action = command === "verify-source" ? validateOrdinaryReviewSource(path.resolve(target))
    : command === "verify-binary" ? validateOrdinaryReviewBinary(path.resolve(target))
      : Promise.reject(new Error("Usage: ordinary-schema-v5-review-package.mjs <verify-source|verify-binary> <path>"));
  action.then((value) => process.stdout.write(`${JSON.stringify(value ?? { ok: true })}\n`)).catch((error) => { process.stderr.write(`${error.message}\n`); process.exitCode = 1; });
}
