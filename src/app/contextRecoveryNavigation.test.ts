import { describe, expect, it, vi } from "vitest";

import { focusContextRecovery } from "./contextRecoveryNavigation";

describe("focusContextRecovery", () => {
  it("focuses and reveals the recovery prompt after React renders it", () => {
    const focus = vi.fn();
    const scrollIntoView = vi.fn();
    const recovery = { focus, scrollIntoView };
    const requestFrame = vi.fn((callback: () => void) => callback());
    const getElementById = vi.fn(() => recovery);

    focusContextRecovery("entry-1", { getElementById, requestFrame });

    expect(requestFrame).toHaveBeenCalledOnce();
    expect(getElementById).toHaveBeenCalledWith("context-recovery-entry-1");
    expect(focus).toHaveBeenCalledWith({ preventScroll: true });
    expect(scrollIntoView).toHaveBeenCalledWith({
      behavior: "smooth",
      block: "center",
    });
  });

  it("fails safely when no recovery prompt is present", () => {
    const requestFrame = (callback: () => void) => callback();

    expect(() => focusContextRecovery("missing-entry", {
      getElementById: () => null,
      requestFrame,
    })).not.toThrow();
  });

  it("avoids smooth scrolling when reduced motion is preferred", () => {
    const focus = vi.fn();
    const scrollIntoView = vi.fn();
    focusContextRecovery("entry-1", {
      getElementById: () => ({ focus, scrollIntoView }),
      requestFrame: (callback) => callback(),
      prefersReducedMotion: () => true,
    });
    expect(scrollIntoView).toHaveBeenCalledWith({ behavior: "auto", block: "center" });
  });
});
