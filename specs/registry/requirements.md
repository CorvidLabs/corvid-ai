---
spec: registry.spec.md
---

## User Stories

- As a developer, I want provider metadata in one deterministic registry so a
  new compatible gateway does not require new HTTP code.

## Acceptance Criteria

### REQ-registry-001

Lookup SHALL trim and case-normalize names and return no row for an unknown
provider.

### REQ-registry-002

The registry SHALL contain the ten providers and three wire-protocol mappings
listed by the canonical spec.

### REQ-registry-003

Every row SHALL provide a slash-free base URL and API-key environment-variable
name, and Anthropic/Gemini rows SHALL provide a default model.

### REQ-registry-004

The known-name list SHALL deterministically expose every registered name for
unknown-provider diagnostics.

## Constraints

- Built-in model defaults are best-effort starting points and may drift from
  provider recommendations.

## Out of Scope

- Runtime registry mutation and remote provider discovery.
