# Current Mission

Status: ready

- Sprint ID: 2026-08-11-historical-relevance-origin-disclosure-correction
- Mission title: Historical Relevance Origin Disclosure Correction
- Origin: founder_request
- Base branch: develop
- Starting commit: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Background: Founder review showed local-history terms absent from the visible Experience excerpt because retrieval aggregates Experience, confirmed Evidence, and saved Reflection matches without disclosing origin.
- Problem: The aggregate reason is true to the retrieval corpus but not inspectable from the candidate card.
- Intended outcome: Preserve local-lexical-v2 matching, ranking, caps, and governed relevance while showing local-only per-origin terms and bounded matching excerpts; return to Step 9R.
- Initial scope: historical retrieval types/implementation/tests, App candidate rendering, i18n/tests, and workflow evidence.
- Explicit non-scope: No persistence, selection, consent, provider, ContextPacket, packet digest, schema, Phase 4, whole-history, commit, push, or merge change.
- Relevant Book Zero definitions: docs/03_Principles.md, docs/06_Memory.md, docs/Reflection.md.
- Relevant ADRs: ADR-0009 remains unchanged.
- Relevant architecture documents: architecture/08 and architecture/09.
- Constraints: Preserve all pre-existing unstaged audit and correction work.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-11T09:05:53.033Z