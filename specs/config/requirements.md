---
spec: config.spec.md
---

## User Stories

- As a developer, I want loose settings resolved consistently so every host
  tool reaches the same provider configuration.

## Acceptance Criteria

### REQ-config-001

Resolution SHALL apply provider, model, API key, base URL, and timeout values
in the precedence order documented by the canonical spec.

### REQ-config-002

Empty model, API-key, and base-URL strings SHALL be treated as unset.

### REQ-config-003

Anthropic and Gemini SHALL require an API key, while OpenAI-compatible
providers SHALL permit a missing key.

### REQ-config-004

Unknown providers, missing models, and missing required keys SHALL return the
corresponding typed error with provider context.

## Constraints

- Environment-based resolution depends on the provider row's committed
  environment-variable name.

## Out of Scope

- Persisting configuration and making network requests.
