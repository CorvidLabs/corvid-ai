---
spec: error.spec.md
---

## User Stories

- As a developer, I want typed, credential-safe failures that a host can handle
  programmatically and present safely.

## Acceptance Criteria

### REQ-error-001

The crate SHALL expose typed variants for unknown providers, missing keys,
missing models, HTTP status failures, transport failures, decode failures, and
empty responses.

### REQ-error-002

Formatted errors SHALL NOT expose API keys or bearer tokens.

### REQ-error-003

Request failures SHALL preserve the provider, status, or URL context specified
by the canonical spec.

## Constraints

- Third-party transport messages must pass through redaction before storage.

## Out of Scope

- Retry policy and provider-specific recovery behavior.
