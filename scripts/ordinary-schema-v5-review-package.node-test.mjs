import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { readFile } from "node:fs/promises";
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

test("ordinary desktop identity remains separate from the current disposable Android M2-B identity", async () => {
  const desktop = JSON.parse(await readFile(path.join(root, "src-tauri", "tauri.conf.json"), "utf8"));
  const android = JSON.parse(await readFile(path.join(root, "src-tauri", "tauri.android.conf.json"), "utf8"));

  assert.equal(desktop.identifier, "com.lifeos.app");
  assert.equal(desktop.productName, "Life OS");
  assert.equal(android.identifier, "com.lifeos.review.m2b");
  assert.equal(android.productName, "Life OS Android M2-B Synthetic Lifecycle Review");
});

test("the main desktop capability grants and records the explicit window close command", async () => {
  const capability = JSON.parse(await readFile(path.join(root, "src-tauri", "capabilities", "default.json"), "utf8"));
  const generated = JSON.parse(await readFile(path.join(root, "src-tauri", "gen", "schemas", "capabilities.json"), "utf8"));

  assert.equal(capability.permissions.filter((permission) => permission === "core:window:allow-close").length, 1);
  assert.deepEqual(generated.default.permissions, capability.permissions);
});
