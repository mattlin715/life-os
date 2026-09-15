import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { access, readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { validateOrdinaryReviewSource } from "./ordinary-schema-v5-review-package.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("ordinary schema-v5 review uses the real ordinary identity with a disposable-only title", async () => {
  assert.deepEqual(await validateOrdinaryReviewSource(root), {
    identifier: "com.lifeos.app",
    title: "Life OS — Ordinary Schema v5 Review (Disposable Only)",
    version: "0.3.0",
  });
});

test("ordinary activation adds no Android generated surface", async () => {
  await assert.rejects(access(path.join(root, "src-tauri", "gen", "android")));
});

test("the main desktop capability grants and records the explicit window close command", async () => {
  const capability = JSON.parse(await readFile(path.join(root, "src-tauri", "capabilities", "default.json"), "utf8"));
  const generated = JSON.parse(await readFile(path.join(root, "src-tauri", "gen", "schemas", "capabilities.json"), "utf8"));

  assert.equal(capability.permissions.filter((permission) => permission === "core:window:allow-close").length, 1);
  assert.deepEqual(generated.default.permissions, capability.permissions);
});
