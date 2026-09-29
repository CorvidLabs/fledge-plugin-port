---
id: CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance
state: archived
type: feature
base_commit: a49b830a880362e2503c1dd2d2e1428b8c47864c
---

# Complete Port requirement evidence and generated integration guidance

## Intent

Complete Port requirement evidence and generated integration guidance

## Affected Canonical Specs

- `port`

## Acceptance Criteria

- All five canonical requirements have truthful concrete acceptance criteria; the semantic delta maps REQ-port-001 through REQ-port-005; generated Claude Cursor and Gemini commands preserve full free-text descriptions and Gemini arguments; strict SpecSync reports real 100% file and LOC coverage; native and hosted platform gates pass

## No-spec Rationale

Not applicable

## Migration Note

Migrated by hand to SpecSync 6 per Leif's decision (2026-09-28); the 6.0.0 tool refused to archive this legacy record (`` exact-only delivery input `.github/workflows/trust.yml` changed after acceptance and requires an audited reopen; run `specsync change reopen CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance` to re-verify the accepted change, or supersede it from a later change under a module granted the path by `owns` in `.specsync/config.toml` ``).

- Workflow v1 (SpecSync 5) record, accepted on 2026-07-13 by the closing approval already stored in `approvals.json`. SpecSync 6.0.0 reports its accepted evidence as stale, for the reason quoted above.
- Moved by hand from `.specsync/changes/CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance/` into the layout `specsync change archive` writes: `accepted-state.json` is the unchanged accepted `state.json`, `state.json` is marked `archived`, and this file's front matter says `archived`.
- `approvals.json`, `verification.json` and every other artifact are the original SpecSync 5 evidence, unchanged. `verification.json` verifies commit `e5d6d0587094cc007e04273d6f317c0dce52ad0e`, not the tree this record was archived from.
- There is no `verification-attempts.json`: SpecSync 5 did not write one for this record, and this migration does not invent attempt history.
- This migration added no verification evidence, test result, attempt history, or approval. It is a manual migration, not a fresh re-verification.
- Closing it through the tool takes `specsync change reopen`, `specsync change verify`, then `specsync change accept`, which writes a new closing approval. Per Leif's decision it was archived by hand instead, so no reopen or new approval is recorded.
