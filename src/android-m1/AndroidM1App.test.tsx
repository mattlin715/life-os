import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import {
  ANDROID_M1_LOCALE_PREFERENCE_FILE,
  AndroidM1App,
  androidM1Copy,
  createM1RequestId,
  normalizeAndroidM1Locale,
} from "./AndroidM1App";
import { androidM1SupportedOperations } from "./androidM1Store";

describe("Android M1 disposable persistence review surface", () => {
  it("renders a truthful loading boundary before storage readiness", () => {
    const html = renderToStaticMarkup(<AndroidM1App />);
    expect(html).toContain('data-runtime="android-m1-disposable"');
    expect(html).toContain('data-storage-state="loading"');
    expect(html).not.toContain("Committed —");
  });

  it.each(["en", "zh-TW", "ja"] as const)("keeps scope and durability limits explicit in %s", (locale) => {
    const copy = androidM1Copy[locale];
    expect(copy.boundaries).toHaveLength(4);
    expect(copy.boundaries.join(" ")).toContain("com.lifeos.review.m1");
    expect(copy.supported).toContain("createExperience");
    expect(copy.unsupported.length).toBeGreaterThan(20);
  });

  it("exposes only the honest LocalEvidenceStore subset", () => {
    expect(androidM1SupportedOperations).toEqual(["createExperience", "listExperiences", "getExperience"]);
  });

  it("creates stable, backend-valid idempotency identifiers", () => {
    expect(createM1RequestId()).toMatch(/^m1_[A-Za-z0-9_-]{5,}$/);
  });

  it.each(["en", "zh-TW", "ja"] as const)("accepts the bounded non-content locale preference %s", (locale) => {
    expect(normalizeAndroidM1Locale(locale)).toBe(locale);
    expect(ANDROID_M1_LOCALE_PREFERENCE_FILE).toBe("android-m1-locale.pref");
  });

  it("fails safely to English when the locale preference is absent or invalid", () => {
    expect(normalizeAndroidM1Locale(null)).toBe("en");
    expect(normalizeAndroidM1Locale("fr")).toBe("en");
  });

  it("aligns Traditional Chinese and Japanese product terms without widening M1", () => {
    expect(androidM1Copy["zh-TW"].title).toContain("片刻");
    expect(androidM1Copy["zh-TW"].save).toContain("儲存");
    expect(androidM1Copy.ja.title).toContain("瞬間");
    expect(androidM1Copy.ja.open).toContain("原文");
    for (const locale of ["zh-TW", "ja"] as const) {
      expect(androidM1Copy[locale].synthetic).toMatch(/合成/);
      expect(androidM1Copy[locale].boundaries.join(" ")).toMatch(/AI/);
    }
  });
});
