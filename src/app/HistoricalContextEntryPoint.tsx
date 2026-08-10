import type { UiCopy } from "./i18n";

interface HistoricalContextEntryPointProps {
  copy: Pick<
    UiCopy,
    | "historicalContextShortcutTitle"
    | "historicalContextShortcutBody"
    | "historicalContextShortcutOpen"
    | "historicalContextShortcutView"
  >;
  controlsId: string;
  isOpen: boolean;
  onOpen: () => void;
}

/**
 * A discoverability shortcut only. Rendering it, ignoring it, or activating it
 * never selects a source and never records consent.
 */
export function HistoricalContextEntryPoint({
  copy,
  controlsId,
  isOpen,
  onOpen,
}: HistoricalContextEntryPointProps) {
  return (
    <aside className="historical-context-entry-point">
      <div>
        <strong>{copy.historicalContextShortcutTitle}</strong>
        <p>{copy.historicalContextShortcutBody}</p>
      </div>
      <button
        type="button"
        className="ghost-button compact"
        aria-controls={controlsId}
        aria-expanded={isOpen}
        onClick={onOpen}
      >
        {isOpen
          ? copy.historicalContextShortcutView
          : copy.historicalContextShortcutOpen}
      </button>
    </aside>
  );
}
