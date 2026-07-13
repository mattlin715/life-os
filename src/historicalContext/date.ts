import { uiText, type AppLanguage } from "../app/i18n";

const dateLocales: Record<AppLanguage, string> = {
  en: "en-US",
  "zh-TW": "zh-TW",
  ja: "ja-JP",
};

export interface TimestampFormatOptions {
  /**
   * Omit this in production to use the device/user timezone. Tests may inject
   * a timezone so they remain deterministic across hosts.
   */
  readonly timeZone?: string;
}

/**
 * The persisted/imported timestamp contract is deliberately narrower than the
 * browser's permissive `Date` parser: a complete RFC3339/ISO datetime with an
 * explicit UTC designator or numeric offset. Presentation consumes the parsed
 * instant, while HTML receives a separately normalized serialization; neither
 * path rewrites the stored source data.
 */
const strictTimestampPattern =
  /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(Z|[+-]\d{2}:\d{2})$/;

/**
 * `Date#toISOString` can emit year zero or a signed extended year after an
 * otherwise governed source instant crosses a UTC year boundary. HTML global
 * datetime values do not safely represent either form, so validate the
 * normalized serialization independently from source parsing.
 */
const htmlGlobalDateTimePattern =
  /^[1-9]\d{3}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])T(?:[01]\d|2[0-3]):[0-5]\d:[0-5]\d\.\d{3}Z$/;

function isLeapYear(year: number): boolean {
  return year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
}

function daysInMonth(year: number, month: number): number {
  const days = [31, isLeapYear(year) ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  return days[month - 1] ?? 0;
}

/**
 * Parses only supported persisted timestamps. It rejects raw whitespace,
 * date-only and host-locale values, and impossible calendar/time/offset
 * components (including year zero) before asking the runtime to construct the
 * instant. Fractional precision beyond milliseconds remains supported source
 * data; JavaScript normalizes it only for the separate HTML serialization.
 */
function parseTimestamp(isoTimestamp: string): Date | null {
  if (typeof isoTimestamp !== "string") return null;

  const match = strictTimestampPattern.exec(isoTimestamp);
  if (!match) return null;

  const [, yearText, monthText, dayText, hourText, minuteText, secondText, offsetText] = match;
  const year = Number(yearText);
  const month = Number(monthText);
  const day = Number(dayText);
  const hour = Number(hourText);
  const minute = Number(minuteText);
  const second = Number(secondText);
  const offsetHour = offsetText === "Z" ? 0 : Number(offsetText.slice(1, 3));
  const offsetMinute = offsetText === "Z" ? 0 : Number(offsetText.slice(4, 6));

  if (
    year < 1 ||
    month < 1 ||
    month > 12 ||
    day < 1 ||
    day > daysInMonth(year, month) ||
    hour > 23 ||
    minute > 59 ||
    second > 59 ||
    offsetHour > 23 ||
    offsetMinute > 59
  ) {
    return null;
  }

  const parsedTimestamp = new Date(isoTimestamp);
  return Number.isNaN(parsedTimestamp.getTime()) ? null : parsedTimestamp;
}

/** Returns whether a stored source value represents a valid timestamp. */
export function isValidTimestamp(isoTimestamp: string): boolean {
  return parseTimestamp(isoTimestamp) !== null;
}

/**
 * Returns a normalized, machine-readable HTML datetime value for a governed
 * source timestamp. This must be used instead of putting imported source text
 * directly into a `time[dateTime]` attribute.
 */
export function toSafeHtmlDateTime(isoTimestamp: string): string | null {
  const parsedTimestamp = parseTimestamp(isoTimestamp);
  if (!parsedTimestamp) return null;

  const normalizedTimestamp = parsedTimestamp.toISOString();
  return htmlGlobalDateTimePattern.test(normalizedTimestamp) ? normalizedTimestamp : null;
}

function formatTimestamp(
  timestamp: Date,
  language: AppLanguage,
  formatOptions: Intl.DateTimeFormatOptions,
  options: TimestampFormatOptions | undefined,
): string {
  return new Intl.DateTimeFormat(dateLocales[language], {
    ...formatOptions,
    ...(options?.timeZone ? { timeZone: options.timeZone } : {}),
  }).format(timestamp);
}

function fallbackFor(language: AppLanguage, unavailableLabel?: string): string {
  return unavailableLabel ?? uiText[language].dateUnavailable;
}

/**
 * Formats a historical source timestamp using the active Life OS language and
 * the device timezone. Source cards intentionally show the date only.
 */
export function formatHistoricalSourceDate(
  isoTimestamp: string,
  language: AppLanguage,
  unavailableLabel?: string,
  options?: TimestampFormatOptions,
): string {
  const parsedTimestamp = parseTimestamp(isoTimestamp);
  if (!parsedTimestamp) return fallbackFor(language, unavailableLabel);

  return formatTimestamp(parsedTimestamp, language, { dateStyle: "medium" }, options);
}

/**
 * Formats an Experience card timestamp using the active Life OS language and
 * the device timezone, without changing the stored source timestamp.
 */
export function formatExperienceTimestamp(
  isoTimestamp: string,
  language: AppLanguage,
  unavailableLabel: string,
  options?: TimestampFormatOptions,
): string {
  const parsedTimestamp = parseTimestamp(isoTimestamp);
  if (!parsedTimestamp) return fallbackFor(language, unavailableLabel);

  return formatTimestamp(
    parsedTimestamp,
    language,
    { dateStyle: "medium", timeStyle: "short" },
    options,
  );
}
