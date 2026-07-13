/**
 * Ephemeral local UI choices. Map and Set retain exact IDs without composing
 * keys, and intentionally have no JSON persistence representation.
 */
export type HistoricalContextSelections = ReadonlyMap<string, ReadonlySet<string>>;

export function isHistoricalContextSelected(
  selections: HistoricalContextSelections,
  currentExperienceId: string,
  sourceExperienceId: string,
): boolean {
  return selections.get(currentExperienceId)?.has(sourceExperienceId) ?? false;
}

export function toggleHistoricalContextSelection(
  selections: HistoricalContextSelections,
  currentExperienceId: string,
  sourceExperienceId: string,
): HistoricalContextSelections {
  const currentSources = selections.get(currentExperienceId);
  const next = new Map(selections);

  if (currentSources?.has(sourceExperienceId)) {
    const remainingSources = new Set(currentSources);
    remainingSources.delete(sourceExperienceId);
    if (remainingSources.size === 0) {
      next.delete(currentExperienceId);
    } else {
      next.set(currentExperienceId, remainingSources);
    }
    return next;
  }

  const nextSources = new Set(currentSources);
  nextSources.add(sourceExperienceId);
  next.set(currentExperienceId, nextSources);
  return next;
}

export function clearHistoricalContextSelection(
  selections: HistoricalContextSelections,
  currentExperienceId: string,
): HistoricalContextSelections {
  if (!selections.has(currentExperienceId)) {
    return selections;
  }

  const next = new Map(selections);
  next.delete(currentExperienceId);
  return next;
}

/**
 * Removes an Experience from both exact-ID roles: current panel and source.
 */
export function removeHistoricalContextSelectionsForExperience(
  selections: HistoricalContextSelections,
  experienceId: string,
): HistoricalContextSelections {
  let changed = false;
  const next = new Map<string, ReadonlySet<string>>();

  for (const [currentExperienceId, sourceExperienceIds] of selections) {
    if (currentExperienceId === experienceId) {
      changed = true;
      continue;
    }

    if (!sourceExperienceIds.has(experienceId)) {
      next.set(currentExperienceId, sourceExperienceIds);
      continue;
    }

    changed = true;
    const remainingSources = new Set(sourceExperienceIds);
    remainingSources.delete(experienceId);
    if (remainingSources.size > 0) {
      next.set(currentExperienceId, remainingSources);
    }
  }

  return changed ? next : selections;
}

/**
 * Drops sources which are no longer eligible candidates for one current
 * Experience. It never selects a candidate, so a later reappearance remains
 * unselected. Returning the original reference for a no-op makes it safe to
 * call from a post-render reconciliation effect.
 */
export function reconcileHistoricalContextSelections(
  selections: HistoricalContextSelections,
  currentExperienceId: string,
  eligibleSourceExperienceIds: ReadonlySet<string>,
): HistoricalContextSelections {
  const currentSources = selections.get(currentExperienceId);
  if (!currentSources) {
    return selections;
  }

  let changed = false;
  const remainingSources = new Set<string>();
  for (const sourceExperienceId of currentSources) {
    if (eligibleSourceExperienceIds.has(sourceExperienceId)) {
      remainingSources.add(sourceExperienceId);
    } else {
      changed = true;
    }
  }

  if (!changed) {
    return selections;
  }

  const next = new Map(selections);
  if (remainingSources.size === 0) {
    next.delete(currentExperienceId);
  } else {
    next.set(currentExperienceId, remainingSources);
  }
  return next;
}

/**
 * Reconciles every selected current Experience against the complete current
 * candidate view. A missing current key is treated as no longer eligible
 * (for example, after that Experience was deleted). The caller should supply
 * all current Experiences, not only panels which happen to be open.
 */
export function reconcileHistoricalContextSelectionsAgainstEligibleCandidates(
  selections: HistoricalContextSelections,
  eligibleSourceExperienceIdsByCurrentExperienceId: ReadonlyMap<string, ReadonlySet<string>>,
): HistoricalContextSelections {
  let changed = false;
  const next = new Map<string, ReadonlySet<string>>();

  for (const [currentExperienceId, sourceExperienceIds] of selections) {
    const eligibleSourceExperienceIds = eligibleSourceExperienceIdsByCurrentExperienceId.get(currentExperienceId);
    if (!eligibleSourceExperienceIds) {
      changed = true;
      continue;
    }

    let currentChanged = false;
    const remainingSources = new Set<string>();
    for (const sourceExperienceId of sourceExperienceIds) {
      if (eligibleSourceExperienceIds.has(sourceExperienceId)) {
        remainingSources.add(sourceExperienceId);
      } else {
        currentChanged = true;
      }
    }

    if (!currentChanged) {
      next.set(currentExperienceId, sourceExperienceIds);
      continue;
    }

    changed = true;
    if (remainingSources.size > 0) {
      next.set(currentExperienceId, remainingSources);
    }
  }

  return changed ? next : selections;
}
