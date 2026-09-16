import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { copyFileSync, cpSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import test from "node:test";

import { createTransition, loadContract, validateWorkflow } from "./ai-workflow.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");

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
