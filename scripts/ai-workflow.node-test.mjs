import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { copyFileSync, cpSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import test from "node:test";

import { createTransition, loadContract, validateWorkflow } from "./ai-workflow.mjs";
import { validateImages } from "./ai-workflow-governance.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");

const digestBytes = (bytes) => createHash("sha256").update(bytes).digest("hex");

function prepareScopedManual(root) {
  const old = prepareResolvedTheoryDecision(root);
  const put = (path, value) => {
    mkdirSync(resolve(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), typeof value === "string" ? value : JSON.stringify(value));
    return { path, sha256: digestBytes(readFileSync(join(root, path))) };
  };
  const source = put("synthetic-source.txt", "unchanged synthetic product source");
  const sourceBinding = put(".artifacts/manual/source.json", {
    branch: readState(root).working_branch, baseGitSha: readState(root).repository_head,
    sourceSetDigest: "a".repeat(64), files: [source], protectedFiles: [],
  });
  const installer = put(".artifacts/manual/candidate.exe", "NOT AN EXECUTABLE: synthetic bytes only");
  const manifest = put(".artifacts/manual/manifest.json", {
    gitSha: readState(root).repository_head, sha256: installer.sha256,
    artifactFilename: "candidate.exe", artifactSize: readFileSync(join(root, installer.path)).length,
  });
  const target = { reviewSuffix: "synthetic-review-001", identity: "com.lifeos.synthetic", platform: "windows", dataClass: "synthetic", providerMode: "mock", installMode: "current-user" };
  const supplement = put(".artifacts/manual/supplement.json", {
    originalBindingSha256: sourceBinding.sha256, sourceSetDigest: "a".repeat(64),
    installerSha256: installer.sha256, manifestSha256: manifest.sha256,
    embeddedPayloadSha256: "b".repeat(64), installedHashFounderReported: "b".repeat(64),
    artifacts: [],
  });
  const steps = [{ step: 1, action: "synthetic observation", result: "PASS_UI_REPORTED", observationSource: "Founder reported, not agent direct" }];
  const ledger = put(".artifacts/manual/ledger.json", {
    environment: { identity: target.identity },
    candidate: { ...target, sourceBindingSha256: sourceBinding.sha256, sourceSetDigest: "a".repeat(64), installerSha256: installer.sha256, manifestSha256: manifest.sha256, supplementSha256: supplement.sha256, embeddedAndFounderInstalledPayloadSha256: "b".repeat(64), baseGitSha: readState(root).repository_head },
    steps, evidence: [], limits: ["No Android or zero-network proof"],
  });
  const receipt = put(".artifacts/manual/acceptance.json", {
    decision: "ACCEPTED", exactFounderResponse: "接受", evidenceSource: "Founder explicit chat reply after scoped acceptance clarification",
    precedingFounderScopeQuote: "specified synthetic Windows journey only", acceptedScope: "Only the synthetic candidate journey, not release",
    recordedAtUtc: "2026-10-01T00:00:00Z", ...target,
    sourceBindingSha256: sourceBinding.sha256, sourceSetDigest: "a".repeat(64), installerSha256: installer.sha256,
    manifestSha256: manifest.sha256, payloadSupplementSha256: supplement.sha256,
    embeddedAndFounderInstalledPayloadSha256: "b".repeat(64), baseGitSha: readState(root).repository_head,
    journeyLedger: ledger.path, journeyLedgerSha256: ledger.sha256,
    observedStepsAccepted: steps, limits: ["No Android or zero-network proof"],
  });
  const request = {
    format: "life-os/scoped-manual-verification-v1", sprintId: old.id,
    acceptanceSource: "direct-founder-chat", scope: "Only the synthetic candidate journey, not release",
    scopeQuote: "specified synthetic Windows journey only", target,
    evidence: { receipt, sourceBinding, installer, manifest, ledger, supplement },
  };
  const requestRef = put(".artifacts/manual/request.json", request);
  const args = () => ["record-scoped-manual-verification", "--sprint-id", old.id, "--expected-sequence", String(readState(root).state_revision), "--evidence", requestRef.path, "--evidence-sha256", digestBytes(readFileSync(join(root, requestRef.path))), "--acceptance-source", "direct-founder-chat", "--founder-response", "接受", "--scope", request.scope];
  return { ...old, request, args, put, requestRef };
}

function pairBytes(root) {
  return ["WORKFLOW_STATE.json", "EVENTS.jsonl"].map((name) => readFileSync(join(root, ".ai/workflow", name), "utf8"));
}

test("scoped manual registration preserves gates, old decision and observation planes; archive retains result", () => {
  withGitWorkflow((root) => {
    const f = prepareScopedManual(root);
    const before = readState(root);
    const events = pairBytes(root)[1];
    const result = runWorkflow(root, ...f.args());
    assert.equal(result.status, 0, result.stderr);
    const after = readState(root);
    assert.equal(after.status, before.status);
    assert.deepEqual(after.decision_resolution, before.decision_resolution);
    assert.deepEqual(after.verification.repository_verify, before.verification.repository_verify);
    assert.deepEqual(after.artifacts, before.artifacts);
    assert.deepEqual(after.verification.manual_ui.scoped_evidence.prior_verification, before.verification.manual_ui);
    assert.equal(after.verification.manual_ui.status, "passed");
    assert.equal(after.verification.manual_ui.scoped_evidence.acceptance.exactFounderResponse, "接受");
    assert.equal(after.verification.manual_ui.scoped_evidence.observations[0].observationSource, "Founder reported, not agent direct");
    assert.ok(pairBytes(root)[1].startsWith(events));
    assert.deepEqual(validateWorkflow(root), []);
    const once = pairBytes(root);
    assert.notEqual(runWorkflow(root, ...f.args()).status, 0, "duplicate must be refused");
    assert.deepEqual(pairBytes(root), once);
    f.request.scope = "different scope";
    f.put(f.requestRef.path, f.request);
    assert.notEqual(runWorkflow(root, ...f.args()).status, 0, "conflict must be refused");
    assert.deepEqual(pairBytes(root), once);

    assert.equal(runWorkflow(root, ...refreshVerificationArgs(after, f.decisionReference)).status, 0);
    assert.equal(runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", f.decisionReference, "--expected-sequence", String(readState(root).state_revision)).status, 0);
    for (const [key, file, status] of [["theory_alignment_review", "THEORY_ALIGNMENT_REVIEW.md", "approved"], ["sprint_report", "SPRINT_REPORT.md", "completed"]]) {
      completeArtifact(root, file, status, f.id);
      assert.equal(runWorkflow(root, "record-artifact", "--artifact", key, "--status", status, "--expected-sequence", String(readState(root).state_revision)).status, 0);
    }
    assert.equal(runWorkflow(root, "transition", "--to", "completed", "--expected-sequence", String(readState(root).state_revision)).status, 0);
    assert.equal(runWorkflow(root, "archive").status, 0);
    const archived = JSON.parse(readFileSync(join(root, ".ai/workflow/HISTORY", f.id, "WORKFLOW_STATE.json")));
    assert.deepEqual(archived.verification.manual_ui, after.verification.manual_ui);
    assert.deepEqual(archived.decision_resolution, before.decision_resolution);
    assert.equal(readState(root).verification.manual_ui.status, "not_run");
    assert.deepEqual(validateWorkflow(root), []);
    // Standalone archived validation has no ignored artifact files to reopen.
    const inspection = join(root, "archive-inspection");
    mkdirSync(join(inspection, ".ai"), { recursive: true });
    cpSync(join(root, ".ai/workflow/HISTORY", f.id), join(inspection, ".ai/workflow"), { recursive: true });
    assert.deepEqual(validateWorkflow(inspection), []);
  });
});

test("scoped manual refuses stale, malformed, missing, forged scope and cross-binding drift without writes", () => {
  withGitWorkflow((root) => {
    const f = prepareScopedManual(root);
    const original = JSON.stringify(f.request);
    const before = pairBytes(root);
    const reject = (args = f.args()) => {
      const r = runWorkflow(root, ...args);
      assert.notEqual(r.status, 0, r.stdout);
      assert.deepEqual(pairBytes(root), before);
    };
    reject([...f.args(), "--dry-run"]);
    for (const [flag, value] of [["--expected-sequence", "0"], ["--sprint-id", "other-sprint"], ["--scope", "accept everything"], ["--founder-response", "recommended"], ["--acceptance-source", "agent-report"], ["--evidence-sha256", "0".repeat(64)]]) {
      const a = f.args(); a[a.indexOf(flag) + 1] = value; reject(a);
    }
    for (const key of ["receipt", "ledger", "sourceBinding", "installer", "manifest", "supplement"]) {
      f.request.evidence[key].sha256 = "0".repeat(64); f.put(f.requestRef.path, f.request); reject();
      Object.assign(f.request, JSON.parse(original));
    }
    for (const mutate of [
      (r) => { r.scope = ""; }, (r) => { r.target.platform = "android"; },
      (r) => { r.evidence.receipt.path = "missing.json"; },
      (r) => { r.evidence.receipt.path = "../escape.json"; },
    ]) { Object.assign(f.request, JSON.parse(original)); mutate(f.request); f.put(f.requestRef.path, f.request); reject(); }
    Object.assign(f.request, JSON.parse(original)); f.put(f.requestRef.path, f.request);
    const receiptPath = f.request.evidence.receipt.path;
    const receipt = JSON.parse(readFileSync(join(root, receiptPath)));
    for (const [key, value] of [["decision", "PENDING"], ["acceptedScope", "different"], ["installerSha256", "0".repeat(64)], ["identity", "other.identity"], ["journeyLedgerSha256", "0".repeat(64)], ["evidenceSource", "agent recommendation"]]) {
      const changed = { ...receipt, [key]: value };
      f.request.evidence.receipt = f.put(receiptPath, changed); f.put(f.requestRef.path, f.request); reject();
    }
    f.request.evidence.receipt = f.put(receiptPath, receipt); f.put(f.requestRef.path, f.request);
    // Valid hashes are not enough: independently corrupt each bound object's content.
    for (const [key, mutate] of [
      ["sourceBinding", (v) => { v.baseGitSha = "0".repeat(40); }],
      ["ledger", (v) => { v.candidate.reviewSuffix = "other-candidate"; }],
      ["ledger", (v) => { v.steps[0].observationSource = "falsely relabeled direct"; }],
      ["supplement", (v) => { v.installedHashFounderReported = "0".repeat(64); }],
      ["manifest", (v) => { v.artifactSize += 1; }],
    ]) {
      const ref = { ...f.request.evidence[key] };
      const originalBytes = readFileSync(join(root, ref.path), "utf8");
      const v = JSON.parse(originalBytes); mutate(v);
      f.request.evidence[key] = f.put(ref.path, v);
      f.put(f.requestRef.path, f.request); reject();
      f.put(ref.path, originalBytes); f.request.evidence[key] = ref;
    }
    f.put(f.requestRef.path, f.request);
    // Malformed hashed receipt is not accepted as an explicit human decision.
    f.request.evidence.receipt = f.put(receiptPath, "{invalid");
    f.put(f.requestRef.path, f.request); reject();
    f.request.evidence.receipt = f.put(receiptPath, receipt); f.put(f.requestRef.path, f.request);
    f.put("synthetic-source.txt", "drift"); reject();
    f.put("synthetic-source.txt", "unchanged synthetic product source");
    f.put(f.requestRef.path, "{broken"); reject();
  });
});

test("scoped extension does not promote legacy state; wrong active state and tampered projection refuse", () => {
  withGitWorkflow((root) => {
    const f = prepareScopedManual(root);
    const before = readState(root);
    assert.equal(before.verification.manual_ui.status, "not_run");
    assert.equal(before.verification.manual_ui.scoped_evidence, undefined);
    assert.deepEqual(validateWorkflow(root), []);
    const statePath = join(root, ".ai/workflow/WORKFLOW_STATE.json");
    const stateBytes = readFileSync(statePath);
    writeFileSync(statePath, JSON.stringify({ ...before, status: "implementation" }));
    const wrongState = pairBytes(root);
    const rejected = runWorkflow(root, ...f.args());
    assert.notEqual(rejected.status, 0);
    assert.match(rejected.stderr, /active validation\/review\/decision state/);
    assert.deepEqual(pairBytes(root), wrongState);
    writeFileSync(statePath, stateBytes);
    assert.equal(runWorkflow(root, ...f.args()).status, 0);
    const tampered = readState(root);
    tampered.verification.manual_ui.scoped_evidence.scope = "all platforms";
    writeFileSync(statePath, JSON.stringify(tampered));
    assert.ok(validateWorkflow(root).some((e) => e.includes("does not match its registration event")));
  });
});

test("scoped optional fields leave an existing historical archive valid and unpromoted", () => {
  withWorkflow((root) => {
    const history = join(root, ".ai/workflow/HISTORY");
    const name = readdirSync(history, { withFileTypes: true }).find((e) => e.isDirectory() && !e.name.startsWith("."))?.name;
    assert.ok(name, "existing legacy archive fixture required");
    const inspection = join(root, "legacy-inspection");
    mkdirSync(join(inspection, ".ai"), { recursive: true });
    cpSync(join(history, name), join(inspection, ".ai/workflow"), { recursive: true });
    const before = pairBytes(inspection);
    assert.deepEqual(validateWorkflow(inspection), []);
    assert.deepEqual(pairBytes(inspection), before);
    assert.equal(readState(inspection).verification.manual_ui.scoped_evidence, undefined);
  });
});

test("scoped manual pair write failures roll back with no acceptance event", () => {
  withGitWorkflow((root) => {
    const f = prepareScopedManual(root);
    const before = pairBytes(root);
    for (const failAt of [1, 2]) {
      const result = spawnSync(process.execPath, [join(root, "scripts/ai-workflow.mjs"), ...f.args()], {
        cwd: root, encoding: "utf8", env: { ...process.env, NODE_ENV: "test", LIFE_OS_AI_WORKFLOW_TEST_FAIL_ATOMIC_RENAME_AT: String(failAt) },
      });
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, /Injected atomic rename failure/);
      assert.deepEqual(pairBytes(root), before);
      assert.deepEqual(validateWorkflow(root), []);
    }
  });
});

const currentWorkflowTemplates = {
  "CURRENT_MISSION.md": "current-mission.template.md",
  "PRODUCT_REVIEW.md": "product-review.template.md",
  "ENGINEERING_PLAN.md": "engineering-plan.template.md",
  "ENGINEERING_REPORT.md": "engineering-report.template.md",
  "THEORY_ALIGNMENT_REVIEW.md": "theory-alignment-review.template.md",
  "DECISION_REQUIRED.md": "decision-required.template.md",
  "SPRINT_REPORT.md": "sprint-report.template.md",
  "WORKFLOW_STATE.json": "workflow-state.template.json",
};

function resetCopiedWorkflowToIdle(root) {
  for (const [target, template] of Object.entries(currentWorkflowTemplates)) {
    copyFileSync(
      join(root, ".ai", "templates", template),
      join(root, ".ai", "workflow", target),
    );
  }
  writeFileSync(join(root, ".ai", "workflow", "EVENTS.jsonl"), "");
}

function withWorkflow(callback) {
  const root = mkdtempSync(join(tmpdir(), "life-os-ai-workflow-"));
  cpSync(join(repositoryRoot, ".ai"), join(root, ".ai"), { recursive: true });
  resetCopiedWorkflowToIdle(root);
  try { callback(root); } finally { rmSync(root, { recursive: true, force: true }); }
}

function withGitWorkflow(callback) {
  const root = mkdtempSync(join(tmpdir(), "life-os-ai-workflow-git-"));
  cpSync(join(repositoryRoot, ".ai"), join(root, ".ai"), { recursive: true });
  resetCopiedWorkflowToIdle(root);
  mkdirSync(join(root, "scripts"), { recursive: true });
  cpSync(join(repositoryRoot, "scripts", "ai-workflow.mjs"), join(root, "scripts", "ai-workflow.mjs"));
  cpSync(join(repositoryRoot, "scripts", "ai-workflow-governance.mjs"), join(root, "scripts", "ai-workflow-governance.mjs"));
  const git = (...args) => execFileSync("git", args, { cwd: root, stdio: "ignore" });
  git("init");
  git("config", "user.email", "workflow-test@example.invalid");
  git("config", "user.name", "Workflow Test");
  git("add", ".");
  git("commit", "-m", "fixture");
  try { callback(root); } finally { rmSync(root, { recursive: true, force: true }); }
}

function idleState(root) {
  return JSON.parse(readFileSync(join(root, ".ai", "workflow", "WORKFLOW_STATE.json"), "utf8"));
}

function completeArtifact(root, filename, status, sprintId) {
  const path = join(root, ".ai", "workflow", filename);
  const templateNames = {
    "PRODUCT_REVIEW.md": "product-review.template.md",
    "ENGINEERING_PLAN.md": "engineering-plan.template.md",
    "ENGINEERING_REPORT.md": "engineering-report.template.md",
    "THEORY_ALIGNMENT_REVIEW.md": "theory-alignment-review.template.md",
    "SPRINT_REPORT.md": "sprint-report.template.md",
  };
  copyFileSync(join(root, ".ai", "templates", templateNames[filename]), path);
  const content = readFileSync(path, "utf8")
    .replace("Status: pending", `Status: ${status}`)
    .replace(/:\s*required\s*$/gim, ": recorded")
    .replace(/^Required.*$/gm, "Recorded.");
  writeFileSync(path, `${content}\nSprint ID: ${sprintId}\n`);
}

function completeCurrentMission(root) {
  const path = join(root, ".ai", "workflow", "CURRENT_MISSION.md");
  const content = readFileSync(path, "utf8")
    .replace("Status: draft", "Status: ready")
    .replaceAll("[required]", "recorded from repository evidence");
  writeFileSync(path, content);
}

function completeOpenDecision(root, sprintId) {
  const source = readFileSync(join(root, ".ai", "templates", "decision-required.template.md"), "utf8");
  const content = source
    .replace("Status: not_required", "Status: open")
    .replace("- Sprint ID: required", `- Sprint ID: ${sprintId}`)
    .replace(/:\s*required\s*$/gim, ": recorded")
    .split(/\r?\n/)
    .map((line) => /^Required after resolution/.test(line) ? line : line.replace(/^Required.*$/, "Recorded."))
    .join("\n");
  writeFileSync(join(root, ".ai", "workflow", "DECISION_REQUIRED.md"), `${content}\n`);
}

function runWorkflow(root, ...args) {
  return spawnSync(process.execPath, [join(root, "scripts", "ai-workflow.mjs"), ...args], {
    cwd: root,
    encoding: "utf8",
  });
}

function readState(root) {
  return JSON.parse(readFileSync(join(root, ".ai", "workflow", "WORKFLOW_STATE.json"), "utf8"));
}

function prepareResolvedTheoryDecision(root, options = {}) {
  const id = options.sprintId ?? "2026-09-14-resolved-theory-decision";
  const decisionReference = options.decisionReference ?? "founder-message-resolved-theory";
  const founderResponse = options.founderResponse ?? "Approve the exact bounded decision only";
  const resumePhase = options.resumePhase ?? "theory_alignment_review";
  const run = (...args) => {
    const result = runWorkflow(root, ...args);
    assert.equal(result.status, 0, result.stderr);
  };

  run("start", "--sprint-id", id, "--mission-title", "Resolved theory decision fixture");
  completeCurrentMission(root);
  run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
  run("transition", "--to", "product_review", "--expected-sequence", "2");
  completeArtifact(root, "PRODUCT_REVIEW.md", "approved", id);
  run("record-artifact", "--artifact", "product_review", "--status", "approved", "--expected-sequence", "3");
  run("transition", "--to", "engineering_planning", "--expected-sequence", "4");
  completeArtifact(root, "ENGINEERING_PLAN.md", "approved", id);
  run("record-artifact", "--artifact", "engineering_plan", "--status", "approved", "--expected-sequence", "5");
  run("transition", "--to", "implementation", "--expected-sequence", "6");
  completeArtifact(root, "ENGINEERING_REPORT.md", "completed", id);
  run("record-artifact", "--artifact", "engineering_report", "--status", "completed", "--expected-sequence", "7");
  run("transition", "--to", "validation", "--expected-sequence", "8");
  run("record-verification", "--status", "passed", "--exit-code", "0", "--expected-sequence", "9");
  run("transition", "--to", "theory_alignment_review", "--expected-sequence", "10");
  completeArtifact(root, "THEORY_ALIGNMENT_REVIEW.md", "human_decision_required", id);
  run("record-artifact", "--artifact", "theory_alignment_review", "--status", "human_decision_required", "--expected-sequence", "11");
  completeOpenDecision(root, id);
  run("record-artifact", "--artifact", "decision_required", "--status", "open", "--expected-sequence", "12");
  run("transition", "--to", "human_decision_required", "--resume-phase", resumePhase, "--decision-ids", "decision-old", "--reason", "Founder authority required", "--expected-sequence", "13");
  if (options.resolved !== false) {
    run("resolve-decision", "--decision-reference", decisionReference, "--founder-response", founderResponse, "--selected-option", "A", "--authorized-scope", "Exact bounded scope", "--expected-sequence", "14");
  }
  return { decisionReference, founderResponse, id };
}

function refreshVerificationArgs(state, decisionReference, overrides = {}) {
  return [
    "refresh-verification-for-resume",
    "--status", overrides.status ?? "passed",
    "--exit-code", String(overrides.exitCode ?? 0),
    "--decision-reference", overrides.decisionReference ?? decisionReference,
    "--expected-sequence", String(overrides.expectedSequence ?? state.state_revision),
  ];
}

function prepareCompletedWorkflowWithFounderDecision(root, options = {}) {
  const founderResponse = options.founderResponse ?? "Founder accepted the exact build-only candidate";
  const fixture = prepareResolvedTheoryDecision(root, {
    sprintId: options.sprintId ?? "2026-09-21-terminal-follow-up",
    decisionReference: options.decisionReference ?? "founder-build-only-acceptance",
    founderResponse,
  });
  let state = readState(root);
  assert.equal(runWorkflow(root, ...refreshVerificationArgs(state, fixture.decisionReference)).status, 0);
  state = readState(root);
  assert.equal(runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", fixture.decisionReference, "--expected-sequence", String(state.state_revision)).status, 0);
  completeArtifact(root, "THEORY_ALIGNMENT_REVIEW.md", "approved", fixture.id);
  state = readState(root);
  assert.equal(runWorkflow(root, "record-artifact", "--artifact", "theory_alignment_review", "--status", "approved", "--expected-sequence", String(state.state_revision)).status, 0);
  completeArtifact(root, "SPRINT_REPORT.md", "completed", fixture.id);
  state = readState(root);
  assert.equal(runWorkflow(root, "record-artifact", "--artifact", "sprint_report", "--status", "completed", "--expected-sequence", String(state.state_revision)).status, 0);
  state = readState(root);
  assert.equal(runWorkflow(root, "transition", "--to", "completed", "--expected-sequence", String(state.state_revision)).status, 0);

  const evidencePath = join(root, ".artifacts", "android-m0", "native-review", "accepted.md");
  mkdirSync(join(root, ".artifacts", "android-m0", "native-review"), { recursive: true });
  const authorizationReference = options.authorizationReference ?? "ANDROID-M0-NATIVE-ACCEPT-001 Option A";
  writeFileSync(evidencePath, `# Accepted native review\n\n${authorizationReference}\n`);
  return { ...fixture, founderResponse, authorizationReference, evidencePath: ".artifacts/android-m0/native-review/accepted.md" };
}

function terminalFollowUpArgs(root, fixture, overrides = {}) {
  const state = readState(root);
  const head = execFileSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).trim();
  const branch = execFileSync("git", ["branch", "--show-current"], { cwd: root, encoding: "utf8" }).trim();
  return [
    "start-terminal-follow-up",
    "--sprint-id", overrides.sprintId ?? state.sprint_id,
    "--expected-head", overrides.expectedHead ?? head,
    "--expected-branch", overrides.expectedBranch ?? branch,
    "--expected-sequence", String(overrides.expectedSequence ?? state.state_revision),
    "--authorization-reference", overrides.authorizationReference ?? fixture.authorizationReference,
    "--authorization-evidence", overrides.authorizationEvidence ?? fixture.evidencePath,
    "--reason", overrides.reason ?? "Reconcile accepted Android M0 native corrections before promotion review",
  ];
}

function capturePlanningTransitionSnapshots(root, sprintId) {
  const script = join(root, "scripts", "ai-workflow.mjs");
  const run = (...args) => {
    const result = spawnSync(process.execPath, [script, ...args], { cwd: root, encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
  };
  const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
  const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");

  run("start", "--sprint-id", sprintId, "--mission-title", "Torn projection test");
  completeCurrentMission(root);
  run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
  run("transition", "--to", "product_review", "--expected-sequence", "2");
  completeArtifact(root, "PRODUCT_REVIEW.md", "approved", sprintId);
  run("record-artifact", "--artifact", "product_review", "--status", "approved", "--expected-sequence", "3");

  const before = {
    state: readFileSync(statePath, "utf8"),
    events: readFileSync(eventsPath, "utf8"),
  };
  run("transition", "--to", "engineering_planning", "--expected-sequence", "4");
  const after = {
    state: readFileSync(statePath, "utf8"),
    events: readFileSync(eventsPath, "utf8"),
  };

  return { statePath, eventsPath, before, after };
}

function assertAtomicTransitionFailureRollsBack(root, sprintId, failAt) {
  const fixture = capturePlanningTransitionSnapshots(root, sprintId);
  writeFileSync(fixture.statePath, fixture.before.state);
  writeFileSync(fixture.eventsPath, fixture.before.events);
  assert.deepEqual(validateWorkflow(root), []);

  const script = join(root, "scripts", "ai-workflow.mjs");
  const result = spawnSync(
    process.execPath,
    [script, "transition", "--to", "engineering_planning", "--expected-sequence", "4"],
    {
      cwd: root,
      encoding: "utf8",
      env: {
        ...process.env,
        NODE_ENV: "test",
        LIFE_OS_AI_WORKFLOW_TEST_FAIL_ATOMIC_RENAME_AT: String(failAt),
      },
    },
  );

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, new RegExp(`Injected atomic rename failure at attempt ${failAt}`));
  assert.equal(readFileSync(fixture.eventsPath, "utf8"), fixture.before.events);
  assert.equal(readFileSync(fixture.statePath, "utf8"), fixture.before.state);
  assert.deepEqual(
    readdirSync(join(root, ".ai", "workflow")).filter((name) => name.includes(".tmp-")),
    [],
  );
  assert.deepEqual(validateWorkflow(root), []);
}

test("idle repository workflow validates", () => {
  withWorkflow((root) => assert.deepEqual(validateWorkflow(root), []));
});

test("copied test workflow resets an active repository state to idle", () => {
  withWorkflow((root) => {
    const state = idleState(root);
    assert.equal(state.status, "idle");
    assert.equal(readFileSync(join(root, ".ai", "workflow", "EVENTS.jsonl"), "utf8"), "");
  });
});

test("validator treats LF and CRLF artifact line endings equivalently", () => {
  withGitWorkflow((root) => {
    const id = "2026-09-17-crlf-artifact-validation";
    assert.equal(runWorkflow(root, "start", "--sprint-id", id, "--mission-title", "CRLF artifact validation").status, 0);
    completeCurrentMission(root);

    const missionPath = join(root, ".ai", "workflow", "CURRENT_MISSION.md");
    const headingOnlyMission = readFileSync(missionPath, "utf8")
      .replace(`- Sprint ID: ${id}`, `## Sprint ID\n\n${id}`);
    writeFileSync(missionPath, headingOnlyMission);
    assert.equal(runWorkflow(root, "record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1").status, 0);
    assert.deepEqual(validateWorkflow(root), []);

    const crlfMission = readFileSync(missionPath, "utf8").replace(/\r?\n/g, "\r\n");
    writeFileSync(missionPath, crlfMission);
    assert.match(crlfMission, new RegExp(`## Sprint ID\\r\\n\\r\\n${id}`));
    assert.deepEqual(validateWorkflow(root), []);
  });
});

test("contract rejects a direct intake to implementation transition", () => {
  const contract = loadContract(repositoryRoot);
  const state = idleState(repositoryRoot);
  state.status = "intake";
  assert.throws(() => createTransition(state, "implementation", contract), /Illegal transition/);
});

test("bounded revision loop refuses a fourth revision", () => {
  const contract = loadContract(repositoryRoot);
  const state = idleState(repositoryRoot);
  state.status = "validation";
  state.current_role = "orchestrator";
  state.review_cycle = 3;
  assert.throws(() => createTransition(state, "revision_required", contract, { reason: "Still fails the approved criterion" }), /limit reached/);
});

test("revision transition requires concrete evidence", () => {
  const contract = loadContract(repositoryRoot);
  const state = idleState(repositoryRoot);
  state.status = "validation";
  state.current_role = "orchestrator";
  assert.throws(() => createTransition(state, "revision_required", contract), /concrete reason/);
});

test("human decision transition requires structured resume data", () => {
  const contract = loadContract(repositoryRoot);
  const state = idleState(repositoryRoot);
  state.status = "product_review";
  state.current_role = "chief_product_theorist";
  assert.throws(() => createTransition(state, "human_decision_required", contract), /resumePhase and decisionIds/);
  const next = createTransition(state, "human_decision_required", contract, { resumePhase: "product_review", decisionIds: ["decision-1"] });
  assert.equal(next.blocked_phase, "product_review");
  assert.equal(next.resume_phase, "product_review");
  assert.deepEqual(next.active_decision_ids, ["decision-1"]);
});

test("human decision cannot resume without exact resolution evidence", () => {
  const contract = loadContract(repositoryRoot);
  const state = idleState(repositoryRoot);
  state.status = "human_decision_required";
  state.current_role = "orchestrator";
  state.resume_phase = "engineering_planning";
  assert.throws(() => createTransition(state, "engineering_planning", contract), /recorded resolution/);
  state.decision_resolution = { status: "resolved", evidence_reference: "DECISION_REQUIRED.md" };
  const next = createTransition(state, "engineering_planning", contract, { decisionReference: "DECISION_REQUIRED.md" });
  assert.equal(next.human_approval_required, false);
});

test("validator rejects state and Markdown artifact disagreement", () => {
  withWorkflow((root) => {
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const state = JSON.parse(readFileSync(statePath, "utf8"));
    state.artifacts.product_review = "approved";
    writeFileSync(statePath, `${JSON.stringify(state, null, 2)}\n`);
    assert.ok(validateWorkflow(root).some((error) => error.includes("PRODUCT_REVIEW.md")));
  });
});

test("validator rejects an over-limit review cycle", () => {
  withWorkflow((root) => {
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const state = JSON.parse(readFileSync(statePath, "utf8"));
    state.review_cycle = 4;
    writeFileSync(statePath, `${JSON.stringify(state, null, 2)}\n`);
    assert.ok(validateWorkflow(root).some((error) => error.includes("review_cycle")));
  });
});

test("validator fails closed when the event journal is ahead of workflow state", () => {
  withGitWorkflow((root) => {
    const fixture = capturePlanningTransitionSnapshots(root, "2026-07-18-event-ahead");
    writeFileSync(fixture.statePath, fixture.before.state);
    writeFileSync(fixture.eventsPath, fixture.after.events);

    const errors = validateWorkflow(root);
    assert.ok(errors.includes("state_revision 4 does not match event count 5"));
    assert.ok(errors.includes("state last event does not match EVENTS.jsonl"));
    assert.ok(errors.includes("state status/sprint does not match last event"));
  });
});

test("validator fails closed when workflow state is ahead of the event journal", () => {
  withGitWorkflow((root) => {
    const fixture = capturePlanningTransitionSnapshots(root, "2026-07-18-state-ahead");
    writeFileSync(fixture.statePath, fixture.after.state);
    writeFileSync(fixture.eventsPath, fixture.before.events);

    const errors = validateWorkflow(root);
    assert.ok(errors.includes("state_revision 5 does not match event count 4"));
    assert.ok(errors.includes("state last event does not match EVENTS.jsonl"));
    assert.ok(errors.includes("state status/sprint does not match last event"));
  });
});

test("first atomic rename failure removes its temp and preserves the workflow pair", () => {
  withGitWorkflow((root) => {
    assertAtomicTransitionFailureRollsBack(root, "2026-07-18-first-rename-failure", 1);
  });
});

test("second atomic rename failure restores events and state without orphan temps", () => {
  withGitWorkflow((root) => {
    assertAtomicTransitionFailureRollsBack(root, "2026-07-18-second-rename-failure", 2);
  });
});

test("CLI rejects an artifact-incomplete transition without advancing state or events", () => {
  withGitWorkflow((root) => {
    const script = join(root, "scripts", "ai-workflow.mjs");
    const start = spawnSync(process.execPath, [script, "start", "--sprint-id", "2026-07-17-rollback", "--mission-title", "Rollback test"], { cwd: root, encoding: "utf8" });
    assert.equal(start.status, 0, start.stderr);
    const beforeState = readFileSync(join(root, ".ai", "workflow", "WORKFLOW_STATE.json"), "utf8");
    const beforeEvents = readFileSync(join(root, ".ai", "workflow", "EVENTS.jsonl"), "utf8");
    const transition = spawnSync(process.execPath, [script, "transition", "--to", "product_review", "--expected-sequence", "1"], { cwd: root, encoding: "utf8" });
    assert.notEqual(transition.status, 0);
    assert.match(transition.stderr, /rejected because destination artifacts are not reconciled/);
    assert.equal(readFileSync(join(root, ".ai", "workflow", "WORKFLOW_STATE.json"), "utf8"), beforeState);
    assert.equal(readFileSync(join(root, ".ai", "workflow", "EVENTS.jsonl"), "utf8"), beforeEvents);
  });
});

test("CLI rejects an approved artifact that still contains required placeholders", () => {
  withGitWorkflow((root) => {
    const script = join(root, "scripts", "ai-workflow.mjs");
    const run = (...args) => spawnSync(process.execPath, [script, ...args], { cwd: root, encoding: "utf8" });
    const id = "2026-07-17-placeholders";
    assert.equal(run("start", "--sprint-id", id, "--mission-title", "Placeholder test").status, 0);
    completeCurrentMission(root);
    assert.equal(run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1").status, 0);
    assert.equal(run("transition", "--to", "product_review", "--expected-sequence", "2").status, 0);
    const productPath = join(root, ".ai", "workflow", "PRODUCT_REVIEW.md");
    const incomplete = readFileSync(join(root, ".ai", "templates", "product-review.template.md"), "utf8")
      .replace("Status: pending", "Status: approved")
      .replace(/:\s*required\s*$/gim, ": recorded");
    writeFileSync(productPath, `${incomplete}\nSprint ID: ${id}\n`);
    const result = run("record-artifact", "--artifact", "product_review", "--status", "approved", "--expected-sequence", "3");
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /required placeholders/);
  });
});

test("CLI records exact founder decision evidence before resuming", () => {
  withGitWorkflow((root) => {
    const script = join(root, "scripts", "ai-workflow.mjs");
    const run = (...args) => {
      const result = spawnSync(process.execPath, [script, ...args], { cwd: root, encoding: "utf8" });
      assert.equal(result.status, 0, result.stderr);
    };
    const id = "2026-07-17-decision";
    run("start", "--sprint-id", id, "--mission-title", "Decision test");
    completeCurrentMission(root);
    run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
    run("transition", "--to", "product_review", "--expected-sequence", "2");
    completeArtifact(root, "PRODUCT_REVIEW.md", "human_decision_required", id);
    run("record-artifact", "--artifact", "product_review", "--status", "human_decision_required", "--expected-sequence", "3");
    completeOpenDecision(root, id);
    run("record-artifact", "--artifact", "decision_required", "--status", "open", "--expected-sequence", "4");
    run("transition", "--to", "human_decision_required", "--resume-phase", "product_review", "--decision-ids", "decision-1", "--reason", "Founder authority required", "--expected-sequence", "5");
    run("resolve-decision", "--decision-reference", "founder-message-1", "--founder-response", "Approve option A only", "--selected-option", "A", "--authorized-scope", "Documentation only", "--expected-sequence", "6");
    run("transition", "--to", "product_review", "--decision-reference", "founder-message-1", "--expected-sequence", "7");

    const decision = readFileSync(join(root, ".ai", "workflow", "DECISION_REQUIRED.md"), "utf8");
    assert.match(decision, /Status: resolved/);
    assert.match(decision, /Approve option A only/);
    assert.doesNotMatch(decision, /Required after resolution/);
    assert.deepEqual(validateWorkflow(root), []);
  });
});

test("resolved theory gate reproduces the stale-verification deadlock through the old command", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    writeFileSync(join(root, "manual-evidence.md"), "new repository evidence\n");
    const state = readState(root);

    const staleResume = runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", decisionReference, "--expected-sequence", String(state.state_revision));
    assert.notEqual(staleResume.status, 0);
    assert.match(staleResume.stderr, /verification evidence is missing or stale/);

    const oldCommand = runWorkflow(root, "record-verification", "--status", "passed", "--exit-code", "0", "--expected-sequence", String(state.state_revision));
    assert.notEqual(oldCommand.status, 0);
    assert.match(oldCommand.stderr, /only be recorded during validation/);
  });
});

test("resolved theory gate records fresh canonical verification and resumes", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    writeFileSync(join(root, "manual-evidence.md"), "new repository evidence\n");
    const before = readState(root);
    const refresh = runWorkflow(root, ...refreshVerificationArgs(before, decisionReference));
    assert.equal(refresh.status, 0, refresh.stderr);

    const refreshed = readState(root);
    assert.equal(refreshed.status, "human_decision_required");
    assert.equal(refreshed.resume_phase, "theory_alignment_review");
    assert.equal(refreshed.verification.repository_verify.status, "passed");
    assert.equal(refreshed.verification.repository_verify.exit_code, 0);
    assert.equal(
      refreshed.verification.repository_verify.working_branch,
      execFileSync("git", ["branch", "--show-current"], { cwd: root, encoding: "utf8" }).trim(),
    );
    const event = readFileSync(join(root, ".ai", "workflow", "EVENTS.jsonl"), "utf8").trim().split(/\r?\n/).map(JSON.parse).at(-1);
    assert.equal(event.event_type, "resume_verification_refreshed");
    assert.equal(event.decision_reference, decisionReference);
    assert.deepEqual(validateWorkflow(root, { checkRepositoryFreshness: true }), []);

    const resume = runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", decisionReference, "--expected-sequence", String(refreshed.state_revision));
    assert.equal(resume.status, 0, resume.stderr);
    assert.equal(readState(root).status, "theory_alignment_review");
  });
});

test("verification refresh rejects an unresolved decision", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root, { resolved: false });
    const state = readState(root);
    const result = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference));
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /requires a resolved decision/);
  });
});

test("verification refresh rejects a resolved decision with another resume phase", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root, { resumePhase: "implementation" });
    const state = readState(root);
    const result = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference));
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /limited to a theory_alignment_review resume/);
  });
});

test("verification refresh accepts only passed status with exit code zero", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    const state = readState(root);
    for (const status of ["failed", "skipped"]) {
      const result = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference, { status }));
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, /requires --status passed/);
    }
    const nonzero = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference, { exitCode: 1 }));
    assert.notEqual(nonzero.status, 0);
    assert.match(nonzero.stderr, /requires --exit-code 0/);
  });
});

test("failed and skipped validation evidence cannot satisfy theory review freshness", () => {
  for (const [status, exitCode] of [["failed", "1"], ["skipped", "0"]]) {
    withGitWorkflow((root) => {
      const script = join(root, "scripts", "ai-workflow.mjs");
      const run = (...args) => {
        const result = spawnSync(process.execPath, [script, ...args], { cwd: root, encoding: "utf8" });
        assert.equal(result.status, 0, result.stderr);
      };
      const id = `2026-09-14-${status}-verification`;
      run("start", "--sprint-id", id, "--mission-title", "Non-passing verification fixture");
      completeCurrentMission(root);
      run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
      run("transition", "--to", "product_review", "--expected-sequence", "2");
      completeArtifact(root, "PRODUCT_REVIEW.md", "approved", id);
      run("record-artifact", "--artifact", "product_review", "--status", "approved", "--expected-sequence", "3");
      run("transition", "--to", "engineering_planning", "--expected-sequence", "4");
      completeArtifact(root, "ENGINEERING_PLAN.md", "approved", id);
      run("record-artifact", "--artifact", "engineering_plan", "--status", "approved", "--expected-sequence", "5");
      run("transition", "--to", "implementation", "--expected-sequence", "6");
      completeArtifact(root, "ENGINEERING_REPORT.md", "completed", id);
      run("record-artifact", "--artifact", "engineering_report", "--status", "completed", "--expected-sequence", "7");
      run("transition", "--to", "validation", "--expected-sequence", "8");
      run("record-verification", "--status", status, "--exit-code", exitCode, "--expected-sequence", "9");
      const transition = runWorkflow(root, "transition", "--to", "theory_alignment_review", "--expected-sequence", "10");
      assert.notEqual(transition.status, 0);
      assert.match(transition.stderr, /verification evidence is missing or stale/);
    });
  }
});

test("verification refresh rejects stale sequence and decision-reference mismatch", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    const state = readState(root);
    const stale = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference, { expectedSequence: state.state_revision - 1 }));
    assert.notEqual(stale.status, 0);
    assert.match(stale.stderr, /Expected sequence/);
    const mismatch = runWorkflow(root, ...refreshVerificationArgs(state, decisionReference, { decisionReference: "different-founder-evidence" }));
    assert.notEqual(mismatch.status, 0);
    assert.match(mismatch.stderr, /exactly match/);
  });
});

test("repository mutation after resolved-gate verification remains stale", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    let state = readState(root);
    assert.equal(runWorkflow(root, ...refreshVerificationArgs(state, decisionReference)).status, 0);
    writeFileSync(join(root, "post-verification-change.md"), "changed after verification\n");
    state = readState(root);
    const resume = runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", decisionReference, "--expected-sequence", String(state.state_revision));
    assert.notEqual(resume.status, 0);
    assert.match(resume.stderr, /verification evidence is missing or stale/);
  });
});

test("verification refresh preserves the resolved decision and exact Founder response", () => {
  withGitWorkflow((root) => {
    const { decisionReference } = prepareResolvedTheoryDecision(root);
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const decisionPath = join(root, ".ai", "workflow", "DECISION_REQUIRED.md");
    const before = readState(root);
    const decisionBefore = readFileSync(decisionPath, "utf8");
    assert.equal(runWorkflow(root, ...refreshVerificationArgs(before, decisionReference)).status, 0);
    const after = readState(root);
    assert.deepEqual(after.decision_resolution, before.decision_resolution);
    assert.deepEqual(after.active_decision_ids, before.active_decision_ids);
    assert.equal(after.resume_phase, before.resume_phase);
    assert.equal(readFileSync(decisionPath, "utf8"), decisionBefore);
    assert.notEqual(readFileSync(statePath, "utf8"), `${JSON.stringify(before, null, 2)}\n`);
  });
});

test("verification refresh rolls back the workflow pair on either atomic rename failure", () => {
  for (const failAt of [1, 2]) {
    withGitWorkflow((root) => {
      const { decisionReference } = prepareResolvedTheoryDecision(root, { sprintId: `2026-09-14-refresh-atomic-${failAt}` });
      const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
      const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");
      const beforeState = readFileSync(statePath, "utf8");
      const beforeEvents = readFileSync(eventsPath, "utf8");
      const state = readState(root);
      const result = spawnSync(process.execPath, [join(root, "scripts", "ai-workflow.mjs"), ...refreshVerificationArgs(state, decisionReference)], {
        cwd: root,
        encoding: "utf8",
        env: { ...process.env, NODE_ENV: "test", LIFE_OS_AI_WORKFLOW_TEST_FAIL_ATOMIC_RENAME_AT: String(failAt) },
      });
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, new RegExp(`Injected atomic rename failure at attempt ${failAt}`));
      assert.equal(readFileSync(statePath, "utf8"), beforeState);
      assert.equal(readFileSync(eventsPath, "utf8"), beforeEvents);
      assert.deepEqual(readdirSync(join(root, ".ai", "workflow")).filter((name) => name.includes(".tmp-")), []);
      assert.deepEqual(validateWorkflow(root), []);
    });
  }
});

test("resumed theory review can open a new decision that still requires a new Founder response", () => {
  withGitWorkflow((root) => {
    const { decisionReference, id } = prepareResolvedTheoryDecision(root);
    let state = readState(root);
    assert.equal(runWorkflow(root, ...refreshVerificationArgs(state, decisionReference)).status, 0);
    state = readState(root);
    assert.equal(runWorkflow(root, "transition", "--to", "theory_alignment_review", "--decision-reference", decisionReference, "--expected-sequence", String(state.state_revision)).status, 0);
    completeOpenDecision(root, id);
    state = readState(root);
    assert.equal(runWorkflow(root, "record-artifact", "--artifact", "decision_required", "--status", "open", "--expected-sequence", String(state.state_revision)).status, 0);
    state = readState(root);
    assert.equal(runWorkflow(root, "transition", "--to", "human_decision_required", "--resume-phase", "theory_alignment_review", "--decision-ids", "decision-new", "--reason", "New Founder authority required", "--expected-sequence", String(state.state_revision)).status, 0);
    state = readState(root);
    assert.deepEqual(state.active_decision_ids, ["decision-new"]);
    assert.equal(state.decision_resolution, null);

    const missingResponse = runWorkflow(root, "resolve-decision", "--decision-reference", "new-founder-message", "--selected-option", "A", "--authorized-scope", "New bounded scope", "--expected-sequence", String(state.state_revision));
    assert.notEqual(missingResponse.status, 0);
    assert.match(missingResponse.stderr, /--founder-response is required/);
    assert.equal(readState(root).decision_resolution, null);
  });
});

test("authorized completed follow-up preserves terminal evidence and requires fresh verification", () => {
  withGitWorkflow((root) => {
    const fixture = prepareCompletedWorkflowWithFounderDecision(root);
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");
    const before = readState(root);
    const eventsBefore = readFileSync(eventsPath, "utf8");
    assert.equal(before.status, "completed");
    assert.match(validateWorkflow(root, { checkRepositoryFreshness: true }).join("\n"), /verification evidence is missing or stale/);

    const result = runWorkflow(root, ...terminalFollowUpArgs(root, fixture));
    assert.equal(result.status, 0, result.stderr);
    const after = readState(root);
    assert.equal(after.status, "validation");
    assert.equal(after.current_phase, "validation");
    assert.equal(after.terminal_follow_up.status, "validation_pending");
    assert.equal(after.terminal_follow_up.authorization_reference, fixture.authorizationReference);
    assert.equal(after.terminal_follow_up.original_terminal_snapshot.status, "completed");
    assert.deepEqual(after.terminal_follow_up.original_terminal_snapshot.verification, before.verification);
    assert.deepEqual(after.terminal_follow_up.original_terminal_snapshot.decision_resolution, before.decision_resolution);
    assert.deepEqual(after.decision_resolution, before.decision_resolution);
    assert.equal(after.decision_resolution.exact_founder_response, fixture.founderResponse);
    assert.equal(after.verification.repository_verify.status, "pending");
    assert.equal(after.verification.manual_ui.status, before.verification.manual_ui.status);
    assert.equal(after.human_approval_required, false);
    assert.deepEqual(after.active_decision_ids, []);
    assert.equal(readFileSync(eventsPath, "utf8").startsWith(eventsBefore), true);
    const followUpEvent = readFileSync(eventsPath, "utf8").trim().split(/\r?\n/).map(JSON.parse).at(-1);
    assert.equal(followUpEvent.event_type, "terminal_follow_up_started");
    assert.equal(followUpEvent.terminal_snapshot.last_event_id, before.last_event_id);
    assert.deepEqual(validateWorkflow(root, { checkRepositoryFreshness: true }), []);

    const prematureTheory = runWorkflow(root, "transition", "--to", "theory_alignment_review", "--expected-sequence", String(after.state_revision));
    assert.notEqual(prematureTheory.status, 0);
    assert.match(prematureTheory.stderr, /verification evidence is missing or stale/);
    assert.equal(readState(root).status, "validation");

    assert.equal(runWorkflow(root, "record-verification", "--status", "passed", "--exit-code", "0", "--expected-sequence", String(after.state_revision)).status, 0);
    const verified = readState(root);
    assert.equal(verified.terminal_follow_up.status, "verified");
    assert.equal(runWorkflow(root, "transition", "--to", "theory_alignment_review", "--expected-sequence", String(verified.state_revision)).status, 0);
    assert.equal(readState(root).status, "theory_alignment_review");
    assert.equal(readFileSync(statePath, "utf8").includes("M1"), false);
  });
});

test("terminal follow-up rejects mismatched coordinates and authorization evidence", () => {
  withGitWorkflow((root) => {
    const fixture = prepareCompletedWorkflowWithFounderDecision(root, { sprintId: "2026-09-21-terminal-rejections" });
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");
    const beforeState = readFileSync(statePath, "utf8");
    const beforeEvents = readFileSync(eventsPath, "utf8");
    const state = readState(root);
    const cases = [
      [{ sprintId: "wrong-sprint" }, /--sprint-id must exactly match/],
      [{ expectedHead: "0".repeat(40) }, /--expected-head must exactly match/],
      [{ expectedBranch: "wrong-branch" }, /--expected-branch must exactly match/],
      [{ expectedSequence: state.state_revision - 1 }, /Expected sequence/],
      [{ authorizationReference: "ANDROID-M0-NATIVE-ACCEPT-001 Option B" }, /must appear exactly/],
    ];
    for (const [overrides, expected] of cases) {
      const result = runWorkflow(root, ...terminalFollowUpArgs(root, fixture, overrides));
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, expected);
      assert.equal(readFileSync(statePath, "utf8"), beforeState);
      assert.equal(readFileSync(eventsPath, "utf8"), beforeEvents);
    }
  });
});

test("terminal follow-up rejects a damaged event chain without writing", () => {
  withGitWorkflow((root) => {
    const fixture = prepareCompletedWorkflowWithFounderDecision(root, { sprintId: "2026-09-21-terminal-damaged-chain" });
    const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
    const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");
    const events = readFileSync(eventsPath, "utf8").trim().split(/\r?\n/).map(JSON.parse);
    events[0].reason = "tampered without rehashing";
    writeFileSync(eventsPath, `${events.map((event) => JSON.stringify(event)).join("\n")}\n`);
    const beforeState = readFileSync(statePath, "utf8");
    const beforeEvents = readFileSync(eventsPath, "utf8");

    const result = runWorkflow(root, ...terminalFollowUpArgs(root, fixture));
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /Terminal follow-up preflight failed/);
    assert.match(result.stderr, /event_hash mismatch/);
    assert.equal(readFileSync(statePath, "utf8"), beforeState);
    assert.equal(readFileSync(eventsPath, "utf8"), beforeEvents);
  });
});

test("terminal follow-up rolls back state and events on either atomic rename failure", () => {
  for (const failAt of [1, 2]) {
    withGitWorkflow((root) => {
      const fixture = prepareCompletedWorkflowWithFounderDecision(root, { sprintId: `2026-09-21-terminal-atomic-${failAt}` });
      const statePath = join(root, ".ai", "workflow", "WORKFLOW_STATE.json");
      const eventsPath = join(root, ".ai", "workflow", "EVENTS.jsonl");
      const beforeState = readFileSync(statePath, "utf8");
      const beforeEvents = readFileSync(eventsPath, "utf8");
      const result = spawnSync(process.execPath, [join(root, "scripts", "ai-workflow.mjs"), ...terminalFollowUpArgs(root, fixture)], {
        cwd: root,
        encoding: "utf8",
        env: { ...process.env, NODE_ENV: "test", LIFE_OS_AI_WORKFLOW_TEST_FAIL_ATOMIC_RENAME_AT: String(failAt) },
      });
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, new RegExp(`Injected atomic rename failure at attempt ${failAt}`));
      assert.equal(readFileSync(statePath, "utf8"), beforeState);
      assert.equal(readFileSync(eventsPath, "utf8"), beforeEvents);
      assert.deepEqual(readdirSync(join(root, ".ai", "workflow")).filter((name) => name.includes(".tmp-")), []);
      assert.deepEqual(validateWorkflow(root), []);
    });
  }
});

test("CLI records role artifacts, verification, completion, and terminal archive", () => {
  withGitWorkflow((root) => {
    const script = join(root, "scripts", "ai-workflow.mjs");
    const run = (...args) => {
      const result = spawnSync(process.execPath, [script, ...args], { cwd: root, encoding: "utf8" });
      assert.equal(result.status, 0, result.stderr);
    };
    const id = "2026-07-17-archive";
    run("start", "--sprint-id", id, "--mission-title", "Archive test");
    completeCurrentMission(root);
    run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
    run("transition", "--to", "product_review", "--expected-sequence", "2");
    completeArtifact(root, "PRODUCT_REVIEW.md", "approved", id);
    run("record-artifact", "--artifact", "product_review", "--status", "approved", "--expected-sequence", "3");
    run("transition", "--to", "engineering_planning", "--expected-sequence", "4");
    completeArtifact(root, "ENGINEERING_PLAN.md", "approved", id);
    run("record-artifact", "--artifact", "engineering_plan", "--status", "approved", "--expected-sequence", "5");
    run("transition", "--to", "implementation", "--expected-sequence", "6");
    completeArtifact(root, "ENGINEERING_REPORT.md", "completed", id);
    run("record-artifact", "--artifact", "engineering_report", "--status", "completed", "--expected-sequence", "7");
    run("transition", "--to", "validation", "--expected-sequence", "8");
    const prematureTheory = spawnSync(process.execPath, [script, "transition", "--to", "theory_alignment_review", "--expected-sequence", "9"], { cwd: root, encoding: "utf8" });
    assert.notEqual(prematureTheory.status, 0);
    assert.match(prematureTheory.stderr, /verification evidence is missing or stale/);
    run("record-verification", "--status", "passed", "--exit-code", "0", "--expected-sequence", "9");
    run("transition", "--to", "theory_alignment_review", "--expected-sequence", "10");
    completeArtifact(root, "THEORY_ALIGNMENT_REVIEW.md", "approved", id);
    run("record-artifact", "--artifact", "theory_alignment_review", "--status", "approved", "--expected-sequence", "11");
    completeArtifact(root, "SPRINT_REPORT.md", "completed", id);
    run("record-artifact", "--artifact", "sprint_report", "--status", "completed", "--expected-sequence", "12");
    run("transition", "--to", "completed", "--expected-sequence", "13");
    run("archive");

    const archived = join(root, ".ai", "workflow", "HISTORY", id);
    assert.equal(JSON.parse(readFileSync(join(archived, "WORKFLOW_STATE.json"), "utf8")).status, "completed");
    assert.equal(JSON.parse(readFileSync(join(archived, "ARCHIVE_MANIFEST.json"), "utf8")).last_event_id, `${id}:0014`);
    assert.equal(idleState(root).status, "idle");
    assert.deepEqual(validateWorkflow(root), []);
  });
});

// All governance fixtures below are freshly synthesized from templates and the
// vocabulary contract. No real R3 state/events/decisions/history is used.
function withGovernance(callback) {
  const root = mkdtempSync(join(tmpdir(), "life-os-governance-synthetic-"));
  mkdirSync(join(root, ".ai/workflow"), { recursive: true });
  cpSync(join(repositoryRoot, ".ai/templates"), join(root, ".ai/templates"), { recursive: true });
  copyFileSync(join(repositoryRoot, ".ai/workflow/WORKFLOW_CONTRACT.json"), join(root, ".ai/workflow/WORKFLOW_CONTRACT.json"));
  resetCopiedWorkflowToIdle(root);
  mkdirSync(join(root, "scripts"));
  for (const name of ["ai-workflow.mjs", "ai-workflow-governance.mjs"]) copyFileSync(join(repositoryRoot, "scripts", name), join(root, "scripts", name));
  const git = (...args) => execFileSync("git", args, { cwd: root, stdio: "ignore" });
  git("init"); git("config", "user.email", "synthetic@example.invalid"); git("config", "user.name", "Synthetic fixture"); git("add", "."); git("commit", "-m", "fresh synthetic fixture only");
  try {
    const id = "synthetic-r3-schema-gate";
    const run = (...args) => { const r = runWorkflow(root, ...args); assert.equal(r.status, 0, r.stderr); return r; };
    run("start", "--sprint-id", id, "--mission-title", "Synthetic unresolved schema");
    completeCurrentMission(root);
    run("record-artifact", "--artifact", "current_mission", "--status", "ready", "--expected-sequence", "1");
    run("transition", "--to", "product_review", "--expected-sequence", "2");
    completeOpenDecision(root, id);
    run("record-artifact", "--artifact", "decision_required", "--status", "open", "--expected-sequence", "3");
    run("transition", "--to", "human_decision_required", "--resume-phase", "engineering_planning", "--decision-ids", "SYNTHETIC-SCHEMA-UNRESOLVED", "--reason", "Synthetic schema unresolved; ADR remains Proposed", "--expected-sequence", "4");
    const request = {
      format: "life-os/sprint-disposition-v1", disposition: "cancelled", sprint_id: id,
      expected_sequence: 5, scope: "cancel-current-sprint-only", authorization_source: "direct-founder-chat",
      authorization_reference: "SYNTHETIC-FOUNDER-TEST-ONLY", exact_founder_response: "SYNTHETIC ONLY: cancel this sprint; schema unresolved, no ADR acceptance or product authority.",
      retained_constraints: { schema_decision: "unresolved", adr_status: "unchanged", product_authority: "none" },
    };
    const evidencePath = join(root, "synthetic-disposition.json");
    const put = () => writeFileSync(evidencePath, `${JSON.stringify(request, null, 2)}\n`);
    put();
    const args = () => ["cancel-sprint", "--sprint-id", id, "--expected-sequence", String(request.expected_sequence), "--evidence", "synthetic-disposition.json", "--evidence-sha256", digestBytes(readFileSync(evidencePath)), "--founder-response", request.exact_founder_response, "--scope", request.scope];
    callback({ root, run, id, request, put, args });
  } finally { rmSync(root, { recursive: true, force: true }); }
}
const currentBytes = (root) => Object.fromEntries(readdirSync(join(root, ".ai/workflow"), { withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => [entry.name, digestBytes(readFileSync(join(root, ".ai/workflow", entry.name)))]));
function pendingRecord(root) {
  const path = join(root, ".ai/workflow/.governance-operation/operation.json");
  return { path, bytes: readFileSync(path), hash: digestBytes(readFileSync(path)), record: JSON.parse(readFileSync(path, "utf8")) };
}
function injectGovernance(root, args, point, mode = "KILL") {
  const result = spawnSync(process.execPath, [join(root, "scripts/ai-workflow.mjs"), ...args], {
    cwd: root, encoding: "utf8", env: { ...process.env, NODE_ENV: "test", [`LIFE_OS_GOVERNANCE_${mode}_AT`]: point },
  });
  process.stdout.write(`${JSON.stringify({ governance_fault: point, mode, status: result.status, signal: result.signal, stderr: result.stderr.trim() })}\n`);
  if (mode === "KILL") { assert.equal(result.signal, "SIGKILL"); assert.equal(result.status, null); }
  else { assert.equal(result.status, 1); assert.match(result.stderr, /Injected governance failure/); }
  const output = process.env.LIFE_OS_GOVERNANCE_EVIDENCE_DIRECTORY;
  const kind = args[0] === "cancel-sprint" ? "cancel" : args[0];
  if (output && mode === "KILL" && ((kind === "cancel" && point === "write:EVENTS.jsonl") || (kind === "archive" && point === "write:WORKFLOW_STATE.json"))) {
    const destination = join(output, `${kind}-interrupted`);
    cpSync(join(root, ".ai/workflow"), destination, { recursive: true });
    writeFileSync(join(output, `${kind}-OBSERVATION.json`), JSON.stringify({ command: args, point, mode, status: result.status, signal: result.signal, operation_sha256: pendingRecord(root).hash }, null, 2));
  }
  return result;
}
function resumeGovernance(f, env) {
  const hash = pendingRecord(f.root).hash;
  const args = ["recover-governance", "--operation-sha256", hash];
  if (env) return injectGovernance(f.root, args, env.point, env.mode);
  f.run(...args);
  assert.deepEqual(validateWorkflow(f.root), []);
  assert.equal(readdirSync(join(f.root, ".ai/workflow")).includes(".governance-operation"), false);
}

test("disposition cancels unresolved schema without resolving it, granting PASS, or changing historical artifacts", () => {
  withGovernance((f) => {
    const before = readState(f.root), oldFiles = currentBytes(f.root), prefix = pairBytes(f.root)[1];
    const resume = runWorkflow(f.root, "transition", "--to", "engineering_planning", "--decision-reference", "pretend", "--expected-sequence", "5");
    assert.equal(resume.status, 1); assert.match(resume.stderr, /recorded resolution/);
    for (const to of ["failed", "cancelled"]) {
      assert.equal(runWorkflow(f.root, "transition", "--to", to, "--expected-sequence", "5").status, 1);
      assert.deepEqual(currentBytes(f.root), oldFiles);
    }
    f.run(...f.args());
    const after = readState(f.root);
    assert.equal(after.status, "cancelled"); assert.equal(after.decision_resolution, null);
    for (const key of ["verification", "active_decision_ids", "blocked_phase", "resume_phase", "blocking_reason", "last_completed_phase", "artifacts"]) assert.deepEqual(after[key], before[key]);
    assert.deepEqual(after.sprint_disposition.checkpoint, before);
    assert.ok(pairBytes(f.root)[1].startsWith(prefix));
    for (const name of Object.keys(oldFiles).filter((name) => !["WORKFLOW_STATE.json", "EVENTS.jsonl"].includes(name))) assert.equal(currentBytes(f.root)[name], oldFiles[name]);
    assert.deepEqual(validateWorkflow(f.root), []);
    assert.equal(runWorkflow(f.root, ...f.args()).status, 1);
    assert.equal(runWorkflow(f.root, "transition", "--to", "engineering_planning", "--expected-sequence", "6").status, 1);
    const statePath = join(f.root, ".ai/workflow/WORKFLOW_STATE.json");
    after.decision_resolution = { status: "resolved" };
    writeFileSync(statePath, JSON.stringify(after));
    assert.match(validateWorkflow(f.root).join("\n"), /preserve decisions/);
  });
});

test("disposition refuses missing authority, stale coordinates, changed hash and scope without any state/artifact writes", () => {
  withGovernance((f) => {
    const before = currentBytes(f.root);
    assert.equal(runWorkflow(f.root, "cancel-sprint").status, 1); assert.deepEqual(currentBytes(f.root), before);
    const replace = (args, key, value) => { const copy = [...args]; copy[copy.indexOf(key) + 1] = value; return copy; };
    for (const [key, value] of [["--evidence", "missing.json"], ["--sprint-id", "other-sprint"], ["--expected-sequence", "4"], ["--evidence-sha256", "0".repeat(64)], ["--scope", "cancel-and-accept-schema"], ["--founder-response", "different"], ["--evidence", ".ai/workflow/DECISION_REQUIRED.md"]]) {
      const result = runWorkflow(f.root, ...replace(f.args(), key, value));
      assert.equal(result.status, 1, `${key}: ${result.stderr}`); assert.deepEqual(currentBytes(f.root), before);
    }
    const valid = structuredClone(f.request);
    for (const [key, value] of [["authorization_source", "agent-recommendation"], ["authorization_reference", ""], ["exact_founder_response", ""], ["sprint_id", "wrong-sprint"], ["expected_sequence", 4], ["scope", "complete-product"], ["retained_constraints", {}]]) {
      Object.assign(f.request, valid, { [key]: value }); f.put();
      assert.equal(runWorkflow(f.root, ...f.args()).status, 1, key); assert.deepEqual(currentBytes(f.root), before);
    }
    Object.assign(f.request, valid, { extra_authority: "accept ADR" }); f.put();
    assert.equal(runWorkflow(f.root, ...f.args()).status, 1); assert.deepEqual(currentBytes(f.root), before);
    delete f.request.extra_authority;
    Object.assign(f.request, valid); f.put(); const oldArgs = f.args();
    writeFileSync(join(f.root, "synthetic-disposition.json"), "changed after hash");
    assert.equal(runWorkflow(f.root, ...oldArgs).status, 1); assert.deepEqual(currentBytes(f.root), before);
  });
});

for (const point of ["prepared", "before-write:EVENTS.jsonl", "before-rename:EVENTS.jsonl", "write:EVENTS.jsonl", "before-write:WORKFLOW_STATE.json", "before-rename:WORKFLOW_STATE.json", "write:WORKFLOW_STATE.json", "verified"]) {
  test(`SIGKILL cancellation at ${point} preserves recovery and blocks all mutations`, () => withGovernance((f) => {
    const before = readState(f.root);
    injectGovernance(f.root, f.args(), point);
    const hash = pendingRecord(f.root).hash, bytes = currentBytes(f.root);
    for (const command of ["start", "transition", "resolve-decision", "archive", "cancel-sprint", "record-artifact", "record-verification", "refresh-verification-for-resume", "start-terminal-follow-up", "record-scoped-manual-verification"]) {
      const result = runWorkflow(f.root, command); assert.equal(result.status, 1); assert.match(result.stderr, /Governance recovery required/); assert.deepEqual(currentBytes(f.root), bytes);
    }
    assert.equal(runWorkflow(f.root, "validate").status, 1);
    assert.equal(runWorkflow(f.root, "recover-governance", "--operation-sha256", "0".repeat(64)).status, 1);
    assert.equal(pendingRecord(f.root).hash, hash);
    resumeGovernance(f);
    assert.deepEqual(readState(f.root).sprint_disposition.checkpoint, before);
    assert.equal(readState(f.root).status, "cancelled");
  }));
}

test("recovery failure and a second SIGKILL preserve the sole record and permit exact reentry", () => withGovernance((f) => {
  injectGovernance(f.root, f.args(), "write:EVENTS.jsonl");
  const hash = pendingRecord(f.root).hash;
  resumeGovernance(f, { point: "before-write:WORKFLOW_STATE.json", mode: "FAIL" });
  assert.equal(pendingRecord(f.root).hash, hash);
  resumeGovernance(f, { point: "write:WORKFLOW_STATE.json", mode: "KILL" });
  assert.equal(pendingRecord(f.root).hash, hash);
  resumeGovernance(f);
  assert.equal(runWorkflow(f.root, "recover-governance", "--operation-sha256", hash).status, 1);
}));

const archiveFileNames = ["CURRENT_MISSION.md", "PRODUCT_REVIEW.md", "ENGINEERING_PLAN.md", "ENGINEERING_REPORT.md", "THEORY_ALIGNMENT_REVIEW.md", "DECISION_REQUIRED.md", "SPRINT_REPORT.md", "WORKFLOW_STATE.json", "EVENTS.jsonl", "WORKFLOW_CONTRACT.json", "SPRINT_DISPOSITION.json", "CANCELLATION_CHECKPOINT.json", "SPRINT_DISPOSITION_EVIDENCE.json", "ARCHIVE_MANIFEST.json"];
const resetFileNames = archiveFileNames.slice(0, 9);
for (const point of ["prepared", ...archiveFileNames.map((name) => `copy:${name}`), "before-publish", "published", ...resetFileNames.flatMap((name) => [`before-write:${name}`, `write:${name}`]), "verified"]) {
  test(`SIGKILL archive at ${point} retains verified checkpoint and resumes only this reset`, () => withGovernance((f) => {
    const gate = readState(f.root), originalDecision = readFileSync(join(f.root, ".ai/workflow/DECISION_REQUIRED.md")), prefix = pairBytes(f.root)[1];
    f.run(...f.args());
    const cancelled = currentBytes(f.root);
    injectGovernance(f.root, ["archive"], point);
    const attempt = runWorkflow(f.root, "start", "--sprint-id", "new-synthetic-mission", "--mission-title", "Must refuse partial reset");
    assert.equal(attempt.status, 1); assert.match(attempt.stderr, /Governance recovery required/);
    resumeGovernance(f);
    const archived = join(f.root, ".ai/workflow/HISTORY", f.id);
    const manifest = JSON.parse(readFileSync(join(archived, "ARCHIVE_MANIFEST.json")));
    for (const name of manifest.files) assert.equal(digestBytes(readFileSync(join(archived, name))), manifest.hashes[name]);
    for (const [name, hash] of Object.entries(cancelled)) assert.equal(digestBytes(readFileSync(join(archived, name))), hash);
    assert.deepEqual(JSON.parse(readFileSync(join(archived, "CANCELLATION_CHECKPOINT.json"))), gate);
    assert.deepEqual(readFileSync(join(archived, "DECISION_REQUIRED.md")), originalDecision);
    assert.ok(readFileSync(join(archived, "EVENTS.jsonl"), "utf8").startsWith(prefix));
    assert.equal(manifest.external_logs.status, "not_acquired");
    const idle = readState(f.root); assert.equal(idle.status, "idle"); assert.equal(idle.verification.repository_verify.status, "not_run");
    assert.equal(idle.sprint_disposition, undefined);
    assert.equal(runWorkflow(f.root, "archive").status, 1);
    f.run("start", "--sprint-id", "new-synthetic-mission", "--mission-title", "New bounded mission");
    const fresh = readState(f.root); assert.equal(fresh.status, "intake"); assert.equal(fresh.decision_resolution, null); assert.equal(fresh.verification.repository_verify.status, "not_run"); assert.deepEqual(fresh.active_decision_ids, []);
    assert.equal(runWorkflow(f.root, "transition", "--to", "implementation", "--expected-sequence", "1").status, 1);
  }));
}

test("unknown current bytes, operation files, replacement hashes and archive conflicts refuse without writes or deleting recovery", () => {
  for (const damage of ["current", "unknown-current-file", "operation-file", "replacement", "record", "published-file", "published-hash", "staged-file", "receipt-conflict"]) withGovernance((f) => {
    f.run(...f.args());
    injectGovernance(f.root, ["archive"], damage.startsWith("published") ? "published" : "prepared");
    const p = pendingRecord(f.root), op = join(f.root, ".ai/workflow/.governance-operation"), archive = join(f.root, ".ai/workflow/HISTORY", f.id);
    if (damage === "unknown-current-file") writeFileSync(join(f.root, ".ai/workflow/unknown-original.txt"), "preserve unknown control-plane file");
    if (damage === "current") writeFileSync(join(f.root, ".ai/workflow/DECISION_REQUIRED.md"), "unknown changes must survive");
    if (damage === "operation-file") writeFileSync(join(op, "unknown.txt"), "preserve");
    if (damage === "replacement") writeFileSync(join(op, "replacement"), "unknown replacement");
    if (damage === "record") writeFileSync(p.path, `${p.bytes} `);
    if (damage === "published-file") writeFileSync(join(archive, "unknown.txt"), "preserve");
    if (damage === "published-hash") writeFileSync(join(archive, "DECISION_REQUIRED.md"), "preserve unknown archive bytes");
    if (damage === "staged-file") { mkdirSync(join(op, "archive")); writeFileSync(join(op, "archive/unknown.txt"), "preserve"); }
    if (damage === "receipt-conflict") mkdirSync(join(f.root, ".ai/workflow/.governance-receipts", p.record.id), { recursive: true });
    const before = currentBytes(f.root), retained = readFileSync(p.path);
    const result = runWorkflow(f.root, "recover-governance", "--operation-sha256", p.hash);
    assert.equal(result.status, 1, damage);
    assert.deepEqual(currentBytes(f.root), before, damage);
    assert.deepEqual(readFileSync(p.path), retained, damage);
  });
});

test("prior repository PASS stays historical after cancellation/archive and is not inherited by a new mission", () => withGovernance((f) => {
  resetCopiedWorkflowToIdle(f.root); // only this freshly synthesized fixture
  prepareResolvedTheoryDecision(f.root, { sprintId: f.id, resolved: false, resumePhase: "engineering_planning" });
  const before = readState(f.root);
  assert.equal(before.verification.repository_verify.status, "passed");
  f.request.expected_sequence = before.state_revision; f.put();
  f.run(...f.args());
  assert.deepEqual(readState(f.root).verification, before.verification);
  f.run("archive");
  f.run("start", "--sprint-id", "fresh-independent-mission", "--mission-title", "No inherited authority");
  const next = readState(f.root);
  assert.equal(next.verification.repository_verify.status, "not_run"); assert.equal(next.verification.manual_ui.status, "not_run");
  assert.equal(next.decision_resolution, null); assert.equal(next.sprint_disposition, undefined);
}));

for (const point of ["copy:CURRENT_MISSION.md", "before-publish", "before-rename:CURRENT_MISSION.md", "before-rename:WORKFLOW_STATE.json", "before-rename:EVENTS.jsonl"]) {
  test(`archive exception at ${point} retains all recovery images and safely completes on reentry`, () => withGovernance((f) => {
    f.run(...f.args());
    injectGovernance(f.root, ["archive"], point, "FAIL");
    const record = pendingRecord(f.root);
    assert.equal(runWorkflow(f.root, "archive").status, 1);
    resumeGovernance(f);
    assert.equal(readState(f.root).status, "idle");
    const receipts = join(f.root, ".ai/workflow/.governance-receipts", record.record.id, "operation.json");
    assert.deepEqual(readFileSync(receipts), record.bytes);
  }));
}

test("unsafe idle candidates and duplicate sprint archive names refuse before creating an operation", () => withGovernance((f) => {
  f.run(...f.args()); const before = currentBytes(f.root);
  const path = join(f.root, ".ai/templates/workflow-state.template.json"), template = readFileSync(path);
  const state = JSON.parse(template); state.verification.repository_verify.status = "passed";
  writeFileSync(path, JSON.stringify(state));
  assert.equal(runWorkflow(f.root, "archive").status, 1); assert.deepEqual(currentBytes(f.root), before);
  assert.equal(readdirSync(join(f.root, ".ai/workflow")).includes(".governance-operation"), false);
  writeFileSync(path, template);
  mkdirSync(join(f.root, ".ai/workflow/HISTORY", f.id), { recursive: true });
  writeFileSync(join(f.root, ".ai/workflow/HISTORY", f.id, "unique-original.txt"), "preserve original archive");
  assert.equal(runWorkflow(f.root, "archive").status, 1); assert.deepEqual(currentBytes(f.root), before);
  assert.equal(readFileSync(join(f.root, ".ai/workflow/HISTORY", f.id, "unique-original.txt"), "utf8"), "preserve original archive");
}));

test("recovery refuses damaged internal images even with freshly supplied record hash, and preserves incomplete preparations", () => withGovernance((f) => {
  injectGovernance(f.root, f.args(), "prepared");
  const p = pendingRecord(f.root), before = currentBytes(f.root);
  p.record.after["WORKFLOW_STATE.json"].sha256 = "0".repeat(64);
  writeFileSync(p.path, JSON.stringify(p.record));
  assert.equal(runWorkflow(f.root, "recover-governance", "--operation-sha256", pendingRecord(f.root).hash).status, 1);
  assert.deepEqual(currentBytes(f.root), before);
  writeFileSync(p.path, p.bytes);
  mkdirSync(join(f.root, ".ai/workflow/.governance-preparing-incomplete"));
  writeFileSync(join(f.root, ".ai/workflow/.governance-preparing-incomplete/unique.txt"), "unique incomplete preparation");
  assert.equal(runWorkflow(f.root, "start").status, 1);
  assert.equal(runWorkflow(f.root, "recover-governance", "--operation-sha256", p.hash).status, 1);
  assert.deepEqual(readFileSync(p.path), p.bytes); assert.deepEqual(currentBytes(f.root), before);
}));

test("dangling directory symlinks and symlink control files are unknown content and refuse without writes", () => {
  for (const target of ["HISTORY", ".governance-receipts", "EVENTS.jsonl"]) withGovernance((f) => {
    injectGovernance(f.root, f.args(), "prepared");
    const p = pendingRecord(f.root), path = join(f.root, ".ai/workflow", target);
    if (target === "EVENTS.jsonl") rmSync(path);
    symlinkSync(join(f.root, "missing-destination"), path);
    const result = runWorkflow(f.root, "recover-governance", "--operation-sha256", p.hash);
    assert.equal(result.status, 1); assert.deepEqual(readFileSync(p.path), p.bytes);
    assert.equal(runWorkflow(f.root, "start").status, 1);
  });
});

function recoveryImage(record, side, name, bytes) {
  const data = Buffer.from(bytes);
  record[side][name] = { sha256: digestBytes(data), base64: data.toString("base64") };
}
function decodedRecoveryImages(record, side) {
  return Object.fromEntries(Object.entries(record[side]).map(([name, image]) => [name, Buffer.from(image.base64, "base64")]));
}
function workflowTreeHashes(root) {
  const entries = {};
  const walk = (path, prefix = "") => {
    for (const entry of readdirSync(path, { withFileTypes: true })) {
      const name = `${prefix}${entry.name}`;
      if (entry.isDirectory()) { entries[`${name}/`] = null; walk(join(path, entry.name), `${name}/`); }
      else entries[name] = digestBytes(readFileSync(join(path, entry.name)));
    }
  };
  walk(join(root, ".ai/workflow"));
  return entries;
}
function assertRecoveryContradictionRefused(f, record, expectedMessage) {
  const before = decodedRecoveryImages(record, "before");
  const after = { ...before, ...decodedRecoveryImages(record, "after") };
  // Both snapshots must independently pass the real validator. Neither a bad
  // image hash nor an already invalid workflow may cause the expected refusal.
  validateImages(before, validateWorkflow);
  validateImages(after, validateWorkflow);
  for (const [name, bytes] of Object.entries(before)) assert.deepEqual(readFileSync(join(f.root, ".ai/workflow", name)), bytes);
  const path = pendingRecord(f.root).path;
  writeFileSync(path, `${JSON.stringify(record, null, 2)}\n`);
  const matchingHash = digestBytes(readFileSync(path));
  const retained = workflowTreeHashes(f.root);
  const result = runWorkflow(f.root, "recover-governance", "--operation-sha256", matchingHash);
  process.stdout.write(`${JSON.stringify({ regression: "R3-GOV-01", expected_refusal: expectedMessage.source, matching_operation_sha256: matchingHash, status: result.status, signal: result.signal, stderr: result.stderr.trim() })}\n`);
  assert.equal(result.status, 1, "Individually valid but contradictory images must refuse even with a matching operation hash");
  assert.match(result.stderr, expectedMessage);
  assert.deepEqual(workflowTreeHashes(f.root), retained, "No control-plane file, pending record or recovery material may change");
  assert.equal(pendingRecord(f.root).hash, matchingHash);
  assert.match(runWorkflow(f.root, "start").stderr, /Governance recovery required/);
}

test("R3-GOV-01 refuses cancellation recovery when before verification differs from after checkpoint", () => withGovernance((f) => {
  injectGovernance(f.root, f.args(), "prepared");
  const { record } = pendingRecord(f.root);
  const prior = JSON.parse(Buffer.from(record.before["WORKFLOW_STATE.json"].base64, "base64"));
  const after = JSON.parse(Buffer.from(record.after["WORKFLOW_STATE.json"].base64, "base64"));
  prior.verification.repository_verify = {
    status: "failed", command: "synthetic historical verifier", exit_code: 1,
    completed_at: "2026-01-01T00:00:00.000Z", repository_head: prior.repository_head,
    working_tree_digest: prior.working_tree_digest,
  };
  assert.notDeepEqual(prior.verification, after.sprint_disposition.checkpoint.verification);
  const bytes = `${JSON.stringify(prior, null, 2)}\n`;
  recoveryImage(record, "before", "WORKFLOW_STATE.json", bytes);
  writeFileSync(join(f.root, ".ai/workflow/WORKFLOW_STATE.json"), bytes); // synthetic fixture only
  assertRecoveryContradictionRefused(f, record, /Cancellation checkpoint does not match before state/);
}));

test("R3-GOV-01 refuses valid before and after chains with a different raw event prefix", () => withGovernance((f) => {
  injectGovernance(f.root, f.args(), "prepared");
  const { record } = pendingRecord(f.root);
  const prefix = Buffer.from(record.before["EVENTS.jsonl"].base64, "base64");
  const after = Buffer.from(record.after["EVENTS.jsonl"].base64, "base64");
  // Legal JSON whitespace preserves every event object and event_hash; it
  // changes only the historical bytes. A semantic event comparison misses it.
  const changed = Buffer.concat([Buffer.from(" "), after]);
  assert.deepEqual(JSON.parse(changed.toString("utf8").split("\n")[0]), JSON.parse(prefix.toString("utf8").split("\n")[0]));
  assert.equal(changed.subarray(0, prefix.length).equals(prefix), false);
  recoveryImage(record, "after", "EVENTS.jsonl", changed);
  assertRecoveryContradictionRefused(f, record, /Cancellation events do not preserve before prefix/);
}));

test("R3-GOV-01 refuses extra appended event-log lines even when both snapshots validate", () => withGovernance((f) => {
  injectGovernance(f.root, f.args(), "prepared");
  const { record } = pendingRecord(f.root);
  const after = Buffer.from(record.after["EVENTS.jsonl"].base64, "base64");
  recoveryImage(record, "after", "EVENTS.jsonl", Buffer.concat([after, Buffer.from("\n")]));
  assertRecoveryContradictionRefused(f, record, /Cancellation must append exactly one event line/);
}));

test("R3-GOV-01 preserves original CRLF and unterminated event prefixes during normal recovery", () => {
  for (const ending of ["crlf", "unterminated"]) withGovernance((f) => {
    const path = join(f.root, ".ai/workflow/EVENTS.jsonl");
    const original = readFileSync(path, "utf8");
    const prefix = Buffer.from(ending === "crlf" ? original.replaceAll("\n", "\r\n") : original.trimEnd());
    writeFileSync(path, prefix);
    const before = readState(f.root);
    assert.deepEqual(validateWorkflow(f.root), []);
    injectGovernance(f.root, f.args(), "write:EVENTS.jsonl");
    resumeGovernance(f);
    const events = readFileSync(path);
    assert.ok(events.subarray(0, prefix.length).equals(prefix), ending);
    assert.deepEqual(readState(f.root).sprint_disposition.checkpoint, before);
    assert.equal(events.toString("utf8").trimEnd().split(/\r?\n/).length, before.state_revision + 1);
  });
});
