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

test("tracked and reviewable Android generated surface is an exact M1 allowlist", () => {
  const actual = execFileSync(
    "git",
    ["ls-files", "--cached", "--others", "--exclude-standard", "--", "src-tauri/gen/android"],
    { cwd: root, encoding: "utf8", windowsHide: true },
  ).trim().split(/\r?\n/).filter(Boolean).sort();

  assert.deepEqual(actual, [...androidSurface].sort());
});

test("Android M1 has a temporary identity and no Android runtime permission", async () => {
  const config = JSON.parse(await read("src-tauri/tauri.android.conf.json"));
  const gradle = await read("src-tauri/gen/android/app/build.gradle.kts");
  const manifest = await read("src-tauri/gen/android/app/src/main/AndroidManifest.xml");

  assert.equal(config.identifier, "com.lifeos.review.m1");
  assert.match(gradle, /applicationId = "com\.lifeos\.review\.m1"/);
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
  const css = await read("src/android-m1/android-m1.css");

  assert.match(activity, /enableEdgeToEdge\(\)/);
  assert.match(activity, /findViewById<View>\(android\.R\.id\.content\)/);
  assert.match(activity, /ViewCompat\.setOnApplyWindowInsetsListener\(contentView\)/);
  assert.match(activity, /WindowInsetsCompat\.Type\.systemBars\(\)\s+or\s+WindowInsetsCompat\.Type\.displayCutout\(\)/);
  assert.match(activity, /view\.setPadding\(\s*safeInsets\.left,\s*safeInsets\.top,\s*safeInsets\.right,\s*safeInsets\.bottom\s*\)/);
  assert.match(activity, /ViewCompat\.requestApplyInsets\(contentView\)/);
  assert.doesNotMatch(css, /env\(safe-area-inset-/);
});

test("Android Rust entry exposes only the bounded M1 façade and no desktop plugin or provider", async () => {
  const lib = await read("src-tauri/src/lib.rs");
  const cargo = await read("src-tauri/Cargo.toml");
  const androidRun = lib.match(/#\[cfg\(target_os = "android"\)\][\s\S]*$/)?.[0] ?? "";

  assert.match(androidRun, /#\[cfg_attr\(mobile, tauri::mobile_entry_point\)\]\s*pub fn run\(\)/);
  assert.match(androidRun, /tauri::Builder::default\(\)/);
  assert.match(androidRun, /android_m1::m1_storage_status/);
  assert.match(androidRun, /android_m1::m1_create_experience/);
  assert.match(androidRun, /android_m1::m1_list_experiences/);
  assert.match(androidRun, /android_m1::m1_get_experience/);
  assert.match(androidRun, /android_m1::m1_get_locale_preference/);
  assert.match(androidRun, /android_m1::m1_set_locale_preference/);
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
    assert.equal(createHash("sha256").update(bytes).digest("hex"), digest, relativePath);
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

test("Android frontend entry uses the M1-only mode and honest three-operation adapter", async () => {
  const main = await read("src/main.tsx");
  const android = await read("src/android-m1/AndroidM1App.tsx");
  const adapter = await read("src/android-m1/androidM1Store.ts");

  assert.match(main, /VITE_LIFE_OS_ANDROID_M1/);
  assert.match(main, /import\("\.\/android-m1\/AndroidM1App"\)/);
  assert.match(adapter, /"createExperience",\s*"listExperiences",\s*"getExperience"/s);
  assert.doesNotMatch(adapter, /updateExperience|deleteExperience|saveArtifacts|historical/i);
  assert.doesNotMatch(android, /fetch\s*\(|provider|openai|gemini/i);
  assert.match(android, /ANDROID_M1_LOCALE_PREFERENCE_FILE = "android-m1-locale\.pref"/);
  assert.match(android, /locale === "zh-TW" \? "zh-Hant" : locale/);
  assert.doesNotMatch(android, /localStorage/i);
  assert.match(adapter, /m1_get_locale_preference/);
  assert.match(adapter, /m1_set_locale_preference/);
});

test("fresh-v5 Android path reuses the canonical base and fails closed without delete or desktop paths", async () => {
  const backend = await read("src-tauri/src/android_m1.rs");
  const sharedBase = await read("src-tauri/src/schema_v5_fresh_base.rs");
  const founder = await read("src-tauri/src/schema_v5_founder_activation.rs");

  assert.match(backend, /EMPTY_V4_BASE_SCHEMA/);
  assert.match(backend, /migrate_disposable_v4/);
  assert.match(backend, /verify_any_committed_v5/);
  assert.match(backend, /m1_pending_initialization_preserved/);
  assert.match(backend, /LOCALE_PREFERENCE_FILENAME: &str = "android-m1-locale\.pref"/);
  assert.match(backend, /matches!\(locale, "en" \| "zh-TW" \| "ja"\)/);
  assert.doesNotMatch(backend, /fs::remove_(file|dir)/);
  assert.doesNotMatch(backend, /life-os\.db|founderdogfood|com\.lifeos\.app/);
  assert.match(sharedBase, /pub\(crate\) const EMPTY_V4_BASE_SCHEMA/);
  assert.match(founder, /schema_v5_fresh_base::EMPTY_V4_BASE_SCHEMA/);
});

test("M0 package and accepted evidence remain separate and untouched", async () => {
  const m0Doc = await read("docs/architecture/20_Android_Build_Feasibility_M0.md");
  const m0Script = await read("scripts/android-m0.ps1");
  assert.match(m0Doc, /com\.lifeos\.feasibility\.m0/);
  assert.match(m0Script, /com\.lifeos\.feasibility\.m0/);
});
