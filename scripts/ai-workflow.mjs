#!/usr/bin/env node

import { createHash, randomUUID } from "node:crypto";
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const defaultRoot = resolve(scriptDirectory, "..");
const mutableWorkflowPaths = new Set([
  ".ai/workflow/WORKFLOW_STATE.json",
  ".ai/workflow/EVENTS.jsonl",
  ".ai/workflow/CURRENT_MISSION.md",
  ".ai/workflow/PRODUCT_REVIEW.md",
  ".ai/workflow/ENGINEERING_PLAN.md",
  ".ai/workflow/ENGINEERING_REPORT.md",
  ".ai/workflow/THEORY_ALIGNMENT_REVIEW.md",
  ".ai/workflow/DECISION_REQUIRED.md",
  ".ai/workflow/SPRINT_REPORT.md",
]);

function isMutableWorkflowPath(path) {
  return mutableWorkflowPaths.has(path.replaceAll("\\", "/"));
}

function stable(value) {
  if (Array.isArray(value)) return `[${value.map(stable).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.entries(value).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)).map(([key, nested]) => `${JSON.stringify(key)}:${stable(nested)}`).join(",")}}`;
  }
  return JSON.stringify(value);
}

function sha256(value) {
  return createHash("sha256").update(value).digest("hex");
}

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function writeAtomic(path, content) {
  mkdirSync(dirname(path), { recursive: true });
  const temporary = `${path}.tmp-${process.pid}-${randomUUID()}`;
  writeFileSync(temporary, content);
  renameSync(temporary, path);
}

function runGit(root, args) {
  return execFileSync("git", args, { cwd: root, encoding: "utf8", windowsHide: true });
}

export function workflowPaths(root = defaultRoot) {
  const workflow = join(root, ".ai", "workflow");
  return {
    root,
    workflow,
    contract: join(workflow, "WORKFLOW_CONTRACT.json"),
    state: join(workflow, "WORKFLOW_STATE.json"),
    events: join(workflow, "EVENTS.jsonl"),
    history: join(workflow, "HISTORY"),
  };
}

export function loadContract(root = defaultRoot) {
  return readJson(workflowPaths(root).contract);
}

export function repositorySnapshot(root = defaultRoot) {
  const head = runGit(root, ["rev-parse", "HEAD"]).trim();
  const branch = runGit(root, ["branch", "--show-current"]).trim();
  const tracked = runGit(root, ["diff", "--binary", "HEAD", "--", ".", ...[...mutableWorkflowPaths].map((path) => `:(exclude)${path}`)]);
  const untracked = runGit(root, ["ls-files", "--others", "--exclude-standard"])
    .split(/\r?\n/)
    .filter(Boolean)
    .map((path) => path.replaceAll("\\", "/"))
    .filter((path) => !isMutableWorkflowPath(path))
    .sort();
  const digest = createHash("sha256");
  digest.update("life-os/worktree-v1\0");
  digest.update(tracked);
  for (const path of untracked) {
    digest.update(`\0${path}\0`);
    digest.update(readFileSync(join(root, path)));
  }
  return { head, branch, working_tree_digest: digest.digest("hex") };
}

export function readEvents(root = defaultRoot) {
  const path = workflowPaths(root).events;
  if (!existsSync(path)) return [];
  return readFileSync(path, "utf8").split(/\r?\n/).filter(Boolean).map((line, index) => {
    try { return JSON.parse(line); }
    catch (error) { throw new Error(`EVENTS.jsonl line ${index + 1}: ${error.message}`); }
  });
}

function markdownStatus(content) {
  return content.match(/^Status:\s*`?([a-z_]+)`?\s*$/m)?.[1] ?? null;
}

function hasRequiredPlaceholders(content, artifactKey, artifactStatus) {
  return content.split(/\r?\n/).some((line) => {
    const trimmed = line.trim();
    if (/\[required\]/i.test(trimmed) || /^-[^:]+:\s*required\s*$/i.test(trimmed)) return true;
    if (!/^Required(?:[\s.,;:]|$)/.test(trimmed)) return false;
    return !(artifactKey === "decision_required" && artifactStatus === "open" && /^Required after resolution/.test(trimmed));
  });
}

function artifactPath(root, filename) {
  return join(root, ".ai", "workflow", filename);
}

function replaceMarkdownSection(content, heading, body) {
  const marker = `${heading}\n`;
  const headingStart = content.indexOf(marker);
  if (headingStart < 0) throw new Error(`Missing Markdown section: ${heading}`);
  const bodyStart = headingStart + marker.length;
  const nextHeading = content.indexOf("\n## ", bodyStart);
  const bodyEnd = nextHeading < 0 ? content.length : nextHeading;
  return `${content.slice(0, bodyStart)}\n${body}\n${content.slice(bodyEnd).replace(/^\n+/, "\n")}`;
}

function validateEventChain(events, contract, errors) {
  let previousHash = null;
  let previousSequence = 0;
  const eventIds = new Set();
  const idempotencyKeys = new Set();
  for (const event of events) {
    if (eventIds.has(event.event_id)) errors.push(`duplicate event_id: ${event.event_id}`);
    if (!event.idempotency_key || idempotencyKeys.has(event.idempotency_key)) errors.push(`event ${event.event_id ?? "unknown"}: missing or duplicate idempotency_key`);
    eventIds.add(event.event_id);
    idempotencyKeys.add(event.idempotency_key);
    if (event.schema_version !== contract.event_schema_version) errors.push(`event ${event.event_id ?? "unknown"}: unsupported schema_version`);
    if (!contract.roles.includes(event.role)) errors.push(`event ${event.event_id ?? "unknown"}: unknown role`);
    if (event.sequence !== previousSequence + 1) errors.push(`event sequence must be contiguous at ${event.event_id ?? "unknown"}`);
    if (event.previous_event_hash !== previousHash) errors.push(`event ${event.event_id ?? "unknown"}: previous_event_hash mismatch`);
    const { event_hash: recordedHash, ...hashInput } = event;
    const expectedHash = sha256(stable(hashInput));
    if (recordedHash !== expectedHash) errors.push(`event ${event.event_id ?? "unknown"}: event_hash mismatch`);
    if (!contract.statuses.includes(event.from_status) || !contract.statuses.includes(event.to_status)) errors.push(`event ${event.event_id ?? "unknown"}: unknown status`);
    if (event.event_type === "decision_resolved") {
      if (event.role !== "orchestrator" || event.from_status !== "human_decision_required" || event.to_status !== "human_decision_required" || !event.decision_reference) errors.push(`event ${event.event_id ?? "unknown"}: invalid decision resolution event`);
    } else if (event.event_type === "artifact_recorded") {
      if (event.from_status !== event.to_status || event.role !== contract.artifact_roles[event.artifact_key] || !contract.artifact_files[event.artifact_key] || !contract.artifact_statuses[event.artifact_key]?.includes(event.artifact_status)) errors.push(`event ${event.event_id ?? "unknown"}: invalid artifact record event`);
    } else if (event.event_type === "verification_recorded") {
      if (event.role !== "orchestrator" || event.from_status !== event.to_status || !["passed", "failed", "skipped"].includes(event.verification_status)) errors.push(`event ${event.event_id ?? "unknown"}: invalid verification record event`);
    } else if (event.event_type !== "transition") {
      errors.push(`event ${event.event_id ?? "unknown"}: unknown event_type ${event.event_type}`);
    } else {
      if (event.role !== contract.phase_roles[event.to_status]) errors.push(`event ${event.event_id ?? "unknown"}: transition role mismatch`);
      if (!(contract.transitions[event.from_status] ?? []).includes(event.to_status)) errors.push(`event ${event.event_id ?? "unknown"}: illegal transition ${event.from_status} -> ${event.to_status}`);
    }
    previousHash = recordedHash;
    previousSequence = event.sequence;
  }
}

export function validateWorkflow(root = defaultRoot, options = {}) {
  const paths = workflowPaths(root);
  const errors = [];
  let contract;
  let state;
  let events;
  try { contract = readJson(paths.contract); } catch (error) { return [`WORKFLOW_CONTRACT.json: ${error.message}`]; }
  try { state = readJson(paths.state); } catch (error) { return [`WORKFLOW_STATE.json: ${error.message}`]; }
  try { events = readEvents(root); } catch (error) { return [error.message]; }

  if (state.schema_version !== contract.state_schema_version) errors.push(`state schema_version must be ${contract.state_schema_version}`);
  if (!contract.statuses.includes(state.status)) errors.push(`unknown workflow status: ${state.status}`);
  if (!contract.roles.includes(state.current_role)) errors.push(`unknown current_role: ${state.current_role}`);
  if (state.current_role !== contract.phase_roles[state.status]) errors.push(`role ${state.current_role} does not own status ${state.status}`);
  if (!Number.isInteger(state.state_revision) || state.state_revision < 0) errors.push("state_revision must be a non-negative integer");
  if (!Number.isInteger(state.review_cycle) || state.review_cycle < 0 || state.review_cycle > contract.max_review_cycles) errors.push(`review_cycle must be 0..${contract.max_review_cycles}`);
  if (state.max_review_cycles !== contract.max_review_cycles) errors.push("max_review_cycles must match workflow contract");
  if (state.state_revision !== events.length) errors.push(`state_revision ${state.state_revision} does not match event count ${events.length}`);

  validateEventChain(events, contract, errors);
  const last = events.at(-1);
  if (last) {
    if (state.last_event_id !== last.event_id || state.last_event_hash !== last.event_hash) errors.push("state last event does not match EVENTS.jsonl");
    if (state.status !== last.to_status || state.sprint_id !== last.sprint_id) errors.push("state status/sprint does not match last event");
  } else if (state.last_event_id !== null || state.last_event_hash !== null) {
    errors.push("idle event log must have null last event fields");
  }

  const sprintPattern = new RegExp(contract.sprint_id_pattern);
  if (state.status === "idle") {
    if (state.sprint_id !== null) errors.push("idle state must not have a sprint_id");
    if (state.state_revision !== 0) errors.push("idle state must have state_revision 0");
  } else if (typeof state.sprint_id !== "string" || !sprintPattern.test(state.sprint_id)) {
    errors.push("active sprint_id does not match contract pattern");
  }

  const decisionOpen = state.status === "human_decision_required";
  if (decisionOpen) {
    if (!state.human_approval_required) errors.push("human_decision_required must set human_approval_required");
    if (!state.blocked_phase || !state.resume_phase) errors.push("human decision state requires blocked_phase and resume_phase");
    if (!Array.isArray(state.active_decision_ids) || state.active_decision_ids.length === 0) errors.push("human decision state requires active_decision_ids");
  } else if (state.human_approval_required && state.decision_resolution?.status !== "resolved") {
    errors.push("human_approval_required is true outside a resolvable human decision state");
  }

  for (const [key, filename] of Object.entries(contract.artifact_files)) {
    const path = artifactPath(root, filename);
    if (!existsSync(path)) { errors.push(`missing workflow artifact: ${filename}`); continue; }
    const content = readFileSync(path, "utf8");
    const status = markdownStatus(content);
    const allowed = contract.artifact_statuses[key] ?? [];
    if (!status || !allowed.includes(status)) errors.push(`${filename}: invalid or missing Status`);
    if (state.artifacts?.[key] !== status) errors.push(`${filename}: Status ${status} disagrees with state artifact ${state.artifacts?.[key]}`);
    if (!["pending", "idle", "draft", "not_required"].includes(status)) {
      if (hasRequiredPlaceholders(content, key, status)) errors.push(`${filename}: required placeholders remain`);
      for (const heading of contract.required_headings[filename] ?? []) {
        if (!content.includes(heading)) errors.push(`${filename}: missing heading ${heading}`);
      }
      if (state.sprint_id && !content.includes(`Sprint ID: ${state.sprint_id}`) && !content.includes(`## Sprint ID\n\n${state.sprint_id}`)) {
        errors.push(`${filename}: sprint ID does not match active state`);
      }
    }
  }

  if (!["idle", "intake"].includes(state.status) && state.artifacts.current_mission !== "ready") errors.push(`${state.status} requires a ready Current Mission`);
  if (["engineering_planning", "implementation", "validation", "theory_alignment_review", "completed"].includes(state.status) && !["approved", "approved_with_conditions"].includes(state.artifacts.product_review)) errors.push(`${state.status} requires approved Product Review`);
  if (["implementation", "validation", "theory_alignment_review", "completed"].includes(state.status) && state.artifacts.engineering_plan !== "approved") errors.push(`${state.status} requires approved Engineering Plan`);
  if (["validation", "theory_alignment_review", "completed"].includes(state.status) && !["completed", "completed_with_follow_up"].includes(state.artifacts.engineering_report)) errors.push(`${state.status} requires completed Engineering Report`);
  if (state.status === "completed" && !["approved", "approved_with_follow_up"].includes(state.artifacts.theory_alignment_review)) errors.push("completed requires approved Theory Alignment Review");
  if (state.status === "completed" && !["completed", "completed_with_follow_up"].includes(state.artifacts.sprint_report)) errors.push("completed requires a completed Sprint Report");
  if (state.status === "failed" && state.artifacts.sprint_report !== "failed") errors.push("failed requires a failed Sprint Report");
  if (state.status === "cancelled" && state.artifacts.sprint_report !== "cancelled") errors.push("cancelled requires a cancelled Sprint Report");
  if (decisionOpen && !["open", "resolved"].includes(state.artifacts.decision_required)) errors.push("human decision state requires open or resolved DECISION_REQUIRED.md");

  if (options.checkRepositoryFreshness && state.status !== "idle") {
    const snapshot = repositorySnapshot(root);
    const verification = state.verification?.repository_verify;
    if (["theory_alignment_review", "completed"].includes(state.status)) {
      if (verification?.status !== "passed" || verification.repository_head !== snapshot.head || verification.working_tree_digest !== snapshot.working_tree_digest) errors.push("repository verification evidence is missing or stale");
    }
  }

  return errors;
}

export function createTransition(state, toStatus, contract, options = {}) {
  if (!(contract.transitions[state.status] ?? []).includes(toStatus)) throw new Error(`Illegal transition ${state.status} -> ${toStatus}`);
  if (options.expectedSequence !== undefined && options.expectedSequence !== state.state_revision) throw new Error(`Expected sequence ${options.expectedSequence}, found ${state.state_revision}`);
  const next = structuredClone(state);
  if (toStatus === "revision_required") {
    if (!options.reason?.trim()) throw new Error("revision_required requires a concrete reason");
    if (next.review_cycle >= contract.max_review_cycles) throw new Error("Bounded revision limit reached");
    next.review_cycle += 1;
  }
  if (toStatus === "human_decision_required") {
    if (!options.resumePhase || !Array.isArray(options.decisionIds) || options.decisionIds.length === 0) throw new Error("Human decision transition requires resumePhase and decisionIds");
    next.human_approval_required = true;
    next.blocked_phase = state.status;
    next.resume_phase = options.resumePhase;
    next.active_decision_ids = options.decisionIds;
    next.decision_resolution = null;
  } else if (state.status === "human_decision_required") {
    if (toStatus !== state.resume_phase) throw new Error(`Resolved decision must resume at ${state.resume_phase}`);
    if (state.decision_resolution?.status !== "resolved" || !options.decisionReference) throw new Error("Human decision resume requires a recorded resolution and decisionReference");
    next.human_approval_required = false;
    next.blocked_phase = null;
    next.resume_phase = null;
    next.active_decision_ids = [];
  }
  next.status = toStatus;
  next.current_phase = toStatus;
  next.current_role = contract.phase_roles[toStatus];
  next.last_completed_phase = state.status;
  next.next_action = options.nextAction ?? null;
  next.blocking_reason = toStatus === "human_decision_required" ? (options.reason ?? "Founder decision required") : null;
  next.updated_at = options.occurredAt ?? new Date().toISOString();
  return next;
}

function buildEvent(previousState, nextState, snapshot, contract, options) {
  const sequence = previousState.state_revision + 1;
  const event = {
    schema_version: contract.event_schema_version,
    event_id: `${nextState.sprint_id}:${String(sequence).padStart(4, "0")}`,
    idempotency_key: options.idempotencyKey ?? randomUUID(),
    sprint_id: nextState.sprint_id,
    sequence,
    previous_event_hash: previousState.last_event_hash,
    event_type: options.eventType ?? "transition",
    from_status: previousState.status,
    to_status: nextState.status,
    role: nextState.current_role,
    occurred_at: nextState.updated_at,
    reason: options.reason ?? null,
    decision_ids: nextState.active_decision_ids,
    decision_reference: options.decisionReference ?? null,
    artifact_key: options.artifactKey ?? null,
    artifact_status: options.artifactStatus ?? null,
    verification_status: options.verificationStatus ?? null,
    repository_head: snapshot.head,
    working_branch: snapshot.branch,
    working_tree_digest: snapshot.working_tree_digest,
  };
  event.event_hash = sha256(stable(event));
  return event;
}

function appendEventAndState(root, previous, next, event) {
  const paths = workflowPaths(root);
  const existing = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  next.state_revision = event.sequence;
  next.last_event_id = event.event_id;
  next.last_event_hash = event.event_hash;
  next.repository_head = event.repository_head;
  next.working_branch = event.working_branch;
  next.working_tree_digest = event.working_tree_digest;
  writeAtomic(paths.events, `${existing}${existing && !existing.endsWith("\n") ? "\n" : ""}${JSON.stringify(event)}\n`);
  writeAtomic(paths.state, `${JSON.stringify(next, null, 2)}\n`);
}

function parseArgs(args) {
  const parsed = { _: [] };
  for (let i = 0; i < args.length; i += 1) {
    const value = args[i];
    if (!value.startsWith("--")) { parsed._.push(value); continue; }
    const key = value.slice(2).replaceAll("-", "_");
    const next = args[i + 1];
    if (next === undefined || next.startsWith("--")) parsed[key] = true;
    else { parsed[key] = next; i += 1; }
  }
  return parsed;
}

function commandValidate(root) {
  const errors = validateWorkflow(root, { checkRepositoryFreshness: true });
  if (errors.length) throw new Error(errors.join("\n"));
  process.stdout.write("AI workflow validation passed.\n");
}

function commandStatus(root) {
  const state = readJson(workflowPaths(root).state);
  process.stdout.write(`${JSON.stringify({ status: state.status, sprint_id: state.sprint_id, phase: state.current_phase, role: state.current_role, revision: state.state_revision, review_cycle: state.review_cycle, next_action: state.next_action }, null, 2)}\n`);
}

function commandStart(root, args) {
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  if (state.status !== "idle") throw new Error("Cannot start: workflow is not idle");
  if (!args.sprint_id || !new RegExp(contract.sprint_id_pattern).test(args.sprint_id)) throw new Error("--sprint-id is required and must match the contract");
  if (!args.mission_title) throw new Error("--mission-title is required");
  const now = new Date().toISOString();
  const snapshot = repositorySnapshot(root);
  const mission = `# Current Mission\n\nStatus: draft\n\n- Sprint ID: ${args.sprint_id}\n- Mission title: ${args.mission_title}\n- Origin: ${args.origin ?? "founder_request"}\n- Base branch: ${args.base_branch ?? "develop"}\n- Starting commit: ${snapshot.head}\n- Background: ${args.background ?? "[required]"}\n- Problem: ${args.problem ?? "[required]"}\n- Intended outcome: ${args.outcome ?? "[required]"}\n- Initial scope: ${args.scope ?? "[required]"}\n- Explicit non-scope: ${args.non_scope ?? "No implicit authority expansion."}\n- Relevant Book Zero definitions: [required]\n- Relevant ADRs: [required]\n- Relevant architecture documents: [required]\n- Relevant code areas: [required]\n- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.\n- Current owner: orchestrator\n- Current phase: intake\n- Created at: ${now}\n`;
  const missionPath = artifactPath(root, "CURRENT_MISSION.md");
  const previousMission = readFileSync(missionPath, "utf8");
  const previousState = readFileSync(paths.state, "utf8");
  const previousEvents = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  writeAtomic(missionPath, mission);
  state.sprint_id = args.sprint_id;
  state.mission_title = args.mission_title;
  state.artifacts.current_mission = "draft";
  state.created_at = now;
  const next = createTransition(state, "intake", contract, { occurredAt: now, nextAction: "Complete Product Review", reason: "Founder mission started" });
  const event = buildEvent(state, next, snapshot, contract, { reason: "Founder mission started", idempotencyKey: args.idempotency_key });
  try {
    appendEventAndState(root, state, next, event);
    const errors = validateWorkflow(root, { checkRepositoryFreshness: true });
    if (errors.length) throw new Error(errors.join("\n"));
  } catch (error) {
    writeAtomic(missionPath, previousMission);
    writeAtomic(paths.events, previousEvents);
    writeAtomic(paths.state, previousState);
    throw new Error(`Sprint start rejected and rolled back: ${error.message}`);
  }
}

function commandTransition(root, args) {
  if (!args.to) throw new Error("--to is required");
  if (args.expected_sequence === undefined) throw new Error("--expected-sequence is required");
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  const snapshot = repositorySnapshot(root);
  const next = createTransition(state, args.to, contract, {
    expectedSequence: args.expected_sequence === undefined ? undefined : Number(args.expected_sequence),
    resumePhase: args.resume_phase,
    decisionIds: args.decision_ids ? args.decision_ids.split(",").filter(Boolean) : [],
    decisionReference: args.decision_reference,
    nextAction: args.next_action,
    reason: args.reason,
  });
  const event = buildEvent(state, next, snapshot, contract, { reason: args.reason, decisionReference: args.decision_reference, idempotencyKey: args.idempotency_key });
  const previousState = readFileSync(paths.state, "utf8");
  const previousEvents = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  appendEventAndState(root, state, next, event);
  const errors = validateWorkflow(root, { checkRepositoryFreshness: true });
  if (errors.length) {
    // A transition is not durable until its destination artifact gate passes.
    // Restore both control-plane files so a failed command cannot strand the
    // repository in a partially advanced workflow state.
    writeAtomic(paths.events, previousEvents);
    writeAtomic(paths.state, previousState);
    throw new Error(`Transition rejected because destination artifacts are not reconciled:\n${errors.join("\n")}`);
  }
}

function commandRecordVerification(root, args) {
  if (!args.status || !["passed", "failed", "skipped"].includes(args.status)) throw new Error("--status must be passed, failed, or skipped");
  if (args.expected_sequence === undefined) throw new Error("--expected-sequence is required");
  const exitCode = args.exit_code === undefined ? null : Number(args.exit_code);
  if (args.status === "passed" && exitCode !== 0) throw new Error("passed verification requires --exit-code 0");
  if (args.status === "failed" && (!Number.isInteger(exitCode) || exitCode === 0)) throw new Error("failed verification requires a non-zero --exit-code");
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  if (state.status !== "validation") throw new Error("Repository verification can only be recorded during validation");
  if (Number(args.expected_sequence) !== state.state_revision) throw new Error(`Expected sequence ${args.expected_sequence}, found ${state.state_revision}`);
  const snapshot = repositorySnapshot(root);
  const next = structuredClone(state);
  next.verification.repository_verify = {
    status: args.status,
    command: args.command ?? "powershell -NoProfile -ExecutionPolicy Bypass -File .\\scripts\\verify.ps1",
    exit_code: exitCode,
    completed_at: new Date().toISOString(),
    repository_head: snapshot.head,
    working_tree_digest: snapshot.working_tree_digest,
  };
  next.updated_at = new Date().toISOString();
  const event = buildEvent(state, next, snapshot, contract, {
    eventType: "verification_recorded",
    reason: "Canonical verification result recorded",
    verificationStatus: args.status,
    idempotencyKey: args.idempotency_key,
  });
  event.role = "orchestrator";
  event.event_hash = sha256(stable(Object.fromEntries(Object.entries(event).filter(([key]) => key !== "event_hash"))));
  const previousState = readFileSync(paths.state, "utf8");
  const previousEvents = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  appendEventAndState(root, state, next, event);
  const errors = validateWorkflow(root, { checkRepositoryFreshness: false });
  if (errors.length) {
    writeAtomic(paths.events, previousEvents);
    writeAtomic(paths.state, previousState);
    throw new Error(`Verification record rejected:\n${errors.join("\n")}`);
  }
}

function commandRecordArtifact(root, args) {
  if (!args.artifact || !args.status) throw new Error("--artifact and --status are required");
  if (args.expected_sequence === undefined) throw new Error("--expected-sequence is required");
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  if (state.status === "idle") throw new Error("No active sprint");
  if (Number(args.expected_sequence) !== state.state_revision) throw new Error(`Expected sequence ${args.expected_sequence}, found ${state.state_revision}`);
  const filename = contract.artifact_files[args.artifact];
  if (!filename || !contract.artifact_statuses[args.artifact]?.includes(args.status)) throw new Error("Unknown artifact or status");
  const content = readFileSync(artifactPath(root, filename), "utf8");
  if (markdownStatus(content) !== args.status) throw new Error(`${filename} Markdown Status must already be ${args.status}`);
  if (!content.includes(`Sprint ID: ${state.sprint_id}`) && !content.includes(`## Sprint ID\n\n${state.sprint_id}`)) throw new Error(`${filename} must contain the active sprint ID`);
  if (hasRequiredPlaceholders(content, args.artifact, args.status)) throw new Error(`${filename} still contains required placeholders`);

  const previousState = readFileSync(paths.state, "utf8");
  const previousEvents = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  const next = structuredClone(state);
  next.artifacts[args.artifact] = args.status;
  next.updated_at = new Date().toISOString();
  const snapshot = repositorySnapshot(root);
  const event = buildEvent(state, next, snapshot, contract, {
    eventType: "artifact_recorded",
    reason: args.reason ?? `${args.artifact} recorded as ${args.status}`,
    artifactKey: args.artifact,
    artifactStatus: args.status,
    idempotencyKey: args.idempotency_key,
  });
  event.role = contract.artifact_roles[args.artifact];
  event.event_hash = sha256(stable(Object.fromEntries(Object.entries(event).filter(([key]) => key !== "event_hash"))));
  appendEventAndState(root, state, next, event);
  const errors = validateWorkflow(root, { checkRepositoryFreshness: false });
  if (errors.length) {
    writeAtomic(paths.events, previousEvents);
    writeAtomic(paths.state, previousState);
    throw new Error(`Artifact record rejected:\n${errors.join("\n")}`);
  }
}

function commandResolveDecision(root, args) {
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  if (state.status !== "human_decision_required") throw new Error("No active human decision to resolve");
  if (args.expected_sequence === undefined || Number(args.expected_sequence) !== state.state_revision) throw new Error(`--expected-sequence must equal ${state.state_revision}`);
  for (const required of ["decision_reference", "founder_response", "selected_option", "authorized_scope"]) {
    if (!args[required]) throw new Error(`--${required.replaceAll("_", "-")} is required`);
  }
  const now = new Date().toISOString();
  const next = structuredClone(state);
  next.decision_resolution = {
    status: "resolved",
    exact_founder_response: args.founder_response,
    selected_option: args.selected_option,
    authorized_scope: args.authorized_scope,
    decided_at: now,
    evidence_reference: args.decision_reference,
  };
  next.artifacts.decision_required = "resolved";
  next.updated_at = now;
  const decisionPath = artifactPath(root, "DECISION_REQUIRED.md");
  const previousDecision = readFileSync(decisionPath, "utf8");
  const previousState = readFileSync(paths.state, "utf8");
  const previousEvents = existsSync(paths.events) ? readFileSync(paths.events, "utf8") : "";
  let content = previousDecision.replace(/^Status:\s*open\s*$/m, "Status: resolved");
  content = replaceMarkdownSection(content, "## Resolution Status", "resolved");
  content = replaceMarkdownSection(content, "## Exact Founder Response", args.founder_response);
  content = replaceMarkdownSection(content, "## Selected Option And Authorized Scope", `- Selected option: ${args.selected_option}\n- Authorized scope: ${args.authorized_scope}`);
  content = replaceMarkdownSection(content, "## Decided At And Evidence Reference", `- Decided at: ${now}\n- Evidence reference: ${args.decision_reference}`);
  content = replaceMarkdownSection(content, "## Resume Phase", state.resume_phase);
  writeAtomic(decisionPath, content);
  const snapshot = repositorySnapshot(root);
  const event = buildEvent(state, next, snapshot, contract, {
    eventType: "decision_resolved",
    reason: "Founder decision recorded",
    decisionReference: args.decision_reference,
    idempotencyKey: args.idempotency_key,
  });
  try {
    appendEventAndState(root, state, next, event);
    const errors = validateWorkflow(root, { checkRepositoryFreshness: false });
    if (errors.length) throw new Error(errors.join("\n"));
  } catch (error) {
    writeAtomic(decisionPath, previousDecision);
    writeAtomic(paths.events, previousEvents);
    writeAtomic(paths.state, previousState);
    throw new Error(`Decision resolution rejected and rolled back: ${error.message}`);
  }
}

function commandArchive(root) {
  const paths = workflowPaths(root);
  const contract = loadContract(root);
  const state = readJson(paths.state);
  if (!contract.terminal_statuses.includes(state.status)) throw new Error("Only a terminal sprint can be archived");
  const errors = validateWorkflow(root, { checkRepositoryFreshness: true });
  if (errors.length) throw new Error(`Cannot archive an invalid workflow:\n${errors.join("\n")}`);

  const finalArchive = join(paths.history, state.sprint_id);
  if (existsSync(finalArchive)) throw new Error(`Archive already exists: ${state.sprint_id}`);
  const temporaryArchive = join(paths.history, `.tmp-${state.sprint_id}-${randomUUID()}`);
  mkdirSync(temporaryArchive, { recursive: true });
  let archivePromoted = false;
  const archivedFiles = [
    ...Object.values(contract.artifact_files),
    "WORKFLOW_STATE.json",
    "EVENTS.jsonl",
    "WORKFLOW_CONTRACT.json",
  ];
  try {
    for (const filename of archivedFiles) copyFileSync(join(paths.workflow, filename), join(temporaryArchive, filename));
    const manifest = {
      schema_version: "1.0",
      sprint_id: state.sprint_id,
      terminal_status: state.status,
      archived_at: new Date().toISOString(),
      repository_head: state.repository_head,
      working_branch: state.working_branch,
      working_tree_digest: state.working_tree_digest,
      last_event_id: state.last_event_id,
      last_event_hash: state.last_event_hash,
      files: archivedFiles,
    };
    writeFileSync(join(temporaryArchive, "ARCHIVE_MANIFEST.json"), `${JSON.stringify(manifest, null, 2)}\n`);
    renameSync(temporaryArchive, finalArchive);
    archivePromoted = true;

    const templateMap = {
      "current-mission.template.md": "CURRENT_MISSION.md",
      "product-review.template.md": "PRODUCT_REVIEW.md",
      "engineering-plan.template.md": "ENGINEERING_PLAN.md",
      "engineering-report.template.md": "ENGINEERING_REPORT.md",
      "theory-alignment-review.template.md": "THEORY_ALIGNMENT_REVIEW.md",
      "decision-required.template.md": "DECISION_REQUIRED.md",
      "sprint-report.template.md": "SPRINT_REPORT.md",
      "workflow-state.template.json": "WORKFLOW_STATE.json",
    };
    for (const [template, current] of Object.entries(templateMap)) {
      writeAtomic(join(paths.workflow, current), readFileSync(join(root, ".ai", "templates", template), "utf8"));
    }
    writeAtomic(paths.events, "");
    const resetErrors = validateWorkflow(root, { checkRepositoryFreshness: false });
    if (resetErrors.length) throw new Error(`Archive created but idle reset is invalid:\n${resetErrors.join("\n")}`);
  } catch (error) {
    if (existsSync(temporaryArchive)) rmSync(temporaryArchive, { recursive: true, force: true });
    if (archivePromoted && existsSync(finalArchive)) {
      // The archive is a complete recovery source. Restore the current control
      // plane before removing this command's just-created archive.
      for (const filename of archivedFiles) {
        if (filename !== "WORKFLOW_CONTRACT.json") copyFileSync(join(finalArchive, filename), join(paths.workflow, filename));
      }
      rmSync(finalArchive, { recursive: true, force: true });
    }
    throw error;
  }
}

function usage() {
  return `Usage:\n  node scripts/ai-workflow.mjs validate\n  node scripts/ai-workflow.mjs status\n  node scripts/ai-workflow.mjs start --sprint-id <id> --mission-title <title>\n  node scripts/ai-workflow.mjs record-artifact --artifact <key> --status <status> --expected-sequence <n>\n  node scripts/ai-workflow.mjs transition --to <status> --expected-sequence <n> [--reason <text>]\n  node scripts/ai-workflow.mjs resolve-decision --decision-reference <ref> --founder-response <text> --selected-option <id> --authorized-scope <text> --expected-sequence <n>\n  node scripts/ai-workflow.mjs record-verification --status <passed|failed|skipped> --exit-code <n> --expected-sequence <n>\n  node scripts/ai-workflow.mjs archive\n`;
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const command = args._[0];
  const root = args.root ? resolve(args.root) : defaultRoot;
  if (command === "validate") commandValidate(root);
  else if (command === "status") commandStatus(root);
  else if (command === "start") commandStart(root, args);
  else if (command === "transition") commandTransition(root, args);
  else if (command === "record-artifact") commandRecordArtifact(root, args);
  else if (command === "resolve-decision") commandResolveDecision(root, args);
  else if (command === "record-verification") commandRecordVerification(root, args);
  else if (command === "archive") commandArchive(root);
  else throw new Error(usage());
}

if (resolve(process.argv[1] ?? "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => { process.stderr.write(`${error.message}\n`); process.exitCode = 1; });
}
