/**
 * Ephemeral UI-only state for explicitly opened historical-context panels.
 * A Set retains each exact Experience ID without object-property lookup or
 * delimiter-composed keys, and intentionally has no persistence format.
 */
export type HistoricalContextOpenPanels = ReadonlySet<string>;

export function isHistoricalContextPanelOpen(
  openPanels: HistoricalContextOpenPanels,
  experienceId: string,
): boolean {
  return openPanels.has(experienceId);
}

export function openHistoricalContextPanel(
  openPanels: HistoricalContextOpenPanels,
  experienceId: string,
): HistoricalContextOpenPanels {
  if (openPanels.has(experienceId)) {
    return openPanels;
  }

  const next = new Set(openPanels);
  next.add(experienceId);
  return next;
}

export function closeHistoricalContextPanel(
  openPanels: HistoricalContextOpenPanels,
  experienceId: string,
): HistoricalContextOpenPanels {
  if (!openPanels.has(experienceId)) {
    return openPanels;
  }

  const next = new Set(openPanels);
  next.delete(experienceId);
  return next;
}

export function toggleHistoricalContextPanel(
  openPanels: HistoricalContextOpenPanels,
  experienceId: string,
): HistoricalContextOpenPanels {
  return openPanels.has(experienceId)
    ? closeHistoricalContextPanel(openPanels, experienceId)
    : openHistoricalContextPanel(openPanels, experienceId);
}

/**
 * Removes an Experience's panel state after that exact Experience is deleted.
 */
export function removeHistoricalContextPanelForExperience(
  openPanels: HistoricalContextOpenPanels,
  experienceId: string,
): HistoricalContextOpenPanels {
  return closeHistoricalContextPanel(openPanels, experienceId);
}
