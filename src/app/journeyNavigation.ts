import type { DailyReflectionStage } from "./dailyReflectionJourney";

interface JourneyNavigationTarget {
  focus(options?: FocusOptions): void;
  scrollIntoView(options?: ScrollIntoViewOptions): void;
}

interface JourneyNavigationEnvironment {
  getElementById(id: string): JourneyNavigationTarget | null;
  requestFrame(callback: () => void): void;
  prefersReducedMotion(): boolean;
}

const browserEnvironment: JourneyNavigationEnvironment = {
  getElementById: (id) => document.getElementById(id),
  requestFrame: (callback) => {
    if (typeof requestAnimationFrame === "function") requestAnimationFrame(callback);
    else callback();
  },
  prefersReducedMotion: () =>
    typeof matchMedia === "function" &&
    matchMedia("(prefers-reduced-motion: reduce)").matches,
};

export function journeyStageId(entryId: string, stage: DailyReflectionStage): string {
  return `journey-${stage}-${entryId}`;
}

export function historicalContextPanelId(entryId: string): string {
  return `historical-context-${entryId}`;
}

export function focusJourneyStage(
  entryId: string,
  stage: DailyReflectionStage,
  environment: JourneyNavigationEnvironment = browserEnvironment,
): void {
  environment.requestFrame(() => {
    const target = environment.getElementById(journeyStageId(entryId, stage));
    if (!target) return;
    target.focus({ preventScroll: true });
    target.scrollIntoView({
      behavior: environment.prefersReducedMotion() ? "auto" : "smooth",
      block: "start",
    });
  });
}

export function focusJourneyTarget(
  id: string,
  environment: JourneyNavigationEnvironment = browserEnvironment,
): void {
  environment.requestFrame(() => {
    const target = environment.getElementById(id);
    if (!target) return;
    target.focus({ preventScroll: true });
    target.scrollIntoView({
      behavior: environment.prefersReducedMotion() ? "auto" : "smooth",
      block: "center",
    });
  });
}

export function focusHistoricalContextPanel(
  entryId: string,
  environment: JourneyNavigationEnvironment = browserEnvironment,
): void {
  environment.requestFrame(() => {
    const target = environment.getElementById(historicalContextPanelId(entryId));
    if (!target) return;
    target.focus({ preventScroll: true });
    target.scrollIntoView({
      behavior: environment.prefersReducedMotion() ? "auto" : "smooth",
      block: "start",
    });
  });
}

export function focusDailyReflectionComposer(
  environment: JourneyNavigationEnvironment = browserEnvironment,
): void {
  environment.requestFrame(() => {
    const target = environment.getElementById("daily-reflection-composer");
    if (!target) return;
    target.focus({ preventScroll: true });
    target.scrollIntoView({
      behavior: environment.prefersReducedMotion() ? "auto" : "smooth",
      block: "center",
    });
  });
}
