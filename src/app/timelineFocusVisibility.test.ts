import { describe, expect, it } from "vitest";

// @ts-expect-error Vitest runs in Node; frontend production code does not import this built-in.
import { readFileSync } from "node:fs";
import { uiText } from "./i18n";

const styles = readFileSync(new URL("../styles.css", import.meta.url), "utf8");

describe("historical timeline keyboard focus visibility", () => {
  it("uses a clipped-safe inset highlight on the exact Experience summary", () => {
    const rule = styles.match(
      /\.entry-disclosure\s*>\s*summary:focus-visible\s*\{([^}]*)\}/,
    )?.[1];

    expect(rule).toBeDefined();
    expect(rule).toContain("outline: none");
    expect(rule).toMatch(/background:\s*rgba\(/);
    expect(rule).toMatch(/box-shadow:\s*inset\s+0\s+0\s+0\s+3px/);
  });

  it.each(["en", "zh-TW", "ja"] as const)(
    "keeps current, older, and open-detail labels distinct in %s",
    (locale) => {
      const copy = uiText[locale];
      expect(copy.currentReflection.trim()).not.toBe("");
      expect(copy.olderReflection.trim()).not.toBe("");
      expect(copy.entryOpenDetails.trim()).not.toBe("");
      expect(copy.currentReflection).not.toBe(copy.olderReflection);
    },
  );
});
