import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { execFileSync } from "node:child_process";
import test from "node:test";
import { fileURLToPath } from "node:url";
import path from "node:path";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (p) => readFile(path.join(root, p), "utf8");

test("M2-B frontend has an exact narrow reviewable surface", () => {
  const files = execFileSync("git", ["ls-files", "--cached", "--others", "--exclude-standard", "--", "src/android-m2b"], { cwd: root, encoding: "utf8", windowsHide: true }).trim().split(/\r?\n/).filter(Boolean).sort();
  assert.deepEqual(files, ["AndroidM2BApp.test.tsx", "AndroidM2BApp.tsx", "android-m2b.css", "androidM2BStore.ts"].map((f) => "src/android-m2b/" + f).sort());
});
test("fixed temporary identity, backup exclusions and no runtime permission", async () => {
  const config = JSON.parse(await read("src-tauri/tauri.android.conf.json"));
  assert.equal(config.identifier, "com.lifeos.review.m2c"); assert.deepEqual(config.app.security.capabilities, ["android-m2c"]);
  const caps = JSON.parse(await read("src-tauri/capabilities/android-m2b.json")); assert.deepEqual(caps.permissions, ["core:default"]);
  const manifest = await read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");
  assert.doesNotMatch(manifest, /uses-permission|FileProvider/); assert.match(manifest, /allowBackup="false"/); assert.match(manifest, /usesCleartextTraffic="false"/);
  assert.match(await read("src-tauri/gen/android/app/build.gradle.kts"), /applicationId = "com\.lifeos\.review\.m2c"/);
});
test("mode selects M2-B before desktop while preserving old mobile entries", async () => {
  const main = await read("src/main.tsx"); assert.ok(main.indexOf("VITE_LIFE_OS_ANDROID_M2B") < main.indexOf('import("./app/App")'));
  for (const prefix of ["M2A", "M1", "FEASIBILITY_M0"]) assert.match(main, new RegExp("VITE_LIFE_OS_ANDROID_" + prefix));
  assert.match(await read("vite.config.ts"), /mode === "android-m2b"/);
});
test("direct initializer is shared with fixed identity-bound storage profiles", async () => {
  const shared = await read("src-tauri/src/android_m2a.rs"); const backend = await read("src-tauri/src/android_m2b.rs");
  assert.match(shared, /paths\.application_id/); assert.match(shared, /m2b_paths_at/); assert.match(shared, /identifier != "com\.lifeos\.review\.m2b"/);
  assert.match(backend, /with_m2b_storage/); assert.doesNotMatch(backend.split("#[cfg(test)]")[0], /EMPTY_V4|migrate_disposable|INSERT|DELETE FROM|UPDATE source|com\.lifeos\.app|println!|dbg!/);
  const runtime = await read("src-tauri/src/schema_v5_runtime.rs"); assert.match(runtime, /mutate_experience_direct_fresh/); assert.match(runtime, /execute_experience_direct\(path, command/);
});
test("immutable exact request identities and content-free acknowledgements", async () => {
  const backend = await read("src-tauri/src/android_m2b.rs"); const adapter = await read("src/android-m2b/androidM2BStore.ts");
  assert.match(backend, /request_id != request_identity/); assert.match(backend, /reconcile_direct_mutation/); assert.match(backend, /refuse_non_synthetic_dependencies/);
  const ack = backend.match(/struct MutationAcknowledgement \{([^}]+)\}/s)[1]; assert.doesNotMatch(ack, /body|text|entry|payload/);
  assert.match(adapter, /Object\.freeze/); assert.match(adapter, /expectedRevisionId/); assert.match(adapter, /crypto\.subtle\.digest/);
});
test("explicit confirmation, safe retry and optional disclosure without provider features", async () => {
  const ui = await read("src/android-m2b/AndroidM2BApp.tsx");
  for (const token of ["m2b-cancel-edit", "m2b-cancel-delete", "m2b-confirm-delete", "m2b-retry", "committedNotCurrent"]) assert.ok(ui.includes(token));
  assert.match(ui, /request \? request : await lifecycleRequest/); assert.match(ui, /setSelected\(null\); setEntries\(\[\]\); setEditText/);
  assert.doesNotMatch(ui, /localStorage|console\.|fetch\(|api\.openai|generativelanguage/); assert.match(ui, /<details/);
});
test("packaging preserves accepted APK and pins separate debug artifact", async () => {
  const build = await read("scripts/android-m2b.ps1"); assert.match(build, /preserved-m2a/); assert.match(build, /85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb/);
  assert.match(build, /review\.apk/); assert.match(build, /--debug --target x86_64/); assert.doesNotMatch(build, /setx|SetEnvironmentVariable|--release/);
});
test("M2-B contract checks are on canonical verification path", async () => {
  assert.match(await read("scripts/verify.ps1"), /node --test scripts\/android-m2b-contract\.node-test\.mjs/);
});

test("native final supplement is exact-owned and cannot replay or reset failed state", async () => {
  const native = await read("scripts/android-m2b-native-review.ps1");
  const supplement = native.slice(native.indexOf("if ($ResumeFinal) {"), native.indexOf("  $devices =", native.indexOf("    Write-Host ($results")));
  for (const binding of ["owner.run_id", "owner.avd", "owner.serial", "owner.package", "owner.apk_sha256", "final-fresh.db", "Pending-Absent", "$supplement-restart.md"]) assert.ok(supplement.includes(binding));
  assert.doesNotMatch(supplement, /Reset-Owned|pm','clear|Adb @\('install|Install-Fixture|--action','retry/);
  assert.match(native, /FAILED-supplement-\$SupplementId\.txt/);
  assert.match(await read("scripts/android-m2b-cdp-probe.mjs"), /"ready state", 120000/);
});

test("copy correction keeps two separately bound disclosures and native three-language coverage", async () => {
  const ui = await read("src/android-m2b/AndroidM2BApp.tsx");
  assert.equal((ui.match(/revisionDetails:/g) ?? []).length, 3);
  assert.equal((ui.match(/buildDetails:/g) ?? []).length, 3);
  assert.doesNotMatch(ui, /\{c\.details\}/);
  assert.match(ui, /m2b-revision-details"><summary>\{c\.revisionDetails\}/);
  assert.match(ui, /m2b-build-details"><summary>\{c\.buildDetails\}/);
  const dialog = ui.slice(ui.indexOf('{confirming ? <div role="dialog"'), ui.indexOf(' : null}', ui.indexOf('{confirming ? <div role="dialog"')));
  assert.match(dialog, /\{c\.confirm\}/); assert.doesNotMatch(dialog, /c\.retention|c\.limits/);
  assert.match(await read("scripts/android-m2b-cdp-probe.mjs"), /action === "copy-review"/);
  assert.match(await read("scripts/android-m2b-native-review.ps1"), /Probe @\('--action','copy-review'\)/);
});

test("status correction is presentation-only, contextual, and keeps all abnormal outcomes visible", async () => {
  const ui = (await read("src/android-m2b/AndroidM2BApp.tsx")).replace(/\r\n/g, "\n");
  const unchangedRuntime = ui.slice(ui.indexOf("export function AndroidM2BApp"), ui.indexOf("  if (storage !=="));
  assert.equal(createHash("sha256").update(unchangedRuntime).digest("hex"), "0b996c7e68678b72268a9c68ed845da2ef0d87bea9d93790e31531fd58bb601b");
  assert.match(ui, /kind="new" visible=\{!!draft\.trim\(\) && !locked\}/);
  assert.match(ui, /kind="edit" visible=\{editText !== selected\.entry\.body && !locked\}/);
  assert.ok(ui.indexOf("<OperationStatus state={state}") < ui.indexOf('<h2>{c.draft}</h2>'));
  const operation = ui.slice(ui.indexOf("export function OperationStatus"), ui.indexOf("export function DraftStatus"));
  for (const state of ["saving", "saved", "deleted", "conflict", "failed"]) assert.ok(operation.includes(`state === "${state}"`));
  assert.doesNotMatch(operation, /c\.unsaved|setState|setTimeout|store\.|invoke/);
  assert.match(operation, /revisionNumber > 1 \? c\.changesSaved : c\.saved/);
  assert.match(await read("scripts/android-m2b-cdp-probe.mjs"), /action === "status-review"/);
  assert.match(await read("scripts/android-m2b-native-review.ps1"), /Probe @\('--action','status-review','--locale',\$locale\)/);
});
