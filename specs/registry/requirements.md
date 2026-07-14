---
spec: registry.spec.md
---

## User Stories

- As a developer, I want provider metadata in one deterministic registry so a
  new compatible gateway does not require new HTTP code.

## Acceptance Criteria

### REQ-registry-001

Lookup SHALL trim and case-normalize names and return no row for an unknown provider.

Acceptance Criteria
- Existing registry lookup tests pass and the parser-complete Public API table documents every detected registry export.

### REQ-registry-002

The registry SHALL contain the ten providers and three wire-protocol mappings listed by the canonical spec.

Acceptance Criteria
- The committed registry table retains all ten documented rows and three kinds.

### REQ-registry-003

Every row SHALL provide a slash-free base URL and API-key environment-variable name, and Anthropic/Gemini rows SHALL provide a default model.

Acceptance Criteria
- Existing registry invariant tests pass and table inspection confirms URL and environment fields.

### REQ-registry-004

The known-name list SHALL deterministically expose every registered name for unknown-provider diagnostics.

Acceptance Criteria
- Existing known-name tests pass.

## Constraints

- Built-in model defaults are best-effort starting points and may drift from
  provider recommendations.

## Out of Scope

- Runtime registry mutation and remote provider discovery.
