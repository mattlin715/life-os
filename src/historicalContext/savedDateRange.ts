import { isValidTimestamp } from "./date";

const calendarDatePattern = /^(\d{4})-(\d{2})-(\d{2})$/;

export type HistoricalSavedDateRangeError =
  | "missing_dates"
  | "invalid_date"
  | "inverted_range"
  | "timezone_unavailable";

export interface HistoricalSavedDateRange {
  readonly startDate: string;
  readonly endDate: string;
  readonly timeZone: string;
  readonly startInclusiveMs: number;
  readonly endExclusiveMs: number;
}

export interface HistoricalSavedDateRangeEnvironment {
  readonly timeZone: string | null;
  readonly localStartOfDay: (date: CalendarDate) => number | null;
}

export interface CalendarDate {
  readonly year: number;
  readonly month: number;
  readonly day: number;
}

export type HistoricalSavedDateRangeResult =
  | { readonly status: "valid"; readonly range: HistoricalSavedDateRange }
  | { readonly status: "invalid"; readonly reason: HistoricalSavedDateRangeError };

export interface HistoricalSavedDateRangeControl {
  readonly enabled: boolean;
  readonly startDate: string;
  readonly endDate: string;
  readonly appliedRange: HistoricalSavedDateRange | null;
  readonly error: HistoricalSavedDateRangeError | null;
}

export type HistoricalSavedDateRangeControls = ReadonlyMap<
  string,
  HistoricalSavedDateRangeControl
>;

export type HistoricalSavedDateRangeConstraint =
  | { readonly status: "inactive" }
  | {
      readonly status: "blocked";
      readonly reason: HistoricalSavedDateRangeError | "not_applied";
    }
  | { readonly status: "applied"; readonly range: HistoricalSavedDateRange };

function isLeapYear(year: number): boolean {
  return year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
}

function daysInMonth(year: number, month: number): number {
  const days = [
    31,
    isLeapYear(year) ? 29 : 28,
    31,
    30,
    31,
    30,
    31,
    31,
    30,
    31,
    30,
    31,
  ];
  return days[month - 1] ?? 0;
}

export function parseCalendarDate(value: string): CalendarDate | null {
  const match = calendarDatePattern.exec(value);
  if (!match) return null;

  const year = Number(match[1]);
  const month = Number(match[2]);
  const day = Number(match[3]);
  if (
    year < 1 ||
    year > 9999 ||
    month < 1 ||
    month > 12 ||
    day < 1 ||
    day > daysInMonth(year, month)
  ) {
    return null;
  }

  return { year, month, day };
}

function nextCalendarDate(date: CalendarDate): CalendarDate | null {
  const utc = new Date(0);
  utc.setUTCFullYear(date.year, date.month - 1, date.day + 1);
  utc.setUTCHours(0, 0, 0, 0);
  const year = utc.getUTCFullYear();
  if (year < 1 || year > 10000) return null;
  return {
    year,
    month: utc.getUTCMonth() + 1,
    day: utc.getUTCDate(),
  };
}

function deviceLocalStartOfDay(date: CalendarDate): number | null {
  const local = new Date(0);
  local.setFullYear(date.year, date.month - 1, date.day);
  local.setHours(0, 0, 0, 0);
  if (
    local.getFullYear() !== date.year ||
    local.getMonth() !== date.month - 1 ||
    local.getDate() !== date.day
  ) {
    return null;
  }
  const epochMs = local.getTime();
  return Number.isFinite(epochMs) ? epochMs : null;
}

export function deviceSavedDateRangeEnvironment(): HistoricalSavedDateRangeEnvironment {
  let timeZone: string | null = null;
  try {
    timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone?.trim() || null;
  } catch {
    timeZone = null;
  }

  return {
    timeZone,
    localStartOfDay: deviceLocalStartOfDay,
  };
}

export function buildHistoricalSavedDateRange(
  startDate: string,
  endDate: string,
  environment: HistoricalSavedDateRangeEnvironment = deviceSavedDateRangeEnvironment(),
): HistoricalSavedDateRangeResult {
  if (!startDate || !endDate) {
    return { status: "invalid", reason: "missing_dates" };
  }

  const start = parseCalendarDate(startDate);
  const end = parseCalendarDate(endDate);
  if (!start || !end) {
    return { status: "invalid", reason: "invalid_date" };
  }
  if (startDate > endDate) {
    return { status: "invalid", reason: "inverted_range" };
  }
  if (!environment.timeZone) {
    return { status: "invalid", reason: "timezone_unavailable" };
  }

  const dayAfterEnd = nextCalendarDate(end);
  if (!dayAfterEnd) {
    return { status: "invalid", reason: "invalid_date" };
  }
  const startInclusiveMs = environment.localStartOfDay(start);
  const endExclusiveMs = environment.localStartOfDay(dayAfterEnd);
  if (
    startInclusiveMs === null ||
    endExclusiveMs === null ||
    !Number.isFinite(startInclusiveMs) ||
    !Number.isFinite(endExclusiveMs) ||
    endExclusiveMs <= startInclusiveMs
  ) {
    return { status: "invalid", reason: "timezone_unavailable" };
  }

  return {
    status: "valid",
    range: {
      startDate,
      endDate,
      timeZone: environment.timeZone,
      startInclusiveMs,
      endExclusiveMs,
    },
  };
}

export function isTimestampInHistoricalSavedDateRange(
  timestamp: string,
  range: HistoricalSavedDateRange,
): boolean {
  if (!isValidTimestamp(timestamp)) return false;
  const instant = Date.parse(timestamp);
  return (
    Number.isFinite(instant) &&
    instant >= range.startInclusiveMs &&
    instant < range.endExclusiveMs
  );
}

export function createHistoricalSavedDateRangeControl(): HistoricalSavedDateRangeControl {
  return {
    enabled: false,
    startDate: "",
    endDate: "",
    appliedRange: null,
    error: null,
  };
}

function controlFor(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
): HistoricalSavedDateRangeControl {
  return controls.get(experienceId) ?? createHistoricalSavedDateRangeControl();
}

function setControl(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
  control: HistoricalSavedDateRangeControl,
): HistoricalSavedDateRangeControls {
  const next = new Map(controls);
  next.set(experienceId, control);
  return next;
}

export function getHistoricalSavedDateRangeControl(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
): HistoricalSavedDateRangeControl {
  return controlFor(controls, experienceId);
}

export function setHistoricalSavedDateRangeEnabled(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
  enabled: boolean,
): HistoricalSavedDateRangeControls {
  const current = controlFor(controls, experienceId);
  return setControl(controls, experienceId, {
    ...current,
    enabled,
    appliedRange: null,
    error: null,
  });
}

export function setHistoricalSavedDateRangeStart(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
  startDate: string,
): HistoricalSavedDateRangeControls {
  const current = controlFor(controls, experienceId);
  return setControl(controls, experienceId, {
    ...current,
    startDate,
    appliedRange: null,
    error: null,
  });
}

export function setHistoricalSavedDateRangeEnd(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
  endDate: string,
): HistoricalSavedDateRangeControls {
  const current = controlFor(controls, experienceId);
  return setControl(controls, experienceId, {
    ...current,
    endDate,
    appliedRange: null,
    error: null,
  });
}

export function applyHistoricalSavedDateRange(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
  environment: HistoricalSavedDateRangeEnvironment = deviceSavedDateRangeEnvironment(),
): HistoricalSavedDateRangeControls {
  const current = controlFor(controls, experienceId);
  if (!current.enabled) return controls;

  const result = buildHistoricalSavedDateRange(
    current.startDate,
    current.endDate,
    environment,
  );
  return setControl(
    controls,
    experienceId,
    result.status === "valid"
      ? { ...current, appliedRange: result.range, error: null }
      : { ...current, appliedRange: null, error: result.reason },
  );
}

export function historicalSavedDateRangeConstraint(
  control: HistoricalSavedDateRangeControl | undefined,
): HistoricalSavedDateRangeConstraint {
  if (!control?.enabled) return { status: "inactive" };
  if (control.appliedRange) {
    return { status: "applied", range: control.appliedRange };
  }
  return { status: "blocked", reason: control.error ?? "not_applied" };
}

export function removeHistoricalSavedDateRangeControl(
  controls: HistoricalSavedDateRangeControls,
  experienceId: string,
): HistoricalSavedDateRangeControls {
  if (!controls.has(experienceId)) return controls;
  const next = new Map(controls);
  next.delete(experienceId);
  return next;
}

export function shouldCloseHistoricalPreflightForSavedDateRangeChange(
  preflightCurrentExperienceId: string | null | undefined,
  changedCurrentExperienceId: string,
): boolean {
  return preflightCurrentExperienceId === changedCurrentExperienceId;
}
