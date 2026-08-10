import type { UiCopy } from "./i18n";

interface DailyReflectionComposerProps {
  readonly copy: UiCopy;
  readonly body: string;
  readonly saving: boolean;
  readonly providerDetail: string;
  readonly onBodyChange: (value: string) => void;
  readonly onSave: () => void;
}

export function DailyReflectionComposer({
  copy,
  body,
  saving,
  providerDetail,
  onBodyChange,
  onSave,
}: DailyReflectionComposerProps) {
  return (
    <section className="welcome-card daily-reflection-primary" aria-labelledby="daily-reflection-heading">
      <div className="welcome-copy">
        <p className="soft-label">{copy.welcome}</p>
        <h1 id="daily-reflection-heading">{copy.hero}</h1>
        <p className="welcome-subtitle">{copy.subtitle}</p>
      </div>
      <div className="reflection-composer">
        <textarea
          id="daily-reflection-composer"
          aria-label={copy.experienceAria}
          placeholder={copy.experiencePlaceholder}
          value={body}
          onChange={(event) => onBodyChange(event.target.value)}
        />
        <div className="composer-footer">
          <p>{providerDetail}</p>
          <button
            type="button"
            className="primary-button"
            disabled={!body.trim() || saving}
            onClick={onSave}
          >
            {saving ? copy.saving : copy.saveMoment}
          </button>
        </div>
      </div>
    </section>
  );
}
