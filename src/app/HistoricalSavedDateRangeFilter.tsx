import type { HistoricalSavedDateRangeControl } from "../historicalContext/savedDateRange";
import type { UiCopy } from "./i18n";

interface HistoricalSavedDateRangeFilterProps {
  readonly control: HistoricalSavedDateRangeControl;
  readonly message: string | null;
  readonly copy: UiCopy;
  readonly onEnabledChange: (enabled: boolean) => void;
  readonly onStartChange: (startDate: string) => void;
  readonly onEndChange: (endDate: string) => void;
  readonly onApply: () => void;
}

export function HistoricalSavedDateRangeFilter({
  control,
  message,
  copy,
  onEnabledChange,
  onStartChange,
  onEndChange,
  onApply,
}: HistoricalSavedDateRangeFilterProps) {
  return (
    <div className="historical-saved-date-filter">
      <label className="historical-saved-date-toggle">
        <input
          type="checkbox"
          checked={control.enabled}
          onChange={(event) => onEnabledChange(event.target.checked)}
        />
        <span>{copy.historicalSavedDateFilterLabel}</span>
      </label>
      {control.enabled ? (
        <div className="historical-saved-date-fields">
          <label>
            <span>{copy.historicalSavedDateStartLabel}</span>
            <input
              type="date"
              value={control.startDate}
              onChange={(event) => onStartChange(event.target.value)}
            />
          </label>
          <label>
            <span>{copy.historicalSavedDateEndLabel}</span>
            <input
              type="date"
              value={control.endDate}
              onChange={(event) => onEndChange(event.target.value)}
            />
          </label>
          <button
            type="button"
            className="ghost-button compact"
            onClick={onApply}
          >
            {copy.historicalSavedDateApply}
          </button>
        </div>
      ) : null}
      {message ? (
        <p className="historical-saved-date-message" role="alert">
          {message}
        </p>
      ) : null}
    </div>
  );
}
