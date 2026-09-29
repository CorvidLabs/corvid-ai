---
id: CHG-0002-record-the-exact-portable-specsync-5-0-1-and-trust-1-0-0-governance-migration-fo
state: archived
type: migration
base_commit: d8a5f794531105eb75b0c4e836794fb11e6aea0c
---

# Record the exact portable SpecSync 5.0.1 and Trust 1.0.0 governance migration for corvid-ai

## Intent

Record the exact portable SpecSync 5.0.1 and Trust 1.0.0 governance migration for corvid-ai

## Affected Canonical Specs

- None

## Acceptance Criteria

- Every migration path is covered exactly; all four agent integrations are installed; released SpecSync 5.0.1 measures every Rust source file at 100%; native verification and Trust pass; no Rust product source changes

## No-spec Rationale

This successor covers governance, agent integration, existing companion documentation, and verification configuration only; Rust runtime behavior and the canonical contracts remain unchanged.

## Migration Note

Migrated by hand to SpecSync 6 per Leif's decision (2026-09-28); the 6.0.0 tool refused to archive this legacy record (`` legacy accepted change `CHG-0002-record-the-exact-portable-specsync-5-0-1-and-trust-1-0-0-governance-migration-fo` requires exactly one distinct valid historical reconstruction, found 0; first reconstruction failure: legacy accepted change `CHG-0002-record-the-exact-portable-specsync-5-0-1-and-trust-1-0-0-governance-migration-fo` cannot reproduce its signed raw-content aggregate ``).

- Workflow v1 (SpecSync 5) record, accepted on 2026-07-14 by the closing approval already stored in `approvals.json`. SpecSync 6.0.0 reports its accepted evidence as stale, for the reason quoted above.
- Moved by hand from `.specsync/changes/CHG-0002-record-the-exact-portable-specsync-5-0-1-and-trust-1-0-0-governance-migration-fo/` into the layout `specsync change archive` writes: `accepted-state.json` is the unchanged accepted `state.json`, `state.json` is marked `archived`, and this file's front matter says `archived`.
- `approvals.json`, `verification.json`, `verification-attempts.json` and every other artifact are the original SpecSync 5 evidence, unchanged. `verification.json` verifies commit `d8a5f794531105eb75b0c4e836794fb11e6aea0c`, not the tree this record was archived from.
- This migration added no verification evidence, test result, attempt history, or approval. It is a manual migration, not a fresh re-verification.
- Closing it through the tool takes `specsync change reopen`, `specsync change verify`, then `specsync change accept`, which writes a new closing approval. Per Leif's decision it was archived by hand instead, so no reopen or new approval is recorded.
