import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { inflateSync } from "node:zlib";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (relativePath) => readFile(path.join(root, relativePath), "utf8");
const readBytes = (relativePath) => readFile(path.join(root, relativePath));

function decodeRgbaPng(bytes) {
  assert.deepEqual(bytes.subarray(0, 8), Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]));
  let offset = 8;
  let width;
  let height;
  const idat = [];
  while (offset < bytes.length) {
    const length = bytes.readUInt32BE(offset);
    const type = bytes.toString("ascii", offset + 4, offset + 8);
    const data = bytes.subarray(offset + 8, offset + 8 + length);
    if (type === "IHDR") {
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      assert.deepEqual(
        { bitDepth: data[8], colorType: data[9], compression: data[10], filter: data[11], interlace: data[12] },
        { bitDepth: 8, colorType: 6, compression: 0, filter: 0, interlace: 0 },
      );
    } else if (type === "IDAT") {
      idat.push(data);
    } else if (type === "IEND") {
      break;
    }
    offset += 12 + length;
  }

  assert.ok(width && height && idat.length, "PNG must contain IHDR and IDAT chunks");
  const packed = inflateSync(Buffer.concat(idat));
  const stride = width * 4;
  assert.equal(packed.length, (stride + 1) * height);
  const pixels = Buffer.alloc(stride * height);
  for (let y = 0; y < height; y += 1) {
    const filter = packed[y * (stride + 1)];
    for (let x = 0; x < stride; x += 1) {
      const raw = packed[y * (stride + 1) + 1 + x];
      const left = x >= 4 ? pixels[y * stride + x - 4] : 0;
      const up = y > 0 ? pixels[(y - 1) * stride + x] : 0;
      const upperLeft = y > 0 && x >= 4 ? pixels[(y - 1) * stride + x - 4] : 0;
      const predictor = left + up - upperLeft;
      const pa = Math.abs(predictor - left);
      const pb = Math.abs(predictor - up);
      const pc = Math.abs(predictor - upperLeft);
      const paeth = pa <= pb && pa <= pc ? left : pb <= pc ? up : upperLeft;
      const reconstructed = filter === 0 ? raw
        : filter === 1 ? raw + left
          : filter === 2 ? raw + up
            : filter === 3 ? raw + Math.floor((left + up) / 2)
              : filter === 4 ? raw + paeth
                : assert.fail(`Unsupported PNG filter ${filter}`);
      pixels[y * stride + x] = reconstructed & 0xff;
    }
  }
  return { width, height, pixels };
}

function alphaBounds(png) {
  let left = png.width;
  let top = png.height;
  let right = -1;
  let bottom = -1;
  for (let y = 0; y < png.height; y += 1) {
    for (let x = 0; x < png.width; x += 1) {
      if (png.pixels[(y * png.width + x) * 4 + 3] > 0) {
        left = Math.min(left, x);
        top = Math.min(top, y);
        right = Math.max(right, x);
        bottom = Math.max(bottom, y);
      }
    }
  }
  return { left, top, right, bottom };
}

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
  "src-tauri/gen/android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml",
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
  "src-tauri/gen/android/app/src/main/res/values/ic_launcher_background.xml",
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

test("tracked and reviewable Android generated surface is an exact M2-A allowlist", () => {
  const actual = execFileSync(
    "git",
    ["ls-files", "--cached", "--others", "--exclude-standard", "--", "src-tauri/gen/android"],
    { cwd: root, encoding: "utf8", windowsHide: true },
  ).trim().split(/\r?\n/).filter(Boolean).sort();

  assert.deepEqual(actual, [...androidSurface].sort());
});

test("Android M2-A has a temporary identity and no Android runtime permission", async () => {
  const config = JSON.parse(await read("src-tauri/tauri.android.conf.json"));
  const gradle = await read("src-tauri/gen/android/app/build.gradle.kts");
  const manifest = await read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");

  assert.equal(config.identifier, "com.lifeos.review.m2a");
  assert.match(gradle, /applicationId = "com\.lifeos\.review\.m2a"/);
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
  const css = await read("src/android-m2a/android-m2a.css");

  assert.match(activity, /enableEdgeToEdge\(\)/);
  assert.match(activity, /findViewById<View>\(android\.R\.id\.content\)/);
  assert.match(activity, /ViewCompat\.setOnApplyWindowInsetsListener\(contentView\)/);
  assert.match(activity, /WindowInsetsCompat\.Type\.systemBars\(\)\s+or\s+WindowInsetsCompat\.Type\.displayCutout\(\)/);
  assert.match(activity, /view\.setPadding\(\s*safeInsets\.left,\s*safeInsets\.top,\s*safeInsets\.right,\s*safeInsets\.bottom\s*\)/);
  assert.match(activity, /ViewCompat\.requestApplyInsets\(contentView\)/);
  assert.doesNotMatch(css, /env\(safe-area-inset-/);
});

test("Android Rust entry exposes only the bounded M2-A façade and no desktop plugin or provider", async () => {
  const lib = await read("src-tauri/src/lib.rs");
  const cargo = await read("src-tauri/Cargo.toml");
  const androidRun = lib.match(/#\[cfg\(target_os = "android"\)\][\s\S]*$/)?.[0] ?? "";

  assert.match(androidRun, /#\[cfg_attr\(mobile, tauri::mobile_entry_point\)\]\s*pub fn run\(\)/);
  assert.match(androidRun, /tauri::Builder::default\(\)/);
  assert.match(androidRun, /android_m2a::m2a_storage_status/);
  assert.match(androidRun, /android_m2a::m2a_create_experience/);
  assert.match(androidRun, /android_m2a::m2a_list_experiences/);
  assert.match(androidRun, /android_m2a::m2a_get_experience/);
  assert.match(androidRun, /android_m2a::m2a_get_locale_preference/);
  assert.match(androidRun, /android_m2a::m2a_set_locale_preference/);
  assert.doesNotMatch(androidRun, /plugin\(|provider|reqwest|desktop_runtime/);
  assert.match(cargo, /\[target\.'cfg\(not\(target_os = "android"\)\)'\.dependencies\]/);
  assert.match(cargo, /\[target\.'cfg\(target_os = "android"\)'\.dependencies\]/);
});

test("Android launcher and adaptive icons are exact derivatives of the canonical desktop icon", async () => {
  const desktopIcon = await readBytes("src-tauri/icons/icon.ico");
  assert.equal(createHash("sha256").update(desktopIcon).digest("hex"), "62d764181ef9137aec875f345345daef4bd398fa828f80cdb35bded563ad2ade");

  const iconCount = desktopIcon.readUInt16LE(4);
  const embedded = Array.from({ length: iconCount }, (_, index) => {
    const entry = 6 + index * 16;
    return {
      width: desktopIcon[entry] || 256,
      height: desktopIcon[entry + 1] || 256,
      size: desktopIcon.readUInt32LE(entry + 8),
      offset: desktopIcon.readUInt32LE(entry + 12),
    };
  }).sort((left, right) => right.width - left.width)[0];
  assert.deepEqual({ width: embedded.width, height: embedded.height }, { width: 256, height: 256 });
  assert.equal(
    createHash("sha256").update(desktopIcon.subarray(embedded.offset, embedded.offset + embedded.size)).digest("hex"),
    "1605b62ebdffb0aec44b2fcdce15ab1098c09ca91807bebbf7c430804b961542",
  );

  const expected = {
    "mipmap-anydpi-v26/ic_launcher.xml": "760d4b8a06bf7163dd010c33ad2cac9e4a75fa0177afaba042f83e311eef0c3e",
    "mipmap-hdpi/ic_launcher_foreground.png": "b093025c0a40a4d0f1ec611d4854ae2236c95290e3ef09f726fc7a688e0586cb",
    "mipmap-hdpi/ic_launcher_round.png": "4486c75a939f78c9953f610bab1914866f9cfb434c26d6cff71e587b81274c7d",
    "mipmap-hdpi/ic_launcher.png": "fa2ead993efd3bfab087cb00a42a4dedd2f629085e3cb2046c2850259cddd412",
    "mipmap-mdpi/ic_launcher_foreground.png": "fc37f2cbe93c8a5a34903229d040a848f90d713f2cbda22162e313f164dfdffa",
    "mipmap-mdpi/ic_launcher_round.png": "347798c9f16d540496b8278ff8f94a414c06ddf32126b893d6ff13f91694a646",
    "mipmap-mdpi/ic_launcher.png": "b56e1e25431e4e29f26c3b2a14cf1f66da88f88fb87696e690f9cb1fde6b37c0",
    "mipmap-xhdpi/ic_launcher_foreground.png": "2f56cd96cc822ad411b495d45c9c46dbea311a02617678b19baee77e1144061a",
    "mipmap-xhdpi/ic_launcher_round.png": "3c00a1dfd20c4ecb152286da21e2533634deb1fe1cf17dac79285e76050dd5db",
    "mipmap-xhdpi/ic_launcher.png": "72612191a425396e469a7fb8696afce15c1ae448de9a44faaca9eeb5f45bb640",
    "mipmap-xxhdpi/ic_launcher_foreground.png": "bd32c9833fe774f91ee9a35b4b1231e47fca6130ae9e2909b2d661bf7b4a4903",
    "mipmap-xxhdpi/ic_launcher_round.png": "d2290ab57c19b36196b2ad40c4cc35f1647a1484cd101ebc100eab534ae80517",
    "mipmap-xxhdpi/ic_launcher.png": "89837e8cc827631823f375cc711bd86e82e44d3ff023406c08b8c5ec5d8ae386",
    "mipmap-xxxhdpi/ic_launcher_foreground.png": "69da3c779376b598b2883ff5aafb9234b11b65d2916a1ec9452d7268752409c7",
    "mipmap-xxxhdpi/ic_launcher_round.png": "24ee297e8870498600d8f985b2b169cd1ea08b80ad5b6291a0bd25aaf349ab35",
    "mipmap-xxxhdpi/ic_launcher.png": "d22ea3be9aa89075cfbfed80fd218aa42ef7078009187c0c04d906f513aa7bb7",
    "values/ic_launcher_background.xml": "f3bc0b26a5c9081dd2527cb59927f9b21855636e7ac77b5a1c8383d4fe26afad",
  };
  for (const [relativePath, digest] of Object.entries(expected)) {
    const bytes = await readBytes(`src-tauri/gen/android/app/src/main/res/${relativePath}`);
    const hashInput = relativePath.endsWith(".xml")
      ? Buffer.from(bytes.toString("utf8").replaceAll("\r\n", "\n"), "utf8")
      : bytes;
    assert.equal(createHash("sha256").update(hashInput).digest("hex"), digest, relativePath);
  }

  const safeForegrounds = [
    ["mdpi", 108, 64, 22],
    ["hdpi", 162, 96, 33],
    ["xhdpi", 216, 128, 44],
    ["xxhdpi", 324, 192, 66],
    ["xxxhdpi", 432, 256, 88],
  ];
  for (const [density, canvas, content, inset] of safeForegrounds) {
    const png = decodeRgbaPng(await readBytes(`src-tauri/gen/android/app/src/main/res/mipmap-${density}/ic_launcher_foreground.png`));
    assert.deepEqual({ width: png.width, height: png.height }, { width: canvas, height: canvas });
    assert.deepEqual(alphaBounds(png), {
      left: inset,
      top: inset,
      right: inset + content - 1,
      bottom: inset + content - 1,
    });
    assert.ok(content / canvas < 0.6, `${density} foreground must remain inside the adaptive safe zone`);
  }

  const canonical = decodeRgbaPng(desktopIcon.subarray(embedded.offset, embedded.offset + embedded.size));
  const xxxhdpi = decodeRgbaPng(await readBytes("src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher_foreground.png"));
  for (let y = 0; y < canonical.height; y += 1) {
    const sourceStart = y * canonical.width * 4;
    const targetStart = ((y + 88) * xxxhdpi.width + 88) * 4;
    assert.deepEqual(
      xxxhdpi.pixels.subarray(targetStart, targetStart + canonical.width * 4),
      canonical.pixels.subarray(sourceStart, sourceStart + canonical.width * 4),
      `xxxhdpi canonical row ${y}`,
    );
  }

  const manifest = await read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");
  const adaptive = await read("src-tauri/gen/android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml");
  const background = await read("src-tauri/gen/android/app/src/main/res/values/ic_launcher_background.xml");
  assert.match(manifest, /android:icon="@mipmap\/ic_launcher"/);
  assert.match(manifest, /android:roundIcon="@mipmap\/ic_launcher_round"/);
  assert.match(adaptive, /@mipmap\/ic_launcher_foreground/);
  assert.match(background, /#14181F/);
});

test("Android frontend entry uses the M2-A-only mode and honest three-operation adapter", async () => {
  const main = await read("src/main.tsx");
  const android = await read("src/android-m2a/AndroidM2AApp.tsx");
  const adapter = await read("src/android-m2a/androidM2AStore.ts");

  assert.match(main, /VITE_LIFE_OS_ANDROID_M2A/);
  assert.match(main, /import\("\.\/android-m2a\/AndroidM2AApp"\)/);
  assert.match(adapter, /"createExperience",\s*"listExperiences",\s*"getExperience"/s);
  assert.doesNotMatch(adapter, /updateExperience|deleteExperience|saveArtifacts|historical/i);
  assert.doesNotMatch(android, /fetch\s*\(|provider|openai|gemini/i);
  assert.match(android, /ANDROID_M2A_LOCALE_PREFERENCE_FILE = "android-m2a-locale\.pref"/);
  assert.match(android, /locale === "zh-TW" \? "zh-Hant" : locale/);
  assert.doesNotMatch(android, /localStorage/i);
  assert.match(adapter, /m2a_get_locale_preference/);
  assert.match(adapter, /m2a_set_locale_preference/);
});

test("direct fresh-v5 path uses canonical final DDL without migration history", async () => {
  const backend = await read("src-tauri/src/android_m2a.rs");
  const direct = await read("src-tauri/src/schema_v5_direct_init.rs");
  const compatibility = await read("src-tauri/schema/schema_v5_compatibility.sql");
  const canonical = await read("src-tauri/schema/schema_v5.sql");

  assert.doesNotMatch(backend, /EMPTY_V4_BASE_SCHEMA|migrate_disposable_v4|verify_any_committed_v5/);
  assert.match(backend, /initialize_direct_fresh_v5/);
  assert.match(backend, /verify_direct_fresh_v5/);
  assert.match(backend, /\.create_new\(true\)/);
  assert.match(backend, /io::copy/);
  assert.match(backend, /destination_file\s*\.sync_all\(\)/);
  assert.match(backend, /#\[cfg\(unix\)\]\s*fn sync_directory[\s\S]*directory\s*\.sync_all\(\)/);
  assert.match(
    backend,
    /m2a_live_publication_directory_sync_failed[\s\S]*mark_retirement_durability_unknown\(&paths\.root\)\?[\s\S]*remove_file\(&paths\.pending_receipt\)[\s\S]*remove_file\(&paths\.pending\)[\s\S]*m2a_pending_retirement_directory_sync_failed[\s\S]*clear_retirement_durability_unknown\(&paths\.root\)\?/,
  );
  assert.match(backend, /m2a_pending_retirement_durability_unknown_preserved/);
  assert.doesNotMatch(backend, /fs::hard_link/);
  assert.match(backend, /\["-wal", "-shm", "-journal"\]/);
  assert.match(backend, /m2a_publication_state_incomplete_preserved/);
  assert.match(backend, /LOCALE_PREFERENCE_FILENAME: &str = "android-m2a-locale\.pref"/);
  assert.match(backend, /matches!\(locale, "en" \| "zh-TW" \| "ja"\)/);
  assert.doesNotMatch(backend, /remove_dir|remove_dir_all/);
  assert.doesNotMatch(backend, /life-os\.db|founderdogfood|com\.lifeos\.app/);
  assert.match(direct, /include_str!\("\.\.\/schema\/schema_v5_compatibility\.sql"\)/);
  assert.match(direct, /split_fixed_ddl\(\)/);
  assert.match(direct, /direct_v5_migration_receipt_forbidden/);
  assert.match(direct, /COUNT\(\*\) FROM schema_migration_receipts/);
  assert.match(direct, /PRAGMA user_version = 5/);
  assert.doesNotMatch(direct, /migrate_disposable_v4|from_version\s*=\s*4/);
  assert.match(compatibility, /CREATE TABLE experience_entries/);
  assert.doesNotMatch(compatibility, /PRAGMA user_version\s*=\s*4/);
  assert.match(canonical, /CREATE TABLE schema_migration_receipts/);
});

test("native review restarts the exact final prepared AVD and proves pending names stay absent", async () => {
  const native = await read("scripts/android-m2a-native-review.ps1");

  assert.match(native, /function Assert-PendingNamesAbsent/);
  assert.match(native, /\.android-m2a-fresh-v5\.pending\.db/);
  assert.match(native, /\.android-m2a-direct-fresh-v5\.pending\.receipt\.json/);
  assert.match(
    native,
    /PASS final Founder review profile is freshly disposable and ready[\s\S]*Assert-PendingNamesAbsent[\s\S]*Stop-DisposableEmulator[\s\S]*Start-DisposableEmulator[\s\S]*Invoke-Cdp 'verify-absent'[\s\S]*Assert-PendingNamesAbsent[\s\S]*PASS final Founder review profile survives exact AVD shutdown\/restart with ready storage and no pending names/,
  );
  assert.doesNotMatch(native, /shell', 'sync'/);
});

test("M0 and Founder-accepted M1 evidence remain preserved", async () => {
  const m0Doc = await read("docs/architecture/20_Android_Build_Feasibility_M0.md");
  const m0Script = await read("scripts/android-m0.ps1");
  const m1Runbook = await read("docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md");
  const archive = await read(".ai/workflow/HISTORY/2026-09-22-android-m1-disposable-persistence-review/ARCHIVE_MANIFEST.json");
  assert.match(m0Doc, /com\.lifeos\.feasibility\.m0/);
  assert.match(m0Script, /com\.lifeos\.feasibility\.m0/);
  assert.match(m1Runbook, /2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975/);
  assert.match(archive, /2026-09-22-android-m1-disposable-persistence-review/);
  assert.equal(
    execFileSync("git", ["rev-parse", "d0e38640a53361c281d65b4befb6208b33f84346^{commit}"], {
      cwd: root,
      encoding: "utf8",
      windowsHide: true,
    }).trim(),
    "d0e38640a53361c281d65b4befb6208b33f84346",
  );
});
