// Read-only comparison of the fixed base and candidate validators on archives.
import { execFileSync } from "node:child_process";
import { cpSync, mkdirSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
const source = resolve(process.argv[2] ?? resolve(import.meta.dirname, "../../.."));
const output = resolve(process.argv[3] ?? join(import.meta.dirname, "legacy-archives.json"));
const scratch = mkdtempSync(join(tmpdir(), "life-os-r3-gov01-archive-check-"));
try {
  const base = join(scratch, "base-workflow.mjs");
  writeFileSync(base, execFileSync("git", ["show", "a9433c4e3a979110e5d159d60e2bfea6a536a1f2:scripts/ai-workflow.mjs"], { cwd: source }));
  const { validateWorkflow: before } = await import(pathToFileURL(base));
  const { validateWorkflow: after } = await import(pathToFileURL(join(source, "scripts/ai-workflow.mjs")));
  const results = [];
  for (const entry of readdirSync(join(source, ".ai/workflow/HISTORY"), { withFileTypes: true }).filter(e => e.isDirectory() && !e.name.startsWith("."))) {
    const root = join(scratch, entry.name);
    mkdirSync(join(root, ".ai"), { recursive: true });
    cpSync(join(source, ".ai/workflow/HISTORY", entry.name), join(root, ".ai/workflow"), { recursive: true });
    const baseline = before(root), candidate = after(root);
    results.push({ archive: entry.name, baseline, candidate, identical: JSON.stringify(baseline) === JSON.stringify(candidate) });
  }
  const result = { archives: results.length, all_verdicts_unchanged: results.every(r => r.identical), baseline_valid: results.filter(r => !r.baseline.length).length, results };
  writeFileSync(output, `${JSON.stringify(result, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ archives: result.archives, all_verdicts_unchanged: result.all_verdicts_unchanged, baseline_valid: result.baseline_valid })}\n`);
  if (!result.all_verdicts_unchanged || result.baseline_valid !== result.archives) process.exitCode = 1;
} finally { rmSync(scratch, { recursive: true, force: true }); }
