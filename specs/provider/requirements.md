---
spec: provider.spec.md
---

## User Stories

- As a developer, I want one synchronous completion API across Anthropic,
  OpenAI-compatible, and Gemini endpoints.

## Acceptance Criteria

### REQ-provider-001

Anthropic requests and responses SHALL follow the Messages API headers, body, endpoint, and text-block concatenation described by the canonical spec.

Acceptance Criteria
- Existing Anthropic builder and parser tests pass.

### REQ-provider-002

OpenAI-compatible requests SHALL add bearer authorization only for non-empty keys and SHALL return the first choice's message content.

Acceptance Criteria
- Existing OpenAI-compatible builder and parser tests pass.

### REQ-provider-003

Gemini requests and responses SHALL use the generateContent endpoint, `x-goog-api-key`, system instruction mapping, and part concatenation described by the canonical spec.

Acceptance Criteria
- Existing Gemini builder and parser tests pass.

### REQ-provider-004

Provider requests SHALL trim trailing base-URL slashes, apply the configured timeout, and return typed status, transport, decode, or empty-response errors.

Acceptance Criteria
- Existing URL, timeout, and response-error tests pass.

### REQ-provider-005

Request-body builders and response parsers SHALL remain pure and testable without network access.

Acceptance Criteria
- Existing pure builder and parser tests pass and the parser-complete Public API table documents every detected provider export.

## Constraints

- Requests are synchronous and use `ureq`; streaming is not supported.

## Out of Scope

- Async runtimes, streaming completions, retries, and tool calling.
