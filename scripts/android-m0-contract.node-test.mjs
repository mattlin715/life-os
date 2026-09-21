import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (relativePath) => readFile(path.join(root, relativePath), "utf8");

const androidSurface = [
  "src-tauri/gen/android/.editorconfig",
  "src-tauri/gen/android/.gitignore",
  "src-tauri/gen/android/app/.gitignore",
  "src-tauri/gen/android/app/build.gradle.kts",
  "src-tauri/gen/android/app/proguard-rules.pro",
  "src-tauri/gen/android/app/src/main/AndroidManifest.xml",
  "src-tauri/gen/android/app/src/main/java/com/lifeos/feasibility/m0/MainActivity.kt",
  "src-tauri/gen/android/app/src/main/res/drawable-v24/ic_launcher_foreground.xml",
  "src-tauri/gen/android/app/src/main/res/drawable/ic_launcher_background.xml",
  "src-tauri/gen/android/app/src/main/res/layout/activity_main.xml",
  "src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher_foreground.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher_round.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher_foreground.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher_round.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher_foreground.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher_round.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher_foreground.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher_round.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher_foreground.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher_round.png",
  "src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher.png",
  "src-tauri/gen/android/app/src/main/res/values-night/themes.xml",
  "src-tauri/gen/android/app/src/main/res/values/colors.xml",
  "src-tauri/gen/android/app/src/main/res/values/strings.xml",
  "src-tauri/gen/android/app/src/main/res/values/themes.xml",
  "src-tauri/gen/android/app/src/main/res/xml/backup_rules.xml",
  "src-tauri/gen/android/app/src/main/res/xml/data_extraction_rules.xml",
  "src-tauri/gen/android/build.gradle.kts",
  "src-tauri/gen/android/buildSrc/build.gradle.kts",
  "src-tauri/gen/android/buildSrc/src/main/java/com/lifeos/feasibility/m0/kotlin/BuildTask.kt",
  "src-tauri/gen/android/buildSrc/src/main/java/com/lifeos/feasibility/m0/kotlin/RustPlugin.kt",
  "src-tauri/gen/android/gradle.properties",
  "src-tauri/gen/android/gradle/wrapper/gradle-wrapper.jar",
  "src-tauri/gen/android/gradle/wrapper/gradle-wrapper.properties",
  "src-tauri/gen/android/gradlew",
  "src-tauri/gen/android/gradlew.bat",
  "src-tauri/gen/android/settings.gradle",
];

test("tracked and reviewable Android generated surface is an exact M0 allowlist", () => {
  const actual = execFileSync(
    "git",
    ["ls-files", "--cached", "--others", "--exclude-standard", "--", "src-tauri/gen/android"],
    { cwd: root, encoding: "utf8", windowsHide: true },
  ).trim().split(/\r?\n/).filter(Boolean).sort();

  assert.deepEqual(actual, [...androidSurface].sort());
});

test("Android M0 has a temporary identity and no Android runtime permission", async () => {
  const config = JSON.parse(await read("src-tauri/tauri.android.conf.json"));
  const gradle = await read("src-tauri/gen/android/app/build.gradle.kts");
  const manifest = await read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");

  assert.equal(config.identifier, "com.lifeos.feasibility.m0");
  assert.match(gradle, /applicationId = "com\.lifeos\.feasibility\.m0"/);
  assert.doesNotMatch(manifest, /<uses-permission\b/);
  assert.doesNotMatch(manifest, /FileProvider|LEANBACK|usesCleartextTraffic="true"/);
  assert.match(manifest, /android:allowBackup="false"/);
  assert.match(manifest, /android:fullBackupContent="@xml\/backup_rules"/);
  assert.match(manifest, /android:dataExtractionRules="@xml\/data_extraction_rules"/);
});

test("backup and device-transfer rules exclude every supported app data domain", async () => {
  const legacy = await read("src-tauri/gen/android/app/src/main/res/xml/backup_rules.xml");
  const modern = await read("src-tauri/gen/android/app/src/main/res/xml/data_extraction_rules.xml");
  const domains = ["root", "file", "database", "sharedpref", "external", "device_root", "device_file", "device_database", "device_sharedpref"];

  for (const domain of domains) {
    assert.match(legacy, new RegExp(`<exclude domain="${domain}" path="\\." \\/>`));
    assert.equal((modern.match(new RegExp(`<exclude domain="${domain}" path="\\." \\/>`, "g")) ?? []).length, 2);
  }
  assert.match(modern, /<cloud-backup>/);
  assert.match(modern, /<device-transfer>/);
});

test("Android activity applies system-bar and display-cutout insets outside the scrolling WebView", async () => {
  const activity = await read("src-tauri/gen/android/app/src/main/java/com/lifeos/feasibility/m0/MainActivity.kt");
  const css = await read("src/android-m0/android-feasibility-m0.css");

  assert.match(activity, /enableEdgeToEdge\(\)/);
  assert.match(activity, /findViewById<View>\(android\.R\.id\.content\)/);
  assert.match(activity, /ViewCompat\.setOnApplyWindowInsetsListener\(contentView\)/);
  assert.match(activity, /WindowInsetsCompat\.Type\.systemBars\(\)\s+or\s+WindowInsetsCompat\.Type\.displayCutout\(\)/);
  assert.match(activity, /view\.setPadding\(\s*safeInsets\.left,\s*safeInsets\.top,\s*safeInsets\.right,\s*safeInsets\.bottom\s*\)/);
  assert.match(activity, /ViewCompat\.requestApplyInsets\(contentView\)/);
  assert.doesNotMatch(css, /env\(safe-area-inset-/);
});

test("Android Rust entry registers no desktop plugin, command, provider, or SQLite module", async () => {
  const lib = await read("src-tauri/src/lib.rs");
  const cargo = await read("src-tauri/Cargo.toml");
  const androidRun = lib.match(/#\[cfg\(target_os = "android"\)\][\s\S]*$/)?.[0] ?? "";

  assert.match(androidRun, /#\[cfg_attr\(mobile, tauri::mobile_entry_point\)\]\s*pub fn run\(\)/);
  assert.match(androidRun, /tauri::Builder::default\(\)/);
  assert.doesNotMatch(androidRun, /invoke_handler|plugin\(|sqlite|provider|schema_v[45]|reqwest/);
  assert.match(cargo, /\[target\.'cfg\(not\(target_os = "android"\)\)'\.dependencies\]/);
});

test("Android frontend entry is isolated from desktop storage and providers", async () => {
  const main = await read("src/main.tsx");
  const android = await read("src/android-m0/AndroidFeasibilityApp.tsx");

  assert.match(main, /VITE_LIFE_OS_ANDROID_FEASIBILITY_M0/);
  assert.match(main, /import\("\.\/android-m0\/AndroidFeasibilityApp"\)/);
  assert.doesNotMatch(android, /from\s+["']@tauri-apps|import[^\n]*LocalEvidenceStore|fetch\s*\(|invoke\s*\(/i);
});
