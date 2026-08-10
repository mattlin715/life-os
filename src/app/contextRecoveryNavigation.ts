interface RecoveryNavigationTarget {
  focus(options?: FocusOptions): void;
  scrollIntoView(options?: ScrollIntoViewOptions): void;
}

interface RecoveryNavigationEnvironment {
  getElementById(id: string): RecoveryNavigationTarget | null;
  requestFrame(callback: () => void): void;
}

export function focusContextRecovery(
  entryId: string,
  environment?: RecoveryNavigationEnvironment,
): void {
  const getElementById = environment?.getElementById
    ?? ((id: string) => document.getElementById(id));
  const requestFrame = environment?.requestFrame
    ?? ((callback: () => void) => {
      if (typeof requestAnimationFrame === "function") {
        requestAnimationFrame(callback);
      } else {
        callback();
      }
    });
  const focusRecovery = () => {
    const recovery = getElementById(`context-recovery-${entryId}`);
    if (!recovery) return;

    recovery.focus({ preventScroll: true });
    recovery.scrollIntoView({ behavior: "smooth", block: "center" });
  };

  requestFrame(focusRecovery);
}
