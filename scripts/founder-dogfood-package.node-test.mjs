import assert from "node:assert/strict";
import { mkdtemp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import {
  FOUNDER_IDENTIFIER,
  createManifest,
  readWindowsSubsystem,
  validateManifest,
  validateFounderBinary,
  validateSourceContract,
  validateWindowsGuiBinary,
} from "./founder-dogfood-package.mjs";

const SHA = "0123456789abcdef0123456789abcdef01234567";
const TARGET = "x86_64-pc-windows-msvc";

async function fixtureRoot() {
  const root = await mkdtemp(path.join(tmpdir(), "life-os-founder-package-"));
  await Promise.all([
    mkdir(path.join(root, "src-tauri", "src"), { recursive: true }),
    mkdir(path.join(root, "src", "shared", "storage", "sqlite"), { recursive: true }),
    mkdir(path.join(root, "scripts"), { recursive: true }),
  ]);
  await writeFile(
    path.join(root, "src-tauri", "tauri.conf.json"),
    JSON.stringify({
      productName: "Life OS",
      version: "0.2.0",
      identifier: "com.lifeos.app",
      app: { windows: [{ title: "Life OS", width: 1000, height: 760, minWidth: 720, minHeight: 560 }] },
    }),
  );
  await writeFile(
    path.join(root, "src-tauri", "tauri.founder-dogfood.conf.json"),
    JSON.stringify({
      $schema: "https://schema.tauri.app/config/2",
      productName: "Life OS Founder Dogfood",
      identifier: FOUNDER_IDENTIFIER,
      app: {
        windows: [
          {
            title: "Life OS — Founder Dogfood (Private)",
            width: 1000,
            height: 760,
            minWidth: 720,
            minHeight: 560,
          },
        ],
      },
      bundle: { active: true, targets: ["nsis"], windows: { nsis: { installMode: "currentUser" } } },
    }),
  );
  await writeFile(path.join(root, "package.json"), JSON.stringify({ version: "0.2.0" }));
  await writeFile(path.join(root, "src-tauri", "src", "sqlite.rs"), "const SCHEMA_VERSION: i64 = 4;\n");
  await writeFile(
    path.join(root, "src", "shared", "storage", "sqlite", "sqliteLocalEvidenceStore.ts"),
    'const DATABASE_PATH = "sqlite:life-os.db";\n',
  );
  await writeFile(path.join(root, ".gitignore"), ".artifacts/\n");
  await writeFile(
    path.join(root, "scripts", "build-founder-dogfood-package.ps1"),
    [
      "$founderTauriConfig = Get-Content -Raw -Encoding UTF8 $configPath",
      "$previousTauriConfig = $env:TAURI_CONFIG",
      "try {",
      "  $env:TAURI_CONFIG = $founderTauriConfig",
      "  cargo rustc --manifest-path x --release --bin life-os --features 'tauri/custom-protocol' -- -C 'link-arg=/SUBSYSTEM:WINDOWS' -C 'link-arg=/ENTRY:mainCRTStartup'",
      "} finally {",
      "  if ($null -eq $previousTauriConfig) { Remove-Item Env:TAURI_CONFIG -ErrorAction SilentlyContinue }",
      "  else { $env:TAURI_CONFIG = $previousTauriConfig }",
      "}",
      "pnpm exec tauri bundle --config $configPath --bundles nsis --ci --no-sign",
    ].join("\n"),
  );
  return root;
}

function syntheticPe(subsystem) {
  const binary = Buffer.alloc(512);
  binary.write("MZ", 0, "ascii");
  binary.writeUInt32LE(128, 0x3c);
  binary.write("PE\0\0", 128, "ascii");
  binary.writeUInt16LE(0x8664, 132);
  binary.writeUInt16LE(0xf0, 148);
  binary.writeUInt16LE(0x20b, 152);
  binary.writeUInt16LE(subsystem, 152 + 68);
  return binary;
}

test("validates the isolated schema-v4 Founder package source contract", async (t) => {
  const root = await fixtureRoot();
  t.after(() => rm(root, { recursive: true, force: true }));
  const result = await validateSourceContract(root, { inspectGit: false });
  assert.equal(result.founderIdentifier, FOUNDER_IDENTIFIER);
  assert.equal(result.schemaVersion, 4);
});

test("rejects an identity collision, schema drift, and forbidden override surface", async (t) => {
  const root = await fixtureRoot();
  t.after(() => rm(root, { recursive: true, force: true }));
  const overridePath = path.join(root, "src-tauri", "tauri.founder-dogfood.conf.json");
  const valid = JSON.parse(await readFile(overridePath, "utf8"));

  await writeFile(overridePath, JSON.stringify({ ...valid, identifier: "com.lifeos.app" }));
  await assert.rejects(validateSourceContract(root, { inspectGit: false }), /Founder identifier/);

  await writeFile(overridePath, JSON.stringify(valid));
  await writeFile(path.join(root, "src-tauri", "src", "sqlite.rs"), "const SCHEMA_VERSION: i64 = 5;\n");
  await assert.rejects(validateSourceContract(root, { inspectGit: false }), /must remain 4/);

  await writeFile(path.join(root, "src-tauri", "src", "sqlite.rs"), "const SCHEMA_VERSION: i64 = 4;\n");
  await writeFile(overridePath, JSON.stringify({ ...valid, plugins: { updater: {} } }));
  await assert.rejects(validateSourceContract(root, { inspectGit: false }), /unexpected keys/);
});

test("rejects a direct final-binary compile that omits the Founder config", async (t) => {
  const root = await fixtureRoot();
  t.after(() => rm(root, { recursive: true, force: true }));
  const buildScript = path.join(root, "scripts", "build-founder-dogfood-package.ps1");
  await writeFile(
    buildScript,
    "cargo rustc --manifest-path x --release --bin life-os --features 'tauri/custom-protocol' -- -C 'link-arg=/SUBSYSTEM:WINDOWS' -C 'link-arg=/ENTRY:mainCRTStartup'\npnpm exec tauri bundle --config $configPath --bundles nsis --ci --no-sign\n",
  );
  await assert.rejects(validateSourceContract(root, { inspectGit: false }), /override JSON must be loaded/);
});

test("creates and verifies an exact content-free manifest", async (t) => {
  const root = await mkdtemp(path.join(tmpdir(), "life-os-founder-artifact-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const artifactPath = path.join(root, "Life-OS-Founder-Dogfood-R1.exe");
  await writeFile(artifactPath, Buffer.from("synthetic installer bytes"));
  const manifest = await createManifest({
    artifactPath,
    applicationVersion: "0.2.0",
    gitSha: SHA,
    buildTarget: TARGET,
  });
  assert.deepEqual(Object.keys(manifest), [
    "applicationVersion",
    "gitSha",
    "buildTarget",
    "artifactFilename",
    "artifactSize",
    "sha256",
  ]);
  await validateManifest(manifest, {
    artifactPath,
    applicationVersion: "0.2.0",
    gitSha: SHA,
    buildTarget: TARGET,
  });
});

test("accepts only a well-formed Windows GUI application subsystem", async (t) => {
  const root = await mkdtemp(path.join(tmpdir(), "life-os-founder-pe-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const gui = path.join(root, "gui.exe");
  const console = path.join(root, "console.exe");
  const malformed = path.join(root, "malformed.exe");
  await writeFile(gui, syntheticPe(2));
  await writeFile(console, syntheticPe(3));
  await writeFile(malformed, Buffer.from("not a PE file"));

  assert.equal(await readWindowsSubsystem(gui), 2);
  await validateWindowsGuiBinary(gui);
  await assert.rejects(validateWindowsGuiBinary(console), /must be GUI \(2\), found 3/);
  await assert.rejects(validateWindowsGuiBinary(malformed), /valid DOS header/);
});

test("requires the Founder identifier and private title in the GUI binary", async (t) => {
  const root = await mkdtemp(path.join(tmpdir(), "life-os-founder-markers-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const valid = path.join(root, "valid.exe");
  const missing = path.join(root, "missing.exe");
  await writeFile(
    valid,
    Buffer.concat([
      syntheticPe(2),
      Buffer.from(`\0${FOUNDER_IDENTIFIER}\0Life OS — Founder Dogfood (Private)\0`, "utf8"),
    ]),
  );
  await writeFile(missing, syntheticPe(2));
  await validateFounderBinary(valid);
  await assert.rejects(validateFounderBinary(missing), /identifier is not embedded/);
});

test("rejects manifest path leakage, extra fields, size drift, and digest drift", async (t) => {
  const root = await mkdtemp(path.join(tmpdir(), "life-os-founder-manifest-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const artifactPath = path.join(root, "installer.exe");
  await writeFile(artifactPath, Buffer.from("closed bytes"));
  const valid = await createManifest({
    artifactPath,
    applicationVersion: "0.2.0",
    gitSha: SHA,
    buildTarget: TARGET,
  });
  const expected = { artifactPath, applicationVersion: "0.2.0", gitSha: SHA, buildTarget: TARGET };

  await assert.rejects(validateManifest({ ...valid, artifactFilename: "C:\\private\\installer.exe" }, expected), /must not contain a path/);
  await assert.rejects(validateManifest({ ...valid, userPath: "C:\\private" }, expected), /unexpected keys/);
  await assert.rejects(validateManifest({ ...valid, artifactSize: valid.artifactSize + 1 }, expected), /size mismatch/);
  await assert.rejects(validateManifest({ ...valid, sha256: "0".repeat(64) }, expected), /SHA-256 mismatch/);
});

test("the real repository satisfies the package source contract", async () => {
  const repositoryRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname.replace(/^\/(.:)/, "$1")), "..");
  const result = await validateSourceContract(repositoryRoot);
  assert.equal(result.developmentIdentifier, "com.lifeos.app");
  assert.equal(result.founderIdentifier, FOUNDER_IDENTIFIER);
});
