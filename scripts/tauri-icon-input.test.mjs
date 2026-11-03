import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { crc32, inflateSync } from "node:zlib";
import { describe, expect, it } from "vitest";

const root = fileURLToPath(new URL("../", import.meta.url));
const pngPath = "src-tauri/icons/icon.png";
const icoPath = "src-tauri/icons/icon.ico";
const signature = Buffer.from("89504e470d0a1a0a", "hex");
const git = (...args) => execFileSync("git", args, { cwd: root });

// Inspect Git objects, not merely a possibly ignored developer-local file.
const entries = git("ls-files", "--stage", "-z").toString().split("\0");
function versionedAsset(path, inventory = entries) {
  const entry = inventory.find((line) => line.split("\t")[1] === path);
  if (!entry) throw new Error(`Missing exact-case versioned asset: ${path}`);
  const [mode, oid, stage] = entry.split("\t")[0].split(" ");
  if (mode !== "100644" || stage !== "0") throw new Error(`Not a regular resolved asset: ${path}`);
  const blob = git("cat-file", "blob", oid);
  if (!readFileSync(new URL(`../${path}`, import.meta.url)).equals(blob)) {
    throw new Error(`Working file differs from versioned asset: ${path}`);
  }
  return blob;
}

function validatePng(png) {
  if (!png.subarray(0, 8).equals(signature)) throw new Error("Invalid PNG signature");
  let offset = 8;
  const chunks = [];
  while (offset < png.length) {
    if (offset + 12 > png.length) throw new Error("Truncated PNG chunk");
    const length = png.readUInt32BE(offset);
    const end = offset + 12 + length;
    if (end > png.length) throw new Error("Truncated PNG data");
    const type = png.toString("ascii", offset + 4, offset + 8);
    const data = png.subarray(offset + 8, end - 4);
    if (crc32(png.subarray(offset + 4, end - 4)) !== png.readUInt32BE(end - 4)) {
      throw new Error("Invalid PNG CRC");
    }
    chunks.push({ type, data });
    offset = end;
  }
  const header = chunks[0];
  if (header?.type !== "IHDR" || header.data.length !== 13) throw new Error("Invalid PNG IHDR");
  const width = header.data.readUInt32BE(0);
  const height = header.data.readUInt32BE(4);
  // Tauri 2.6.3 CachedIcon::new_png requires RGBA; this source is 8-bit, non-interlaced.
  if (width !== 256 || height !== 256 || !header.data.subarray(8).equals(Buffer.from([8, 6, 0, 0, 0]))) {
    throw new Error("Expected 256x256 8-bit RGBA PNG");
  }
  const last = chunks.at(-1);
  if (last?.type !== "IEND" || last.data.length !== 0) throw new Error("Missing PNG IEND");
  const pixels = inflateSync(Buffer.concat(chunks.filter((chunk) => chunk.type === "IDAT").map((chunk) => chunk.data)));
  const stride = 1 + width * 4;
  if (pixels.length !== stride * height) throw new Error("Invalid PNG pixel data length");
  for (let row = 0; row < height; row++) {
    if (pixels[row * stride] > 4) throw new Error("Invalid PNG row filter");
  }
}

function embeddedPng(ico) {
  if (ico.readUInt16LE(0) !== 0 || ico.readUInt16LE(2) !== 1) throw new Error("Invalid ICO");
  for (let index = 0; index < ico.readUInt16LE(4); index++) {
    const entry = 6 + index * 16;
    if (ico[entry] !== 0 || ico[entry + 1] !== 0) continue; // ICO encodes 256 as zero.
    const offset = ico.readUInt32LE(entry + 12);
    const png = ico.subarray(offset, offset + ico.readUInt32LE(entry + 8));
    validatePng(png);
    return png;
  }
  throw new Error("Missing 256x256 embedded PNG");
}

describe("Tauri clean-checkout icon inputs", () => {
  it("provides the exact-case, versioned Linux default with the existing ICO design", () => {
    const config = JSON.parse(git("show", ":src-tauri/tauri.conf.json"));
    // Text checkout EOL conversion is valid; asset bytes must match exactly.
    expect(JSON.parse(readFileSync(new URL("../src-tauri/tauri.conf.json", import.meta.url)))).toEqual(config);
    const selected = config.bundle.icon?.find((path) => path.endsWith(".png")) ?? "icons/icon.png";
    expect(`src-tauri/${selected}`).toBe(pngPath);
    const png = versionedAsset(pngPath);
    validatePng(png);
    expect(png.equals(embeddedPng(versionedAsset(icoPath)))).toBe(true);
  });

  it("rejects an absent or developer-local-only PNG even if the working file exists", () => {
    expect(() => versionedAsset(pngPath, [])).toThrow("Missing exact-case versioned asset");
  });

  it("rejects wrong Git path casing even on a case-insensitive host", () => {
    const wrongCase = entries.map((entry) => entry.replace(`\t${pngPath}`, "\tsrc-tauri/icons/Icon.png"));
    expect(() => versionedAsset(pngPath, wrongCase)).toThrow("Missing exact-case versioned asset");
  });

  it("rejects an invalid format or LFS pointer instead of a PNG blob", () => {
    expect(() => validatePng(Buffer.from("version https://git-lfs.github.com/spec/v1\n"))).toThrow("Invalid PNG signature");
  });

  it("rejects truncation and corrupt image data", () => {
    const png = embeddedPng(versionedAsset(icoPath));
    expect(() => validatePng(png.subarray(0, -1))).toThrow("Truncated PNG");
    const corrupt = Buffer.from(png);
    corrupt[corrupt.length - 16] ^= 1;
    expect(() => validatePng(corrupt)).toThrow("Invalid PNG CRC");
  });

  it("rejects a CRC-valid non-RGBA PNG", () => {
    const rgb = Buffer.from(embeddedPng(versionedAsset(icoPath)));
    rgb[25] = 2;
    rgb.writeUInt32BE(crc32(rgb.subarray(12, 29)), 29);
    expect(() => validatePng(rgb)).toThrow("Expected 256x256 8-bit RGBA PNG");
  });
});
