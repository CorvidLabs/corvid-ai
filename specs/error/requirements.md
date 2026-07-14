---
spec: error.spec.md
---

## User Stories

- As a developer, I want typed, credential-safe failures that a host can handle
  programmatically and present safely.

## Acceptance Criteria

### REQ-error-001

The crate SHALL expose typed variants for unknown providers, missing keys, missing models, HTTP status failures, transport failures, decode failures, and empty responses.

Acceptance Criteria
- Existing error-path tests pass and the parser-complete Public API table documents both detected error exports.

### REQ-error-002

Formatted errors SHALL NOT expose API keys or bearer tokens.

Acceptance Criteria
- Existing redaction unit tests pass before error text is exposed.

### REQ-error-003

Request failures SHALL preserve the provider, status, or URL context specified by the canonical spec.

Acceptance Criteria
- Existing provider response and failure tests pass with typed context.

## Constraints

- Third-party transport messages must pass through redaction before storage.

## Out of Scope

- Retry policy and provider-specific recovery behavior.
