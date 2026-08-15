import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { validateCandidateSource } from "./founder-schema-v5-candidate-package.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

test("isolated Founder schema-v5 candidate keeps ordinary identity and schema v4", async () => {
  const result = await validateCandidateSource(root);
  assert.deepEqual(result, {
    identifier: "com.lifeos.founderdogfood",
    ordinaryIdentifier: "com.lifeos.app",
    ordinarySchemaVersion: 4,
  });
});
