// Bounded, single-writer cancellation/archive recovery. No product authority.
import { createHash, randomUUID } from "node:crypto";
import { closeSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { isDeepStrictEqual } from "node:util";

export const dispositionScope = "cancel-current-sprint-only";
export const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
export const jsonBytes = (value) => `${JSON.stringify(value, null, 2)}\n`;
const pendingName = ".governance-operation";
const fixed = ["CURRENT_MISSION.md", "PRODUCT_REVIEW.md", "ENGINEERING_PLAN.md", "ENGINEERING_REPORT.md", "THEORY_ALIGNMENT_REVIEW.md", "DECISION_REQUIRED.md", "SPRINT_REPORT.md", "WORKFLOW_STATE.json", "EVENTS.jsonl"];
const workflow = (root) => join(root, ".ai", "workflow");
const pending = (root) => join(workflow(root), pendingName);
const encode = (bytes) => ({ sha256: digest(bytes), base64: Buffer.from(bytes).toString("base64") });
function existsSync(path) {
  try { lstatSync(path); return true; }
  catch (error) { if (error.code === "ENOENT") return false; throw error; }
}
function validateIdleImages(images) {
  const state = JSON.parse(images["WORKFLOW_STATE.json"]);
  if (state.status !== "idle" || state.sprint_id !== null || state.state_revision !== 0 || state.human_approval_required !== false
    || state.decision_resolution !== null || state.active_decision_ids?.length !== 0 || state.blocked_phase !== null || state.resume_phase !== null
    || state.sprint_disposition !== undefined || state.terminal_follow_up !== undefined
    || state.verification?.repository_verify?.status !== "not_run" || state.verification?.manual_ui?.status !== "not_run"
    || state.verification.manual_ui.scoped_evidence !== undefined || Buffer.from(images["EVENTS.jsonl"]).length !== 0) throw new Error("Idle candidate must not inherit sprint decisions, PASS or authority");
}
function validateCancellationImages(before, after, prior, result) {
  if (!isDeepStrictEqual(result.sprint_disposition.checkpoint, prior)) throw new Error("Cancellation checkpoint does not match before state");
  const prefix = before["EVENTS.jsonl"];
  const events = after["EVENTS.jsonl"];
  if (!events.subarray(0, prefix.length).equals(prefix)) throw new Error("Cancellation events do not preserve before prefix");
  const separator = prefix.length && prefix.at(-1) !== 0x0a ? "\n" : "";
  const appended = events.subarray(prefix.length).toString("utf8");
  const line = appended.slice(separator.length);
  if (!appended.startsWith(separator) || !line.endsWith("\n") || line.slice(0, -1).includes("\n")) throw new Error("Cancellation must append exactly one event line");
  const event = JSON.parse(line);
  if (event.event_type !== "sprint_cancelled" || event.from_status !== prior.status || event.to_status !== "cancelled"
    || event.sprint_id !== prior.sprint_id || event.sequence !== prior.state_revision + 1
    || event.previous_event_hash !== prior.last_event_hash) throw new Error("Invalid cancellation append event");
  // The existing candidate validator below still checks the complete event
  // chain, disposition authority and resulting state projection.
}
function decode(value) {
  if (!value || typeof value.base64 !== "string" || !/^[a-f0-9]{64}$/.test(value.sha256 ?? "")) throw new Error("Invalid recovery image");
  const bytes = Buffer.from(value.base64, "base64");
  if (bytes.toString("base64") !== value.base64 || digest(bytes) !== value.sha256) throw new Error("Recovery image hash mismatch");
  return bytes;
}
function file(path) {
  if (!lstatSync(path).isFile()) throw new Error(`Expected regular file: ${path}`);
  return readFileSync(path);
}
function durableFile(path, bytes) {
  const fd = openSync(path, "wx");
  try { writeFileSync(fd, bytes); fsyncSync(fd); } finally { closeSync(fd); }
}
export function governancePoint(name) {
  if (process.env.NODE_ENV !== "test") return;
  if (process.env.LIFE_OS_GOVERNANCE_FAIL_AT === name) throw new Error(`Injected governance failure: ${name}`);
  if (process.env.LIFE_OS_GOVERNANCE_KILL_AT === name) process.kill(process.pid, "SIGKILL");
}
export function assertNoGovernanceOperation(root) {
  const names = readdirSync(workflow(root));
  if (names.some((name) => name === pendingName || name.startsWith(".governance-preparing-"))) throw new Error("Governance recovery required; mutations/start refused");
}
export function pendingGovernance(root) {
  try { assertNoGovernanceOperation(root); return false; } catch { return true; }
}
export function controlImages(root) {
  return Object.fromEntries([...fixed, "WORKFLOW_CONTRACT.json"].map((name) => [name, file(join(workflow(root), name))]));
}
export function validateImages(images, validate) {
  const root = mkdtempSync(join(tmpdir(), "life-os-governance-candidate-"));
  try {
    mkdirSync(workflow(root), { recursive: true });
    for (const [name, bytes] of Object.entries(images)) writeFileSync(join(workflow(root), name), bytes);
    const errors = validate(root, { checkRepositoryFreshness: false });
    if (errors.length) throw new Error(`Invalid governance candidate:\n${errors.join("\n")}`);
  } finally { rmSync(root, { recursive: true, force: true }); }
}
function safeDirectoryIfPresent(path) {
  if (existsSync(path) && !lstatSync(path).isDirectory()) throw new Error(`Unknown directory/symlink: ${path}`);
}
function directoryImages(path, images, partial = false) {
  if (!lstatSync(path).isDirectory()) throw new Error(`Expected regular directory: ${path}`);
  for (const name of readdirSync(path)) {
    if (!(name in images) || digest(file(join(path, name))) !== images[name].sha256) throw new Error(`Unknown archive file/hash: ${name}`);
  }
  if (!partial && readdirSync(path).length !== Object.keys(images).length) throw new Error("Incomplete archive");
}
function assertKnownControlEntries(root) {
  const allowed = new Set([...fixed, "WORKFLOW_CONTRACT.json", "WORKFLOW.md", "WORKFLOW_EVALUATION.md", "PREPARED_RECOVERY_PLATFORM_BOUNDARY_R1_AUTHORIZATION.md", "HISTORY", ".governance-receipts", pendingName]);
  for (const name of readdirSync(workflow(root))) if (!allowed.has(name)) throw new Error(`Unknown control-plane entry: ${name}`);
}
export function prepareGovernance(root, kind, after, archiveFiles, validate) {
  assertNoGovernanceOperation(root);
  assertKnownControlEntries(root);
  safeDirectoryIfPresent(join(workflow(root), "HISTORY"));
  safeDirectoryIfPresent(join(workflow(root), ".governance-receipts"));
  const before = controlImages(root);
  if (!['cancel', 'archive'].includes(kind)) throw new Error("Unknown governance operation");
  const names = kind === 'cancel' ? ["EVENTS.jsonl", "WORKFLOW_STATE.json"] : fixed;
  const next = { ...before, ...after };
  if (kind === "archive") validateIdleImages(next);
  validateImages(next, validate);
  if (archiveFiles) validateImages(archiveFiles, validate);
  const state = JSON.parse(before["WORKFLOW_STATE.json"]);
  const record = {
    format: "life-os/governance-operation-v1", id: randomUUID(), kind,
    sprint_id: state.sprint_id, sequence: state.state_revision,
    before: Object.fromEntries(Object.entries(before).map(([name, bytes]) => [name, encode(bytes)])),
    after: Object.fromEntries(names.map((name) => [name, encode(next[name])])),
    archive: archiveFiles ? Object.fromEntries(Object.entries(archiveFiles).map(([name, bytes]) => [name, encode(bytes)])) : null,
  };
  if (kind === "archive" && existsSync(join(workflow(root), "HISTORY", record.sprint_id))) throw new Error(`Archive already exists: ${record.sprint_id}`);
  const preparing = join(workflow(root), `.governance-preparing-${record.id}`);
  mkdirSync(preparing);
  durableFile(join(preparing, "operation.json"), jsonBytes(record));
  renameSync(preparing, pending(root));
  governancePoint("prepared");
  return digest(jsonBytes(record));
}
export function recoverGovernance(root, expectedHash, validate) {
  const operation = pending(root);
  if (!existsSync(operation)) throw new Error("No pending governance operation");
  if (readdirSync(workflow(root)).some((name) => name.startsWith(".governance-preparing-"))) throw new Error("Conflicting preparing operation; preserve for review");
  assertKnownControlEntries(root);
  if (!lstatSync(operation).isDirectory()) throw new Error("Invalid operation directory");
  const raw = file(join(operation, "operation.json"));
  if (!/^[a-f0-9]{64}$/.test(expectedHash ?? "") || digest(raw) !== expectedHash) throw new Error("--operation-sha256 must match the pending record");
  const r = JSON.parse(raw);
  if (r.format !== "life-os/governance-operation-v1" || !/^[a-f0-9-]{36}$/.test(r.id ?? "") || !/^[a-z0-9][a-z0-9-]{2,79}$/.test(r.sprint_id ?? "") || !['cancel','archive'].includes(r.kind)) throw new Error("Unknown operation record");
  safeDirectoryIfPresent(join(workflow(root), "HISTORY"));
  const receipts = join(workflow(root), ".governance-receipts");
  safeDirectoryIfPresent(receipts);
  if (existsSync(join(receipts, r.id))) throw new Error("Recovery receipt conflict; preserved pending operation");
  const names = r.kind === "cancel" ? ["EVENTS.jsonl", "WORKFLOW_STATE.json"] : fixed;
  if (JSON.stringify(Object.keys(r.before).sort()) !== JSON.stringify([...fixed, "WORKFLOW_CONTRACT.json"].sort()) || JSON.stringify(Object.keys(r.after).sort()) !== JSON.stringify([...names].sort())) throw new Error("Unknown operation file list");
  const before = Object.fromEntries(Object.entries(r.before).map(([name, value]) => [name, decode(value)]));
  const after = { ...before, ...Object.fromEntries(Object.entries(r.after).map(([name, value]) => [name, decode(value)])) };
  const prior = JSON.parse(before["WORKFLOW_STATE.json"]);
  if (prior.sprint_id !== r.sprint_id || prior.state_revision !== r.sequence) throw new Error("Operation checkpoint mismatch");
  const result = JSON.parse(after["WORKFLOW_STATE.json"]);
  if (r.kind === "cancel" && (prior.status !== "human_decision_required" || result.status !== "cancelled" || !result.sprint_disposition)) throw new Error("Invalid cancellation recovery images");
  if (r.kind === "archive" && (!["completed", "failed", "cancelled"].includes(prior.status) || result.status !== "idle")) throw new Error("Invalid archive recovery images");
  if (r.kind === "cancel") validateCancellationImages(before, after, prior, result);
  if (r.kind === "archive") validateIdleImages(after);
  validateImages(before, validate); validateImages(after, validate);
  for (const name of readdirSync(operation)) if (!["operation.json", "replacement", "archive"].includes(name)) throw new Error(`Unknown operation file: ${name}`);
  if (existsSync(join(operation, "replacement"))) {
    const hash = digest(file(join(operation, "replacement")));
    if (!Object.values(r.after).some((value) => value.sha256 === hash)) throw new Error("Unknown replacement hash");
  }
  // Check EVERY current file before publishing or writing any one of them.
  for (const [name, value] of Object.entries(r.before)) {
    const hash = digest(file(join(workflow(root), name)));
    if (hash !== value.sha256 && hash !== r.after[name]?.sha256) throw new Error(`Unknown current file/hash: ${name}`);
  }
  const stage = join(operation, "archive");
  const destination = join(workflow(root), "HISTORY", r.sprint_id);
  if (r.kind === "archive") {
    if (!r.archive) throw new Error("Missing archive images");
    const archive = Object.fromEntries(Object.entries(r.archive).map(([name, value]) => [name, decode(value)]));
    // Restrict archive filenames before using them as filesystem paths.
    const allowed = [...fixed, "WORKFLOW_CONTRACT.json", "ARCHIVE_MANIFEST.json", ...(prior.sprint_disposition ? ["SPRINT_DISPOSITION.json", "CANCELLATION_CHECKPOINT.json", "SPRINT_DISPOSITION_EVIDENCE.json"] : [])];
    if (Object.keys(archive).some((name) => !allowed.includes(name))) throw new Error("Unknown archive image filename");
    for (const [name, bytes] of Object.entries(before)) if (!archive[name]?.equals(bytes)) throw new Error(`Archive checkpoint mismatch: ${name}`);
    if (prior.sprint_disposition) {
      for (const [name, bytes] of Object.entries({ "SPRINT_DISPOSITION.json": jsonBytes(prior.sprint_disposition), "CANCELLATION_CHECKPOINT.json": jsonBytes(prior.sprint_disposition.checkpoint), "SPRINT_DISPOSITION_EVIDENCE.json": prior.sprint_disposition.evidence.content })) {
        if (!archive[name]?.equals(Buffer.from(bytes))) throw new Error(`Archived disposition mismatch: ${name}`);
      }
    }
    const manifest = JSON.parse(archive["ARCHIVE_MANIFEST.json"]);
    const expectedFiles = Object.keys(archive).filter((name) => name !== "ARCHIVE_MANIFEST.json").sort();
    if (manifest.schema_version !== "2.0" || manifest.sprint_id !== r.sprint_id || manifest.terminal_status !== prior.status || JSON.stringify([...manifest.files].sort()) !== JSON.stringify(expectedFiles)) throw new Error("Archive manifest mismatch");
    for (const name of expectedFiles) if (manifest.hashes[name] !== digest(archive[name])) throw new Error(`Archive manifest hash mismatch: ${name}`);
    validateImages(archive, validate);
    if (existsSync(stage)) directoryImages(stage, r.archive, true);
    if (existsSync(destination)) {
      if (existsSync(stage)) throw new Error("Conflicting archive publication");
      directoryImages(destination, r.archive);
    } else {
      // No reset may have happened before a verified archive publication.
      for (const name of fixed) if (!file(join(workflow(root), name)).equals(before[name])) throw new Error("Reset without published archive");
      mkdirSync(stage, { recursive: true });
      for (const [name, bytes] of Object.entries(archive)) {
        if (!existsSync(join(stage, name))) durableFile(join(stage, name), bytes);
        governancePoint(`copy:${name}`);
      }
      directoryImages(stage, r.archive);
      governancePoint("before-publish");
      mkdirSync(join(workflow(root), "HISTORY"), { recursive: true });
      renameSync(stage, destination);
      governancePoint("published");
      directoryImages(destination, r.archive);
    }
  } else if (r.archive !== null || existsSync(stage)) throw new Error("Unexpected cancellation archive");
  for (const name of names) {
    governancePoint(`before-write:${name}`);
    const target = join(workflow(root), name);
    if (!file(target).equals(after[name])) {
      const replacement = join(operation, "replacement");
      if (existsSync(replacement)) rmSync(replacement); // only a preflight-verified known image
      durableFile(replacement, after[name]);
      governancePoint(`before-rename:${name}`);
      renameSync(replacement, target);
    }
    governancePoint(`write:${name}`);
  }
  for (const [name, bytes] of Object.entries(after)) if (!file(join(workflow(root), name)).equals(bytes)) throw new Error(`Post-write mismatch: ${name}`);
  validateImages(controlImages(root), validate);
  governancePoint("verified");
  mkdirSync(receipts, { recursive: true });
  if (existsSync(join(receipts, r.id))) throw new Error("Recovery receipt conflict; preserved pending operation");
  renameSync(operation, join(receipts, r.id)); // retain all recovery images, never delete the only copy
}
