---
id: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai
state: archived
type: migration
base_commit: e18ff01dce558b5c6f51dd2673daa15282a79dc8
---

# Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for corvid-ai

## Intent

Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for corvid-ai

## Affected Canonical Specs

- `config`
- `error`
- `provider`
- `registry`

## Acceptance Criteria

- SpecSync strict check passes at 100 percent coverage; all four agent integrations report installed; fledge trust doctor and native verification pass; existing Rust CI and Atlas Pages workflows remain intact; the additive immutable Trust gate runs on every pull request

## No-spec Rationale

Not applicable

## Migration Note

Migrated by hand to SpecSync 6 per Leif's decision (2026-09-28); the 6.0.0 tool refused to archive this legacy record (`` legacy accepted change `CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai` requires exactly one distinct valid historical reconstruction, found 0; first reconstruction failure: legacy accepted change `CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai` cannot reproduce its signed raw-content aggregate ``).

- Workflow v1 (SpecSync 5) record, accepted on 2026-07-14 by the closing approval already stored in `approvals.json`. SpecSync 6.0.0 reports its accepted evidence as stale, for the reason quoted above.
- Moved by hand from `.specsync/changes/CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai/` into the layout `specsync change archive` writes: `accepted-state.json` is the unchanged accepted `state.json`, `state.json` is marked `archived`, and this file's front matter says `archived`.
- `approvals.json`, `verification.json`, `verification-attempts.json` and every other artifact are the original SpecSync 5 evidence, unchanged. `verification.json` verifies commit `d8a5f794531105eb75b0c4e836794fb11e6aea0c`, not the tree this record was archived from.
- This migration added no verification evidence, test result, attempt history, or approval. It is a manual migration, not a fresh re-verification.
- Closing it through the tool takes `specsync change reopen`, `specsync change verify`, then `specsync change accept`, which writes a new closing approval. Per Leif's decision it was archived by hand instead, so no reopen or new approval is recorded.
