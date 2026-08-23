import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { access } from "node:fs/promises";
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
