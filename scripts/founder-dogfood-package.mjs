import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { access, open, readFile, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

export const FOUNDER_PRODUCT_NAME = "Life OS Founder Dogfood";
export const FOUNDER_IDENTIFIER = "com.lifeos.founderdogfood";
export const FOUNDER_WINDOW_TITLE = "Life OS — Founder Dogfood (Private)";
export const DEVELOPMENT_PRODUCT_NAME = "Life OS";
export const DEVELOPMENT_IDENTIFIER = "com.lifeos.app";
export const SUPPORTED_SCHEMA_VERSION = 4;
export const MANIFEST_KEYS = Object.freeze([
  "applicationVersion",
  "gitSha",
  "buildTarget",
  "artifactFilename",
  "artifactSize",
  "sha256",
]);

const SOURCE_ALLOWLIST = new Set([
  ".gitignore",
  "scripts/build-founder-dogfood-package.ps1",
  "scripts/founder-dogfood-package.mjs",
  "scripts/founder-dogfood-package.node-test.mjs",
  "scripts/verify.ps1",
  "src-tauri/tauri.founder-dogfood.conf.json",
  "docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md",
  "docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md",
  ".ai/workflow/CURRENT_MISSION.md",
  ".ai/workflow/PRODUCT_REVIEW.md",
  ".ai/workflow/ENGINEERING_PLAN.md",
  ".ai/workflow/ENGINEERING_REPORT.md",
  ".ai/workflow/THEORY_ALIGNMENT_REVIEW.md",
  ".ai/workflow/SPRINT_REPORT.md",
  ".ai/workflow/DECISION_REQUIRED.md",
  ".ai/workflow/WORKFLOW_STATE.json",
  ".ai/workflow/EVENTS.jsonl",
]);
const HISTORY_PREFIXES = [
  ".ai/workflow/HISTORY/2026-08-12-windows-founder-dogfooding-package-r1/",
  ".ai/workflow/HISTORY/2026-08-12-windows-founder-dogfooding-package-r1-console-correction/",
  ".ai/workflow/HISTORY/2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction/",
];

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function readJson(filePath) {
  return JSON.parse(await readFile(filePath, "utf8"));
}

function exactKeys(value, expected, label) {
  assert(value && typeof value === "object" && !Array.isArray(value), `${label} must be an object.`);
  const actual = Object.keys(value).sort();
  const wanted = [...expected].sort();
  assert(JSON.stringify(actual) === JSON.stringify(wanted), `${label} has unexpected keys: ${actual.join(", ")}`);
}

function runGit(root, args, options = {}) {
  const result = spawnSync("git", args, {
    cwd: root,
    encoding: "utf8",
    windowsHide: true,
    ...options,
  });
  assert(result.status === 0, `git ${args.join(" ")} failed: ${(result.stderr || result.stdout).trim()}`);
  return result.stdout;
}

function normalizeRepositoryPath(value) {
  return value.replaceAll("\\", "/").replace(/^\.\//, "");
}

export async function sha256File(filePath) {
  await access(filePath);
  return new Promise((resolve, reject) => {
    const hash = createHash("sha256");
    const stream = createReadStream(filePath);
    stream.on("error", reject);
    stream.on("data", (chunk) => hash.update(chunk));
    stream.on("end", () => resolve(hash.digest("hex")));
  });
}

export async function validateSourceContract(root, { inspectGit = true } = {}) {
  const basePath = path.join(root, "src-tauri", "tauri.conf.json");
  const overridePath = path.join(root, "src-tauri", "tauri.founder-dogfood.conf.json");
  const packagePath = path.join(root, "package.json");
  const schemaPath = path.join(root, "src-tauri", "src", "sqlite.rs");
  const storagePath = path.join(root, "src", "shared", "storage", "sqlite", "sqliteLocalEvidenceStore.ts");
  const ignorePath = path.join(root, ".gitignore");
  const buildScriptPath = path.join(root, "scripts", "build-founder-dogfood-package.ps1");

  const [base, override, packageJson, schemaSource, storageSource, ignoreSource, buildScriptSource] = await Promise.all([
    readJson(basePath),
    readJson(overridePath),
    readJson(packagePath),
    readFile(schemaPath, "utf8"),
    readFile(storagePath, "utf8"),
    readFile(ignorePath, "utf8"),
    readFile(buildScriptPath, "utf8"),
  ]);

  assert(base.productName === DEVELOPMENT_PRODUCT_NAME, "Normal Tauri productName drifted.");
  assert(base.identifier === DEVELOPMENT_IDENTIFIER, "Normal Tauri identifier drifted.");
  assert(override.productName === FOUNDER_PRODUCT_NAME, "Founder productName is not exact.");
  assert(override.identifier === FOUNDER_IDENTIFIER, "Founder identifier is not exact.");
  assert(override.identifier !== base.identifier, "Founder and development identifiers must differ.");
  exactKeys(override, ["$schema", "productName", "identifier", "app", "bundle"], "Founder override");
  exactKeys(override.app, ["windows"], "Founder app override");
  assert(Array.isArray(override.app.windows) && override.app.windows.length === 1, "Founder override must define one window.");
  exactKeys(
    override.app.windows[0],
    ["title", "width", "height", "minWidth", "minHeight"],
    "Founder window override",
  );
  assert(override.app.windows[0].title === FOUNDER_WINDOW_TITLE, "Founder window title is not exact.");
  for (const dimension of ["width", "height", "minWidth", "minHeight"]) {
    assert(
      override.app.windows[0][dimension] === base.app.windows[0][dimension],
      `Founder ${dimension} must match the normal window.`,
    );
  }
  exactKeys(override.bundle, ["active", "targets", "windows"], "Founder bundle override");
  assert(override.bundle.active === true, "Founder bundle must be active.");
  assert(JSON.stringify(override.bundle.targets) === JSON.stringify(["nsis"]), "Founder bundle target must be NSIS only.");
  exactKeys(override.bundle.windows, ["nsis"], "Founder Windows bundle override");
  exactKeys(override.bundle.windows.nsis, ["installMode"], "Founder NSIS override");
  assert(override.bundle.windows.nsis.installMode === "currentUser", "Founder installer must be current-user only.");

  const schemaMatch = schemaSource.match(/const\s+SCHEMA_VERSION\s*:\s*i64\s*=\s*(\d+)\s*;/);
  assert(schemaMatch, "Production SCHEMA_VERSION declaration was not found.");
  assert(Number(schemaMatch[1]) === SUPPORTED_SCHEMA_VERSION, "Production SCHEMA_VERSION must remain 4.");
  assert(storageSource.includes('const DATABASE_PATH = "sqlite:life-os.db";'), "Relative SQLite database path drifted.");
  assert(!JSON.stringify(override).includes("life-os.db"), "Founder override must not supply a database path.");
  assert(packageJson.version === base.version, "Package and Tauri versions must match.");
  assert(ignoreSource.split(/\r?\n/).includes(".artifacts/"), "Founder review output must be ignored.");
  assert(buildScriptSource.includes("$founderTauriConfig = Get-Content -Raw -Encoding UTF8 $configPath"), "Founder override JSON must be loaded exactly.");
  assert(buildScriptSource.includes("$env:TAURI_CONFIG = $founderTauriConfig"), "Founder config must be embedded during final-binary compilation.");
  assert(
    buildScriptSource.includes("cargo rustc --manifest-path") &&
      buildScriptSource.includes("--release --bin life-os --features 'tauri/custom-protocol' -- -C 'link-arg=/SUBSYSTEM:WINDOWS' -C 'link-arg=/ENTRY:mainCRTStartup'"),
    "Founder package must compile only the final production-protocol GUI application binary with exact linker flags.",
  );
  assert(
    buildScriptSource.includes("pnpm exec tauri bundle --config $configPath --bundles nsis --ci --no-sign"),
    "Founder package must bundle using the same exact override.",
  );
  assert(
    buildScriptSource.includes("Remove-Item Env:TAURI_CONFIG -ErrorAction SilentlyContinue") &&
      buildScriptSource.includes("$env:TAURI_CONFIG = $previousTauriConfig"),
    "Founder package must restore the caller's TAURI_CONFIG state.",
  );

  if (inspectGit) {
    const changed = runGit(root, ["diff", "HEAD", "--name-only", "--"])
      .split(/\r?\n/)
      .filter(Boolean);
    const untracked = runGit(root, ["ls-files", "--others", "--exclude-standard"])
      .split(/\r?\n/)
      .filter(Boolean);
    const paths = [...new Set([...changed, ...untracked].map(normalizeRepositoryPath))].sort();
    const outside = paths.filter(
      (item) => !SOURCE_ALLOWLIST.has(item) && !HISTORY_PREFIXES.some((prefix) => item.startsWith(prefix)),
    );
    assert(outside.length === 0, `Changed paths outside the Founder package allowlist: ${outside.join(", ")}`);
    const ignoreCheck = spawnSync(
      "git",
      ["check-ignore", "--quiet", "--no-index", ".artifacts/windows-founder-dogfood-r1/review-placeholder.exe"],
      { cwd: root, windowsHide: true },
    );
    assert(ignoreCheck.status === 0, "Founder review artifact path is not ignored by Git.");
  }

  return {
    applicationVersion: packageJson.version,
    developmentIdentifier: base.identifier,
    founderIdentifier: override.identifier,
    schemaVersion: Number(schemaMatch[1]),
  };
}

export async function readWindowsSubsystem(binaryPath) {
  const handle = await open(binaryPath, "r");
  try {
    const dos = Buffer.alloc(64);
    const dosRead = await handle.read(dos, 0, dos.length, 0);
    assert(dosRead.bytesRead === dos.length && dos.toString("ascii", 0, 2) === "MZ", "Windows binary has no valid DOS header.");
    const peOffset = dos.readUInt32LE(0x3c);
    assert(peOffset >= 64 && peOffset <= 16 * 1024 * 1024, "Windows binary PE offset is invalid.");
    const header = Buffer.alloc(96);
    const headerRead = await handle.read(header, 0, header.length, peOffset);
    assert(headerRead.bytesRead === header.length, "Windows binary PE header is truncated.");
    assert(header.toString("ascii", 0, 4) === "PE\0\0", "Windows binary has no valid PE signature.");
    const optionalHeaderSize = header.readUInt16LE(20);
    assert(optionalHeaderSize >= 70, "Windows binary optional header is too small.");
    const magic = header.readUInt16LE(24);
    assert(magic === 0x10b || magic === 0x20b, "Windows binary optional header magic is unsupported.");
    return header.readUInt16LE(24 + 68);
  } finally {
    await handle.close();
  }
}

export async function validateWindowsGuiBinary(binaryPath) {
  const subsystem = await readWindowsSubsystem(binaryPath);
  assert(subsystem === 2, `Windows application subsystem must be GUI (2), found ${subsystem}.`);
  return true;
}

export async function validateFounderBinary(binaryPath) {
  await validateWindowsGuiBinary(binaryPath);
  const binary = await readFile(binaryPath);
  assert(binary.includes(Buffer.from(FOUNDER_IDENTIFIER, "utf8")), "Founder identifier is not embedded in the application binary.");
  assert(binary.includes(Buffer.from(FOUNDER_WINDOW_TITLE, "utf8")), "Founder private window title is not embedded in the application binary.");
  return true;
}

export async function createManifest({ artifactPath, applicationVersion, gitSha, buildTarget }) {
  assert(/^[0-9a-f]{40}$/.test(gitSha), "Git SHA must be 40 lowercase hexadecimal characters.");
  assert(typeof applicationVersion === "string" && applicationVersion.length > 0, "Application version is required.");
  assert(/^[A-Za-z0-9_.-]+$/.test(buildTarget), "Build target contains unsupported characters.");
  const artifact = await stat(artifactPath);
  assert(artifact.isFile(), "Artifact must be a regular file.");
  return {
    applicationVersion,
    gitSha,
    buildTarget,
    artifactFilename: path.basename(artifactPath),
    artifactSize: artifact.size,
    sha256: await sha256File(artifactPath),
  };
}

export async function validateManifest(manifest, { artifactPath, applicationVersion, gitSha, buildTarget }) {
  exactKeys(manifest, MANIFEST_KEYS, "Founder package manifest");
  assert(manifest.applicationVersion === applicationVersion, "Manifest applicationVersion mismatch.");
  assert(manifest.gitSha === gitSha, "Manifest Git SHA mismatch.");
  assert(manifest.buildTarget === buildTarget, "Manifest build target mismatch.");
  assert(/^[0-9a-f]{40}$/.test(manifest.gitSha), "Manifest Git SHA is malformed.");
  assert(/^[A-Za-z0-9_.-]+$/.test(manifest.buildTarget), "Manifest build target is malformed.");
  assert(
    typeof manifest.artifactFilename === "string" &&
      manifest.artifactFilename === path.basename(manifest.artifactFilename) &&
      !manifest.artifactFilename.includes("/") &&
      !manifest.artifactFilename.includes("\\"),
    "Manifest artifact filename must not contain a path.",
  );
  assert(manifest.artifactFilename === path.basename(artifactPath), "Manifest artifact filename mismatch.");
  assert(Number.isSafeInteger(manifest.artifactSize) && manifest.artifactSize > 0, "Manifest artifact size is invalid.");
  assert(/^[0-9a-f]{64}$/.test(manifest.sha256), "Manifest SHA-256 is malformed.");
  const artifact = await stat(artifactPath);
  assert(artifact.isFile(), "Artifact must be a regular file.");
  assert(artifact.size === manifest.artifactSize, "Manifest artifact size mismatch.");
  assert((await sha256File(artifactPath)) === manifest.sha256, "Manifest SHA-256 mismatch.");
  return true;
}

function parseArgs(argv) {
  const [command, ...rest] = argv;
  const values = {};
  for (let index = 0; index < rest.length; index += 2) {
    const key = rest[index];
    assert(key?.startsWith("--") && rest[index + 1] !== undefined, `Malformed argument near ${key ?? "end"}.`);
    values[key.slice(2)] = rest[index + 1];
  }
  return { command, values };
}

async function main(argv) {
  const { command, values } = parseArgs(argv);
  if (command === "verify-source") {
    const result = await validateSourceContract(path.resolve(values.root));
    process.stdout.write(`${JSON.stringify(result)}\n`);
    return;
  }
  if (command === "write-manifest") {
    const manifest = await createManifest({
      artifactPath: path.resolve(values.artifact),
      applicationVersion: values.version,
      gitSha: values["git-sha"],
      buildTarget: values.target,
    });
    await writeFile(path.resolve(values.manifest), `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
    process.stdout.write(`${JSON.stringify(manifest)}\n`);
    return;
  }
  if (command === "verify-windows-binary") {
    await validateWindowsGuiBinary(path.resolve(values.binary));
    process.stdout.write("Windows GUI subsystem verified.\n");
    return;
  }
  if (command === "verify-founder-binary") {
    await validateFounderBinary(path.resolve(values.binary));
    process.stdout.write("Founder embedded identity and Windows GUI subsystem verified.\n");
    return;
  }
  if (command === "verify-manifest") {
    const manifest = await readJson(path.resolve(values.manifest));
    await validateManifest(manifest, {
      artifactPath: path.resolve(values.artifact),
      applicationVersion: values.version,
      gitSha: values["git-sha"],
      buildTarget: values.target,
    });
    process.stdout.write("Founder package manifest verified.\n");
    return;
  }
  throw new Error("Usage: founder-dogfood-package.mjs <verify-source|verify-windows-binary|verify-founder-binary|write-manifest|verify-manifest> [options]");
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 1;
  });
}
