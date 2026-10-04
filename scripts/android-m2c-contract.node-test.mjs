import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import test from "node:test";
const read=(path)=>readFileSync(path,"utf8").replaceAll("\r\n","\n");

test("exact M2-C frontend and fixed isolated identity",()=>{
  const files=execFileSync("git",["ls-files","--cached","--others","--exclude-standard","--","src/android-m2c"],{encoding:"utf8",windowsHide:true}).trim().split(/\r?\n/).sort();
  assert.deepEqual(files,["AndroidM2CApp.tsx","AndroidM2CApp.test.tsx","ReflectionJourney.tsx","ReflectionJourney.test.tsx","androidM2CStore.ts","android-m2c.css","journeyCopy.ts"].map(v=>"src/android-m2c/"+v).sort());
  const config=JSON.parse(read("src-tauri/tauri.android.conf.json"));assert.equal(config.identifier,"com.lifeos.review.m2c");assert.deepEqual(config.app.security.capabilities,["android-m2c"]);
  assert.deepEqual(JSON.parse(read("src-tauri/capabilities/android-m2c.json")).permissions,["core:default"]);
  assert.match(read("src-tauri/gen/android/app/build.gradle.kts"),/applicationId = "com\.lifeos\.review\.m2c"/);
  assert.match(read("src-tauri/gen/android/app/src/main/java/com/lifeos/feasibility/m0/MainActivity.kt"),/^package com\.lifeos\.review\.m2c/);
  const manifest=read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");assert.doesNotMatch(manifest,/uses-permission|FileProvider/);assert.match(manifest,/allowBackup="false"/);
});
test("mobile entry precedes desktop and never imports provider transport",()=>{
  const main=read("src/main.tsx");assert.ok(main.indexOf("VITE_LIFE_OS_ANDROID_M2C")<main.indexOf('import("./app/App")'));
  for(const old of ["M2B","M2A","M1","FEASIBILITY_M0"])assert.ok(main.includes("VITE_LIFE_OS_ANDROID_"+old));
  for(const p of ["src/android-m2c/AndroidM2CApp.tsx","src/android-m2c/ReflectionJourney.tsx","src/android-m2c/androidM2CStore.ts"]){assert.doesNotMatch(read(p),/localStorage|console\.|fetch\(|openaiProvider|geminiProvider|sharedJsonProvider|suggestPatternNotes|generateHistoricalReflectionQuestions|createLocalEvidenceStoreRuntime/)}
});
test("canonical transaction routing and explicit direct-origin verification",()=>{
  const adapter=read("src-tauri/src/schema_v5_android_reflection.rs").split("#[cfg(test)]")[0];
  assert.doesNotMatch(adapter,/INSERT|UPDATE |DELETE FROM|INSERT OR REPLACE|migrate_disposable|schema_migration_receipts/);
  for(const token of ["E::Create","E::CorrectPending","E::ConfirmPending","E::RejectPending","R::CreateSuggested","R::SaveResponse","R::SkipSuggested","verify_scope","exact_dependencies","request_identity","reconcile(path, r)"])assert.ok(adapter.includes(token),token);
  assert.doesNotMatch(adapter,/R::CorrectResponse|R::DeleteAnswered/);
  for(const kind of ["evidence","reflection"]){const writer=read(`src-tauri/src/schema_v5_${kind}_write.rs`);assert.match(writer,/execute_with_adapter_origin\(path, command, context, adapter, false\)/);assert.match(writer,/execute_with_adapter_origin\(path, command, context, &SqlCommitOutcomeAdapter, true\)/);assert.match(writer,/BEGIN IMMEDIATE/);assert.match(writer,/verify_read_only_origin\(path, &prepared.operation_manifest, direct\)/)}
  const facade=read("src-tauri/src/android_m2c.rs");assert.match(facade,/with_m2c_storage/);assert.match(facade,/mutate_experience_direct_fresh/);assert.doesNotMatch(facade,/refuse_non_synthetic_dependencies|com\.lifeos\.app|println!|dbg!/);
});
test("all canonical command bodies before the origin adapter remain byte-identical",()=>{
  for(const [kind,hash] of [["evidence","a91999686be17067cad6aef35e1308e4fb2005266644ab39a7ffda2ca3708409"],["reflection","8778897dc568c17895fe9eb31ae57c02f6a4db8c645659832a68f8d639ac1b42"]]){
    const policy=read(`src-tauri/src/schema_v5_${kind}_write.rs`).split("async fn prepare_write(")[0];
    assert.equal(createHash("sha256").update(policy).digest("hex"),hash);
  }
});
test("truthful quote presentation and shared version/provenance safeguards",()=>{
  const adapter=read("src-tauri/src/schema_v5_android_reflection.rs");
  for(const token of ["Your own words","你寫下的原文","あなたが書いた原文","local_mock","model: None","harness-v1"])assert.ok(adapter.includes(token),token);
  assert.doesNotMatch(adapter.split("#[cfg(test)]")[0],/Directly observable|從紀錄中可直接確認|記録から直接確認できること/);
  const version=read("src/ai/harness/version.ts");assert.match(version,/HARNESS_VERSION = "harness-v1"/);assert.match(version,/PROMPT_VERSION = "v1"/);
  const journey=read("src/android-m2c/ReflectionJourney.tsx");assert.match(journey,/createContextPacket/);assert.match(journey,/decideContextGate/);assert.match(journey,/generateLocalReflectionPrompts/);assert.match(journey,/answerReflectionPrompt/);assert.match(journey,/onClick=\{\(\) => void execute\(frozen\)\}/);
  assert.match(journey,/v.lifecycleState === "active" && v.reviewState === "confirmed" && v.eligibilityState === "eligible"/);
});
test("accepted APKs preserved and packaging stays debug/process-local",()=>{
  const build=read("scripts/android-m2c.ps1");
  for(const token of [".artifacts\\android-m2b\\review.apk","8be61db3f500514f01055846273460307580e3ed9246b078d2c3a50cc21eca43","85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb","--debug --target x86_64","preserved-m2b"])assert.ok(build.includes(token));
  assert.doesNotMatch(build,/setx|SetEnvironmentVariable|--release/);
});
test("native harness binds a new disposable target and preserves other devices/failures",()=>{
  const native=read("scripts/android-m2c-native-review.ps1");
  for(const token of ["emulator-5588","$port = 5588","$cdpPort = 9228","ANDROID-M2C-SYNTHETIC-REFLECTION-001","$preservedDeviceInventory","owner.apk_sha256","Native run failed; ownership/AVD/fixtures preserved","reflection-retry",".m2c-$phase.hold","Pending-Absent"])assert.ok(native.includes(token));
  assert.match(native,/if \(\$name -ne \$avdName\)/);assert.match(native,/if \(-not \$owned\)/);
  assert.doesNotMatch(native,/adb -d|emulator-5586|com\.lifeos\.app|wipe-data/);
  assert.match(read("scripts/verify.ps1"),/node --test scripts\/android-m2c-contract\.node-test\.mjs/);
});

test("Founder UI correction keeps source/policy exact and technical probes outside ordinary layout",()=>{
  const ui=read("src/android-m2c/AndroidM2CApp.tsx"),journey=read("src/android-m2c/ReflectionJourney.tsx"),css=read("src/android-m2c/android-m2c.css");
  assert.match(ui,/selectedId=\{selected\?\.entry\.id\}/);assert.match(ui,/id="m2c-source-content"/);
  assert.match(ui,/<details hidden data-debug-only="source"/);assert.match(journey,/<details hidden data-debug-only="artifact"/);
  assert.match(css,/\[hidden\] \{ display: none !important; \}/);assert.match(css,/overflow-wrap: anywhere/);
  assert.doesNotMatch(journey,/data-testid="m2c-exact-source"|<p>\{c\.disclosure\}<\/p>/);
  assert.match(journey,/record\.text === `\$\{label\}:\\n\$\{source\.entry\.body\.trim\(\)\}`/);
  assert.match(journey,/repeat \|\| reviewed/);assert.match(journey,/data-testid="m2c-technical-provenance"/);
  assert.match(ui,/journeyCopy\[locale\]\.disclosure/);assert.match(ui,/journeyCopy\[locale\]\.retention/);
});

test("UX001 preserves every rendered mutation and input handler",async()=>{
  const ts=(await import("typescript")).default;
  for(const [path,expected] of [["src/android-m2c/AndroidM2CApp.tsx","a6911fbb583496ca14133520424c43282d5f99b8da9c6a1bc12303f150db3ae4"],["src/android-m2c/ReflectionJourney.tsx","4eb7b1b3c92a8307f71ec56681628604fc195424716de0770a0bce329f4d94fc"]]){
    const code=read(path),file=ts.createSourceFile("ui.tsx",code,ts.ScriptTarget.Latest,true,ts.ScriptKind.TSX),values=[];
    const walk=n=>{if(ts.isJsxAttribute(n)&&["onClick","onChange"].includes(n.name.getText(file)))values.push(n.getText(file));ts.forEachChild(n,walk)};walk(file);
    assert.equal(createHash("sha256").update(JSON.stringify(values)).digest("hex"),expected,path);
  }
});
test("UX001 preserves accepted behavior-bearing blocks and protected contracts",()=>{
  const blocks=[["src/android-m2c/AndroidM2CApp.tsx","export function AndroidM2CApp","  if (storage !== \"ready\")","1bd208c2601a7e75f024062fd558ceb4acee47191c843936869621afa8dc4206"],["src/android-m2c/ReflectionJourney.tsx","export function currentEvidence","  return <section data-testid=\"m2c-journey\"","460e0912b25c74b8957588f3ffc7ee7c78c115606e5f642d79916c7b5da79e3f"]];
  for(const [p,start,end,expected] of blocks){const code=read(p);assert.equal(createHash("sha256").update(code.slice(code.indexOf(start),code.indexOf(end))).digest("hex"),expected,p);}
  for(const [p,expected] of [["src/android-m2c/androidM2CStore.ts","aedf074743788d38fde288785e33b058fa830463c58b5d38a0ccce6d66712d63"],["src/ai/providers/placeholderProvider.ts","17f27a8dd71ad70a8c74f03dcd9a83824852dd70a317d1a3cf8cf99cefafda53"],["src/ai/harness/contextPacket.ts","bea4eab27ea715e42165f71ad8d953dee28695e9b888c03f9a3ccedb180e75c0"],["src/ai/harness/gateDecision.ts","68a72ff9d810c587188b66652835b3ee80c094c118e75f2f3824040096daaa56"],["src/ai/harness/reflectionResponse.ts","ea19c1d36933abca4528bf2a907ec6e8d6a2751d59b8de7bf87dd49f6b91f519"],["src/types/domain.ts","44a365a37e18212a57d578a1b36c39f22055bea873aeb76afb2640243e7fc784"],["src-tauri/src/android_m2c.rs","83984c40a0b4bc1a6469b935d9082130551168a186fae90b5ac9fcf953202cac"],["src-tauri/src/schema_v5_android_reflection.rs","5fa3b0fed871e391f600f4c23ee384703f2dff7de2009dd10bb1e9bbf2702b92"],["src-tauri/src/schema_v5_evidence_write.rs","67b8f93164069b46ee63fdbda72c48efc61991242fa9e2199e112ef08edd6593"],["src-tauri/src/schema_v5_reflection_write.rs","aa87ac1bd33d4ee07e9653027d03f95d784ef9be8ed6a1cf8e81c115e0abfa7b"],["src-tauri/tauri.android.conf.json","360eabf48c9cc0f5ebd5703d6b9ecf8ea6cbbb81f7473532ba8f480049fd93d4"]])assert.equal(createHash("sha256").update(read(p)).digest("hex"),expected,p);
  const ui=read("src/android-m2c/AndroidM2CApp.tsx"),journey=read("src/android-m2c/ReflectionJourney.tsx");
  assert.match(ui,/<textarea autoFocus aria-describedby="m2c-source-hint" id="m2c-edit"/);
  assert.match(journey,/<textarea autoFocus aria-describedby="m2c-correction-help" id="m2c-correction"/);
  assert.match(journey,/<CandidateScopeNote locale=\{locale\} \/>/);
});

test("SYNC001 retains every native assertion and fails closed until exact source ready",async()=>{
  const ts=(await import("typescript")).default,vm=await import("node:vm");
  const code=read("scripts/android-m2c-cdp-probe.mjs"),file=ts.createSourceFile("probe.mjs",code,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),values=[];
  const walk=n=>{if(ts.isCallExpression(n)&&n.expression.getText(file).startsWith("assert."))values.push(n.getText(file));ts.forEachChild(n,walk)};walk(file);
  assert.equal(values.length,150);assert.equal(createHash("sha256").update(JSON.stringify(values)).digest("hex"),"1da14013804348b04a9eec5d35ab6c15175ada9211e57a924eb23403589525b1");
  const declaration=file.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="exactSourceReadiness");assert.ok(declaration);
  const predicate=vm.runInNewContext(declaration.getText(file).replace(/^export /,"")+"; exactSourceReadiness");
  const source={entry:{id:"synthetic-source",body:"Exact synthetic original"},revisionNumber:2,authorship:"user",revisionId:"v5sr_current",predecessorRevisionId:"v5sr_previous"};
  const version="revision=2 · authoredBy=user · id=v5sr_current · predecessor=v5sr_previous";
  const ready=()=>new Map([
    ['[data-runtime="android-m2c-disposable"]',{dataset:{storageState:"ready",saveState:"draft"}}],
    ['[data-experience-id="synthetic-source"]',{}],['[data-testid="m2c-exact-text"]',{textContent:source.entry.body}],
    ['[data-testid="m2c-revision"]',{textContent:version}],['[data-testid="m2c-journey"]',{dataset:{artifactState:"idle"}}],
    ['[data-testid="m2c-edit"]',{disabled:false}],['[data-debug-phase="rollbackAfterProjection"]',{disabled:false}]
  ]);
  const evaluate=map=>vm.runInNewContext(predicate(source),{document:{querySelector:s=>map.get(s)??null}});
  assert.equal(evaluate(ready()),true);
  for(const key of ready().keys()){const map=ready();map.delete(key);assert.equal(evaluate(map),false,"missing "+key)}
  for(const value of ["loading","working","failed","conflict"]){const map=ready();map.get('[data-testid="m2c-journey"]').dataset.artifactState=value;assert.equal(evaluate(map),false)}
  for(const key of ['[data-testid="m2c-edit"]','[data-debug-phase="rollbackAfterProjection"]']){const map=ready();map.get(key).disabled=true;assert.equal(evaluate(map),false)}
  for(const [key,value]of [['[data-testid="m2c-exact-text"]',"Different source"],['[data-testid="m2c-revision"]',version.replace("v5sr_current","v5sr_foreign")],['[data-testid="m2c-revision"]',version.replace("revision=2","revision=3")],['[data-testid="m2c-revision"]',version.replace("authoredBy=user","authoredBy=local_mock")],['[data-testid="m2c-revision"]',version.replace("v5sr_previous","v5sr_unknown")]]){const map=ready();map.get(key).textContent=value;assert.equal(evaluate(map),false)}
  for(const [property,value]of [["storageState","blocked"],["saveState","conflict"],["saveState","saving"]]){const map=ready();map.get('[data-runtime="android-m2c-disposable"]').dataset[property]=value;assert.equal(evaluate(map),false)}
  assert.match(code,/await wait\(exactSourceReadiness\(openedSource\), "exact source ready after reopen", 30000\)/);
  assert.match(code,/await wait\(exactSourceReadiness\(reopenedSource\), "exact source ready before fault", 30000\);\n      await click\('\[data-debug-phase="rollbackAfterProjection"\]'\)/);
  assert.doesNotMatch(declaration.getText(file),/setTimeout|invoke|\.click\(|dispatchEvent|request/);
});

test("Founder003 groups only rejected Evidence and preserves exact action/policy boundaries",async()=>{
  const ts=(await import("typescript")).default;
  const journey=read("src/android-m2c/ReflectionJourney.tsx"),copy=read("src/android-m2c/journeyCopy.ts");
  const file=ts.createSourceFile("journey.tsx",journey,ts.ScriptTarget.Latest,true,ts.ScriptKind.TSX);
  const helpers=file.statements.filter(n=>ts.isFunctionDeclaration(n)&&["isRejectedCandidate","candidateGenerationLabel","RejectedCandidateHistory"].includes(n.name?.text));
  assert.equal(helpers.length,3);
  for(const fn of helpers)assert.doesNotMatch(fn.getText(file),/store\.|invoke|submit\(|useEffect|useState|onClick|onChange|payload|\.text\b/);
  assert.match(journey,/row\.kind === "evidence" && row\.lifecycleState !== "active" && row\.reviewState === "rejected"/);
  assert.match(journey,/<RejectedCandidateHistory rows=\{rows\} locale=\{locale\} \/>/);
  assert.match(journey,/if \(isRejectedCandidate\(row\)\) return null;/);
  assert.match(journey,/rows\.every\(\(v\) => v\.kind !== "evidence" \|\| v\.lifecycleState !== "active"\)/);
  assert.match(journey,/onClick=\{\(\) => void submit\("candidate", null\)\}>\{candidateGenerationLabel\(rows, locale\)\}/);
  assert.equal((journey.match(/data-testid="m2c-rejection-summary"/g)??[]).length,1);
  assert.match(journey,/<details className="android-m2c__secondary-details" data-testid="m2c-rejection-history">/);
  assert.match(journey,/id: row\.id, kind: row\.kind, revisionId: row\.revisionId, reviewState: row\.reviewState,\n\s+lifecycleState: row\.lifecycleState, eligibilityState: row\.eligibilityState/);
  for(const label of ["Create a new local-demo candidate","建立新的本機示範候選","端末内デモの候補を新しく作る"])assert.ok(copy.includes(label));
  const probe=read("scripts/android-m2c-rejection-cdp-probe.mjs");
  assert.match(probe,/round<3/);assert.match(probe,/assert\.equal\(rejected\.length, 2\)/);
  assert.match(probe,/assert\.deepEqual\(await getRows\(source\.entry\.id\), before\)/);
  assert.match(probe,/assert\.deepEqual\(await getSource\(source\.entry\.id\), source\)/);
  assert.match(probe,/assert\.equal\(row\.payload, null\)/);
  assert.doesNotMatch(probe,/m2c_mutate_artifact|m2c_update_experience|m2c_delete_experience|pm.*clear|--action.*retry/);
});
