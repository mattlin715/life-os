import { describe, expect, it, vi } from "vitest";
import { focusDailyReflectionComposer, focusJourneyStage, focusJourneyTarget } from "./journeyNavigation";

function environment(reducedMotion = false) {
  const focus = vi.fn();
  const scrollIntoView = vi.fn();
  return {
    focus,
    scrollIntoView,
    value: {
      getElementById: vi.fn(() => ({ focus, scrollIntoView })),
      requestFrame: (callback: () => void) => callback(),
      prefersReducedMotion: () => reducedMotion,
    },
  };
}

describe("journey navigation", () => {
  it("focuses the explicit stage without executing a product action", () => {
    const target = environment();
    focusJourneyStage("entry", "reflection", target.value);
    expect(target.value.getElementById).toHaveBeenCalledWith("journey-reflection-entry");
    expect(target.focus).toHaveBeenCalledWith({ preventScroll: true });
    expect(target.scrollIntoView).toHaveBeenCalledWith({ behavior: "smooth", block: "start" });
  });

  it("uses non-animated scrolling when reduced motion is preferred", () => {
    const target = environment(true);
    focusJourneyStage("entry", "completion", target.value);
    expect(target.scrollIntoView).toHaveBeenCalledWith({ behavior: "auto", block: "start" });
  });

  it("focuses the composer as an explicit reversible navigation choice", () => {
    const target = environment();
    focusDailyReflectionComposer(target.value);
    expect(target.value.getElementById).toHaveBeenCalledWith("daily-reflection-composer");
  });

  it("supports event-free composer callbacks used by completion actions", () => {
    const target = environment();
    const onRecordAnother = () => focusDailyReflectionComposer(target.value);

    expect(() => onRecordAnother()).not.toThrow();
    expect(target.value.getElementById).toHaveBeenCalledWith("daily-reflection-composer");
    expect(target.focus).toHaveBeenCalledWith({ preventScroll: true });
  });

  it("focuses an exact generated review item without invoking it", () => {
    const target = environment();
    focusJourneyTarget("reflection-prompt-1", target.value);
    expect(target.value.getElementById).toHaveBeenCalledWith("reflection-prompt-1");
    expect(target.focus).toHaveBeenCalledOnce();
  });

  it("fails safely when a stage is no longer present", () => {
    expect(() => focusJourneyStage("missing", "evidence", {
      getElementById: () => null,
      requestFrame: (callback) => callback(),
      prefersReducedMotion: () => false,
    })).not.toThrow();
  });
});
