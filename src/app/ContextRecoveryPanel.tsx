import type { ContextRecoveryTurn } from "../types/domain";
import type { UiCopy } from "./i18n";

interface ContextRecoveryPanelProps {
  copy: Pick<
    UiCopy,
    "recoveryTitle" | "recoveryNote" | "recoverySave" | "recoverySkip" | "recoveryMutationFailed"
  >;
  entryId: string;
  turns: ContextRecoveryTurn[];
  mutationFailed: boolean;
  onResponseChange: (turnId: string, response: string) => void;
  onSave: (turnId: string) => void;
  onSkip: (turnId: string) => void;
}

export function ContextRecoveryPanel({
  copy,
  entryId,
  turns,
  mutationFailed,
  onResponseChange,
  onSave,
  onSkip,
}: ContextRecoveryPanelProps) {
  return (
    <div
      aria-label={copy.recoveryTitle}
      className="session-note"
      id={`context-recovery-${entryId}`}
      role="region"
      tabIndex={-1}
    >
      <strong>{copy.recoveryTitle}</strong>
      <p>{copy.recoveryNote}</p>
      {mutationFailed ? <p role="alert">{copy.recoveryMutationFailed}</p> : null}
      {turns.map((turn) => (
        <div key={turn.id} className="candidate-card">
          <p>{turn.question}</p>
          <textarea
            value={turn.response ?? ""}
            disabled={turn.status !== "suggested"}
            onChange={(event) => onResponseChange(turn.id, event.target.value)}
          />
          {turn.status === "suggested" ? (
            <div className="candidate-actions">
              <button type="button" className="ghost-button compact" onClick={() => onSave(turn.id)}>
                {copy.recoverySave}
              </button>
              <button type="button" className="ghost-button compact" onClick={() => onSkip(turn.id)}>
                {copy.recoverySkip}
              </button>
            </div>
          ) : null}
        </div>
      ))}
    </div>
  );
}
