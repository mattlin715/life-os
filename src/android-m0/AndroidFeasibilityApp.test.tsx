import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { AndroidFeasibilityApp, androidM0Copy } from "./AndroidFeasibilityApp";

describe("Android feasibility M0 disclosure", () => {
  it("renders a session-only synthetic mirror with no product action", () => {
    const html = renderToStaticMarkup(<AndroidFeasibilityApp />);

    expect(html).toContain("Life OS Android feasibility shell");
    expect(html).toContain("No real AI");
    expect(html).toContain("No product database");
    expect(html).toContain("No desktop profile");
    expect(html).toContain("session-only");
    expect(html).toContain("com.lifeos.feasibility.m0");
    expect(html).toContain('data-runtime="android-feasibility-m0"');
  });

  it.each(["en", "zh-TW", "ja"] as const)("keeps all five boundaries visible in %s", (locale) => {
    const copy = androidM0Copy[locale];

    expect(copy.boundaries).toHaveLength(5);
    expect(copy.summary.length).toBeGreaterThan(10);
    expect(copy.identity).toContain("com.lifeos.feasibility.m0");
    expect(copy.footer.length).toBeGreaterThan(10);
  });
});
