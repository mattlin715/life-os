import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import test from "node:test";
const read=(path)=>readFileSync(path,"utf8").replaceAll("\r\n","\n");

test("exact M2-C frontend and fixed isolated identity",()=>{
  const files=execFileSync("git",["ls-files","--cached","--others","--exclude-standard","--","src/android-m2c"],{encoding:"utf8",windowsHide:true}).trim().split(/\r?\n/).sort();
  assert.deepEqual(files,["AndroidM2CApp.tsx","AndroidM2CApp.test.tsx","ReflectionJourney.tsx","ReflectionJourney.test.tsx","androidM2CStore.ts","android-m2c.css","journeyCopy.ts","PilotModelPanels.tsx","pilotModel.ts","pilotModel.test.ts","pilotModelCopy.ts"].map(v=>"src/android-m2c/"+v).sort());
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
  for(const token of ["E::Create","E::CorrectPending","E::ConfirmPending","E::RejectPending","R::CreateSuggested","R::CreateSuggestedBatch","R::SaveResponse","R::CorrectResponse","R::SkipSuggested","verify_scope","exact_dependencies","request_identity","reconcile(path, r)"])assert.ok(adapter.includes(token),token);
  assert.doesNotMatch(adapter,/R::DeleteAnswered/);
  for(const kind of ["evidence","reflection"]){const writer=read(`src-tauri/src/schema_v5_${kind}_write.rs`);assert.match(writer,/execute_with_adapter_origin\(path, command, context, adapter, false\)/);assert.match(writer,/execute_with_adapter_origin\(path, command, context, &SqlCommitOutcomeAdapter, true\)/);assert.match(writer,/BEGIN IMMEDIATE/);assert.match(writer,/verify_read_only_origin\(path, &prepared.operation_manifest, direct\)/)}
  const facade=read("src-tauri/src/android_m2c.rs");assert.match(facade,/with_m2c_storage/);assert.match(facade,/mutate_experience_direct_fresh/);assert.doesNotMatch(facade,/refuse_non_synthetic_dependencies|com\.lifeos\.app|println!|dbg!/);
});
test("first/append uses a frozen batch through the Android-only locked canonical path",()=>{
  const store=read("src/android-m2c/androidM2CStore.ts"), ui=read("src/android-m2c/ReflectionJourney.tsx");
  const facade=read("src-tauri/src/android_m2c.rs"), app=read("src-tauri/src/lib.rs");
  const adapter=read("src-tauri/src/schema_v5_android_reflection.rs").split("#[cfg(test)]")[0];
  assert.match(store,/invoke\("m2c_generate_questions"/);
  assert.match(app,/android_m2c::m2c_generate_questions/);
  assert.match(ui,/await questionBatchRequest\(source, rows, refs, locale/);
  assert.match(ui,/store\.generateQuestions\(request, debugPhase\)/);
  assert.match(ui,/batchVisible\(request, actual\)/);
  assert.match(facade,/with_m2c_storage\(app, \|path\|/);
  assert.match(facade,/request\.operation == "question" && !cfg!\(debug_assertions\)/);
  assert.match(adapter,/execute_direct_fresh\([\s\S]*?R::CreateSuggestedBatch/);
  assert.match(adapter,/verify_batch_items\(path, &added, &members\)\.await\?/);
  assert.match(adapter,/ORDER BY h\.created_at,h\.id/);
});

test("canonical R2 source-linkage policy stays pinned outside the isolated M3 dispatch",()=>{
  for(const [kind,hash] of [["evidence","a91999686be17067cad6aef35e1308e4fb2005266644ab39a7ffda2ca3708409"],["reflection","340360238d461959afc145795b1fd3cdeb7c09e5749c02d8ed3e6392b6e3ec7c"]]){
    let policy=read(`src-tauri/src/schema_v5_${kind}_write.rs`).split("async fn prepare_write(")[0];
    if(kind==="reflection"){
      for(const marker of ["CreateAnsweredModelTrial {\n","ReflectionWriteCommand::CreateAnsweredModelTrial {"]){
        const start=policy.indexOf('    #[cfg(feature = "android-pilot-online")]\n'+(marker.startsWith("Reflection")?"        ":"    ")+marker);
        if(marker.startsWith("Reflection")){
          const arm=policy.indexOf('        #[cfg(feature = "android-pilot-online")]\n        '+marker);
          const end=policy.indexOf("        ReflectionWriteCommand::CreateSuggested {",arm);
          assert.ok(arm>=0 && end>arm);
          const body=policy.slice(arm,end);
          assert.match(body,/create_suggested\(connection,/);
          assert.match(body,/save_response\(connection,/);
          policy=policy.slice(0,arm)+policy.slice(end);
        }else{
          const end=policy.indexOf("    SaveResponse {",start);
          assert.ok(start>=0 && end>start);
          policy=policy.slice(0,start)+policy.slice(end);
        }
      }
    }
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

// Saved-response R1 and first/append R1 extend only the authorized M2-C handlers; historical native assertions stay pinned.
test("UX001 and authorized R2 rendered mutation and input handlers",async()=>{
  const ts=(await import("typescript")).default;
  // Pilot starts the existing clue correction with an empty draft for a verbatim quote; no change to save or confirmation handlers.
  for(const [path,expected] of [["src/android-m2c/AndroidM2CApp.tsx","d2ccc9d3fa398cc23fdf0193ce7aa682d45bda3fe78a4b414ea1e63825a87b18"],["src/android-m2c/ReflectionJourney.tsx","bc1d26ca9cde216fbc414b0a7d22cfc7e0c3b074073aaa1d9c93172b6c78136f"]]){
    const code=read(path),file=ts.createSourceFile("ui.tsx",code,ts.ScriptTarget.Latest,true,ts.ScriptKind.TSX),values=[];
    const walk=n=>{if(ts.isJsxAttribute(n)&&["onClick","onChange"].includes(n.name.getText(file)))values.push(n.getText(file));ts.forEachChild(n,walk)};walk(file);
    assert.equal(createHash("sha256").update(JSON.stringify(values)).digest("hex"),expected,path);
  }
});
test("UX001 preserves accepted behavior-bearing blocks and protected contracts",()=>{
  // Pilot's visible clue state replaces generic success; canonical reread and protected write contracts stay pinned.
  // R3 adds readback-only historical model presentation; mutation handlers stay pinned above.
  const blocks=[["src/android-m2c/AndroidM2CApp.tsx","export function AndroidM2CApp","  if (storage !== \"ready\")","8def1e937883c2e6a11668616d4b7fde0c5364743fcfc883be15627971ea49ae"],["src/android-m2c/ReflectionJourney.tsx","export function currentEvidence","  return <section data-testid=\"m2c-journey\"","37d92d9dd85cec6df7bd602d2b8c5601009796339bc8f8ff2bec6100dc817c07"]];
  for(const [p,start,end,expected] of blocks){const code=read(p);assert.equal(createHash("sha256").update(code.slice(code.indexOf(start),code.indexOf(end))).digest("hex"),expected,p);}
  for(const [p,expected] of [["src/android-m2c/androidM2CStore.ts","5c3e56b7c68d24b30988a26569da888489fa281fd4d4d5fbf3f48d66a785957e"],["src/ai/providers/placeholderProvider.ts","17f27a8dd71ad70a8c74f03dcd9a83824852dd70a317d1a3cf8cf99cefafda53"],["src/ai/harness/contextPacket.ts","bea4eab27ea715e42165f71ad8d953dee28695e9b888c03f9a3ccedb180e75c0"],["src/ai/harness/gateDecision.ts","68a72ff9d810c587188b66652835b3ee80c094c118e75f2f3824040096daaa56"],["src/ai/harness/reflectionResponse.ts","ea19c1d36933abca4528bf2a907ec6e8d6a2751d59b8de7bf87dd49f6b91f519"],["src/types/domain.ts","44a365a37e18212a57d578a1b36c39f22055bea873aeb76afb2640243e7fc784"],["src-tauri/src/android_m2c.rs","3da34a0aa4553cc1af47baf29e82da70b335eb860c9c4e00f5f7753eef62a560"],["src-tauri/src/schema_v5_android_reflection.rs","91fb6f37d224f82b94ed5a1fba9d3f1aea093002e6c787c1a89d54de447e6642"],["src-tauri/src/schema_v5_evidence_write.rs","67b8f93164069b46ee63fdbda72c48efc61991242fa9e2199e112ef08edd6593"],["src-tauri/src/schema_v5_reflection_write.rs","e188e83bad51d1c7f46b01b23071cb51947969e81867cd3cd3fee641fdd85fb2"],["src-tauri/tauri.android.conf.json","360eabf48c9cc0f5ebd5703d6b9ecf8ea6cbbb81f7473532ba8f480049fd93d4"]])assert.equal(createHash("sha256").update(read(p)).digest("hex"),expected,p);
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
  assert.match(journey,/onClick=\{\(\) => void submit\("candidate", null\)\}>\{guided \? c\.createDemoClue : candidateGenerationLabel\(rows, locale\)\}/);
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

// Source-level pilot boundary; final ABI, permissions and signature are checked on the APK.
test("pilot is opt-in, fail-closed and does not turn on historical debug hooks",()=>{
  const pilot=JSON.parse(read("src-tauri/tauri.android-pilot.conf.json"));
  assert.equal(pilot.identifier,"com.lifeos.review.pilot");
  assert.equal(pilot.build.devUrl,null);
  assert.equal(pilot.build.frontendDist,"../dist"); // A Windows drive path can deserialize as a URL before Directory.
  assert.deepEqual(pilot.app.security.capabilities,["android-m2c"]);
  const vite=read("vite.config.ts");
  assert.match(vite,/VITE_LIFE_OS_ANDROID_M2C": JSON.stringify\(mode === "android-m2c" \|\| mode === "android-pilot"/);
  assert.match(vite,/VITE_LIFE_OS_ANDROID_M2C_DEBUG_HOOKS": JSON.stringify\(mode === "android-m2c" \?/);
  const storage=read("src-tauri/src/android_m2a.rs"), ipc=read("src-tauri/src/android_m2c.rs");
  assert.match(storage,/M2C_APPLICATION_ID: &str = if cfg!\(feature = "android-pilot-online"\) \{\n\s+"com\.lifeos\.review\.pilot\.model"\n\} else if cfg!\(feature = "android-pilot"\)/);
  assert.match(storage,/if app.config\(\).identifier != M2C_APPLICATION_ID/);
  assert.match(storage,/application_id: M2C_APPLICATION_ID/);
  assert.match(storage,/android-pilot-disposable-v5\.db/);
  assert.match(ipc,/"applicationId":M2C_APPLICATION_ID/);
  const lib=read("src-tauri/src/lib.rs");
  assert.match(lib,/compile_error!\("The Android pilot must be built without debug assertions/);
  assert.equal((lib.match(/#\[cfg\(not\(feature = "android-pilot"\)\)\]\n\s+android_m2[ab]::/g)??[]).length,13);
});

test("connected Founder Pilot is an explicit isolated HTTPS variant with local-only Save",()=>{
  const offline=read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");
  const manifest=read("src-tauri/android-pilot-online/AndroidManifest.xml");
  assert.doesNotMatch(offline,/android\.permission\.INTERNET/);
  assert.match(manifest,/android\.permission\.INTERNET/);
  assert.match(manifest,/android:usesCleartextTraffic="false"/);
  assert.match(manifest,/android:allowBackup="false"/);
  const config=JSON.parse(read("src-tauri/tauri.android-pilot-online.conf.json"));
  assert.equal(config.identifier,"com.lifeos.review.pilot.model");
  const cargo=read("src-tauri/Cargo.toml"),lib=read("src-tauri/src/lib.rs");
  assert.match(cargo,/android-pilot-online = \["android-pilot", "dep:reqwest"\]/);
  for(const method of ["has_key","set_key","clear_key","ask","save","feedback"]) assert.match(lib,new RegExp('#\\[cfg\\(feature = "android-pilot-online"\\)\\]\\s+android_pilot_model::pilot_model_'+method));
  const model=read("src-tauri/src/android_pilot_model.rs");
  assert.match(model,/key: Mutex<Option<HeaderValue>>/);
  assert.match(model,/set_sensitive\(true\)/);
  assert.match(model,/https:\/\/generativelanguage\.googleapis\.com\/v1beta\/models\/gemini-3\.1-flash-lite:generateContent/);
  assert.match(model,/\.redirect\(reqwest::redirect::Policy::none\(\)\)/);
  assert.match(model,/\.retry\(reqwest::retry::never\(\)\)/);
  assert.doesNotMatch(model,/std::fs|println!|dbg!|ContextPacket|writeLocale|localStorage/);
  const save=model.split("fn pilot_model_save")[1].split("#[cfg(test)]")[0];
  assert.match(save,/save_model_trial\(/);
  assert.doesNotMatch(save,/reqwest|\.send\(|ENDPOINT|\.post\(/);
});

test("R2 keeps synthetic consent and the source-only exception bounded to the connected Pilot",()=>{
  const writer=read("src-tauri/src/schema_v5_reflection_write.rs");
  const helper=writer.split("fn is_source_only_model_prompt(")[1].split("fn validate_prompt(")[0];
  assert.match(helper,/cfg!\(feature = "android-pilot-online"\)/);
  for (const qualifier of ["gemini","gemini-3.1-flash-lite","harness-v1","android-gemini-reflection-v2"]) {
    assert.ok(helper.includes('"'+qualifier+'"'));
  }
  assert.equal((writer.match(/create_suggested\(connection, &expected_source_revision_id, prompt, context, false\)/g)??[]).length,2);
  assert.match(writer,/source_only && expected_status != "answered"/);

  const model=read("src-tauri/src/android_pilot_model.rs");
  for (const command of ["ask","feedback"]) {
    const body=model.split("fn pilot_model_"+command+"(")[1].split("send_body(key.clone(), &body).await?")[0];
    assert.match(body,/if !request\.synthetic_confirmed/);
    assert.match(body,/spawn_blocking/);
    assert.match(body,/session\.sending\.swap\(true/);
  }
  assert.match(model,/original\.contains\(quote\)/);
  const feedback=model.split("fn pilot_model_feedback(")[1].split("fn pilot_model_save(")[0];
  assert.doesNotMatch(feedback,/save_model_trial|session\.ticket/);
  const main=read("src/android-m2c/ReflectionJourney.tsx");
  assert.doesNotMatch(main,/pilot-local-secondary/);
  assert.match(main,/pilot-saved-model-pairs/);
  assert.match(main,/: null : renderExisting\(\)\}/);
  assert.match(main,/if \(!actual.some/);
  assert.match(main,/throw "save_unconfirmed"/);
});

test("R3 presentation removes connected demo and focus without altering native persistence",()=>{
  const panels=read("src/android-m2c/PilotModelPanels.tsx");
  assert.doesNotMatch(panels,/pilot-focus|c\.focus|validFocus|c\.generated/);
  assert.match(panels,/controller\.preview\(\{ sourceId: input\.sourceId, sourceRevisionId: input\.sourceRevisionId,\s+original: input\.original, locale: input\.locale \}\)/);
  assert.match(panels,/c\.questionOrigin/);
  assert.match(panels,/c\.feedbackOrigin.*c\.sessionFeedback/);
});

test("R3 credential retention is explicit, native-only and outside ordinary app storage",()=>{
  const frontend=read("src/android-m2c/pilotModel.ts"),panel=read("src/android-m2c/PilotModelPanels.tsx");
  assert.match(frontend,/setKey: \(key, remember = false\)/);
  assert.match(panel,/useState\(false\)/);
  assert.match(panel,/model\.setKey\(value, remember\)/);
  assert.match(panel,/setRemembered\(!clear && remember\)/);
  assert.doesNotMatch(frontend,/loadKey|replaceKey|localStorage|sessionStorage/);
  const native=read("src-tauri/src/android_pilot_credentials.rs");
  assert.doesNotMatch(native,/#\[tauri::command\]|println!|log::|\.invoke_handler/);
  const model=read("src-tauri/src/android_pilot_model.rs");
  assert.match(model,/remember\.unwrap_or\(false\)/);
  assert.match(model,/async fn credential_task[\s\S]*spawn_blocking/);
  assert.match(model,/credential_operation\.lock\(\)/);
  const java=read("src-tauri/android-pilot-online/java/com/lifeos/pilot/credentials/AndroidCredentialBackend.java");
  for(const token of ["AndroidKeyStore","AES/GCM/NoPadding","getNoBackupFilesDir()","AtomicFile","wrappingKey(false)","updateAAD(aad)","setRandomizedEncryptionRequired(true)"])assert.ok(java.includes(token));
  assert.doesNotMatch(java,/SharedPreferences|SQLite|Log\.|System\.out|printStackTrace|EXTERNAL_STORAGE/);
  const plugin=read("src-tauri/android-pilot-online/java/com/lifeos/pilot/credentials/PilotCredentialsPlugin.java");
  assert.doesNotMatch(plugin,/invoke\.reject\([^;]*error|Log\.|printStackTrace/);
});

test("R3 forget commits disabled state before wrapping-key destruction and never manufactures a recovered key",()=>{
  const store=read("src-tauri/android-pilot-online/java/com/lifeos/pilot/credentials/CredentialStore.java");
  assert.match(store,/backend\.write\(new byte\[0\]\);\s+backend\.destroy\(\)/);
  assert.match(store,/blob == null \|\| blob\.length == 0/);
  const backend=read("src-tauri/android-pilot-online/java/com/lifeos/pilot/credentials/AndroidCredentialBackend.java");
  assert.match(backend,/if \(!create\) throw new Exception/);
  assert.match(backend,/file\.finishWrite\(output\)/);
  assert.match(backend,/file\.failWrite\(output\)/);
  const lib=read("src-tauri/src/lib.rs");
  assert.match(lib,/#\[cfg\(all\(any\(target_os = "android", test\), feature = "android-pilot-online"\)\)\]\s+mod android_pilot_credentials/);
  assert.match(lib,/#\[cfg\(feature = "android-pilot-online"\)\]\s+let builder = builder\.manage\(android_pilot_model::ModelSession::default\(\)\)\s+\.plugin\(android_pilot_credentials::init\(\)\);/);
});

test("R3 connected native build refuses missing or stale credential Java staging",()=>{
  const build=read("src-tauri/build.rs"),guard=read("src-tauri/build_support/pilot_credentials_inputs.rs");
  assert.match(build,/CARGO_CFG_TARGET_OS/);
  assert.match(build,/CARGO_FEATURE_ANDROID_PILOT_ONLINE/);
  assert.match(build,/pilot_credentials_inputs::verify/);
  assert.match(guard,/symlink_metadata/);
  assert.match(guard,/file_type\(\)\.is_file\(\)/);
  assert.match(guard,/fs::read\(input\)/);
  assert.match(guard,/credential_source_missing/);
  assert.match(guard,/credential_source_differs/);
});
