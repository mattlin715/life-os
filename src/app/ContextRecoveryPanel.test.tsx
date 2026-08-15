import { Children, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import type { ContextRecoveryTurn } from "../types/domain";
import { ContextRecoveryPanel } from "./ContextRecoveryPanel";
import { uiText } from "./i18n";

const turn: ContextRecoveryTurn = {
  id: "recovery",
  sourceEntryId: "entry",
  question: "What happened?",
  status: "suggested",
  locale: "en",
  promptProvenance: {
    origin: "local_mock",
    sourceEntryId: "entry",
    sourceArtifactIds: [],
    provider: "mock",
    model: null,
    harnessVersion: "harness-v1",
    promptVersion: "v1",
    generatedAt: "2026-08-16T00:00:00.000Z",
  },
  createdAt: "2026-08-16T00:00:00.000Z",
  updatedAt: "2026-08-16T00:00:00.000Z",
};

describe("ContextRecoveryPanel", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "shows the localized mutation failure inside the active region in %s",
    (locale) => {
      const html = renderToStaticMarkup(
        <ContextRecoveryPanel
          copy={uiText[locale]}
          entryId="entry"
          turns={[turn]}
          mutationFailed
          onResponseChange={() => undefined}
          onSave={() => undefined}
          onSkip={() => undefined}
        />,
      );
      expect(html).toContain(`id="context-recovery-entry"`);
      expect(html).toContain('role="alert"');
      expect(html).toContain(uiText[locale].recoveryMutationFailed);
      expect(html).toContain(uiText[locale].recoverySave);
      expect(html).toContain(uiText[locale].recoverySkip);
    },
  );

  it("does not invoke an action until the Founder activates its button", () => {
    const onSave = vi.fn();
    const panel = ContextRecoveryPanel({
      copy: uiText.en,
      entryId: "entry",
      turns: [turn],
      mutationFailed: false,
      onResponseChange: () => undefined,
      onSave,
      onSkip: () => undefined,
    });
    expect(onSave).not.toHaveBeenCalled();
    const children = Children.toArray(panel.props.children);
    const candidate = children[children.length - 1] as ReactElement;
    const actions = Children.toArray(candidate.props.children)[2] as ReactElement;
    const save = Children.toArray(actions.props.children)[0] as ReactElement<{ onClick: () => void }>;
    save.props.onClick();
    expect(onSave).toHaveBeenCalledWith("recovery");
  });

  it("omits the alert after a successful mutation", () => {
    const html = renderToStaticMarkup(
      <ContextRecoveryPanel
        copy={uiText.en}
        entryId="entry"
        turns={[turn]}
        mutationFailed={false}
        onResponseChange={() => undefined}
        onSave={() => undefined}
        onSkip={() => undefined}
      />,
    );
    expect(html).not.toContain('role="alert"');
  });
});
