import { readFile } from "node:fs/promises";
import path from "node:path";
import { validateWindowsGuiBinary } from "./founder-dogfood-package.mjs";

export const IDENTIFIER = "com.lifeos.founderdogfood";
export const TITLE = "Life OS — Founder Schema v5 Candidate (Private)";

function assert(condition, message) { if (!condition) throw new Error(message); }

export async function validateCandidateSource(root) {
  const [base, config, cargo, adapter, ordinary] = await Promise.all([
    readFile(path.join(root, "src-tauri", "tauri.conf.json"), "utf8").then(JSON.parse),
    readFile(path.join(root, "src-tauri", "tauri.founder-dogfood-v5-candidate.conf.json"), "utf8").then(JSON.parse),
    readFile(path.join(root, "src-tauri", "Cargo.toml"), "utf8"),
    readFile(path.join(root, "src", "shared", "storage", "sqlite", "founderSchemaV5.ts"), "utf8"),
    readFile(path.join(root, "src-tauri", "src", "sqlite.rs"), "utf8"),
  ]);
  assert(base.identifier === "com.lifeos.app", "Ordinary identity drifted.");
  assert(config.identifier === IDENTIFIER, "Candidate identity drifted.");
  assert(config.app.windows?.[0]?.title === TITLE, "Candidate private title drifted.");
  assert(cargo.includes("founder-schema-v5 = []"), "Candidate Cargo feature is missing.");
  assert(adapter.includes('VITE_LIFE_OS_FOUNDER_SCHEMA_V5 === "1"'), "Candidate build-time renderer gate is missing.");
  assert(/const\s+SCHEMA_VERSION\s*:\s*i64\s*=\s*4\s*;/.test(ordinary), "Ordinary SCHEMA_VERSION must remain 4.");
  return { identifier: config.identifier, ordinaryIdentifier: base.identifier, ordinarySchemaVersion: 4 };
}

export async function validateCandidateBinary(binaryPath) {
  await validateWindowsGuiBinary(binaryPath);
  const bytes = await readFile(binaryPath);
  for (const marker of [IDENTIFIER, TITLE, "inspect_founder_schema_v5_startup", "save_founder_v5_artifacts"]) {
    assert(bytes.includes(Buffer.from(marker, "utf8")), `Candidate binary marker missing: ${marker}`);
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === path.resolve(new URL(import.meta.url).pathname.replace(/^\/(.:)/, "$1"))) {
  const [command, target] = process.argv.slice(2);
  const action = command === "verify-source" ? validateCandidateSource(path.resolve(target))
    : command === "verify-binary" ? validateCandidateBinary(path.resolve(target))
      : Promise.reject(new Error("Usage: founder-schema-v5-candidate-package.mjs <verify-source|verify-binary> <path>"));
  action.then((value) => process.stdout.write(`${JSON.stringify(value ?? { ok: true })}\n`)).catch((error) => { process.stderr.write(`${error.message}\n`); process.exitCode = 1; });
}
