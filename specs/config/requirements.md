---
spec: config.spec.md
---

## User Stories

- As a developer, I want loose settings resolved consistently so every host
  tool reaches the same provider configuration.

## Acceptance Criteria

### REQ-config-001

Resolution SHALL apply provider, model, API key, base URL, and timeout values in the precedence order documented by the canonical spec.

Acceptance Criteria
- Existing configuration resolution tests pass and the parser-complete Public API table documents every detected config export.

### REQ-config-002

Empty model, API-key, and base-URL strings SHALL be treated as unset.

Acceptance Criteria
- Existing resolution tests preserve fallback behavior for empty overrides.

### REQ-config-003

Anthropic and Gemini SHALL require an API key, while OpenAI-compatible providers SHALL permit a missing key.

Acceptance Criteria
- Existing required-key and keyless Ollama tests pass.

### REQ-config-004

Unknown providers, missing models, and missing required keys SHALL return the corresponding typed error with provider context.

Acceptance Criteria
- Existing unknown-provider, missing-model, and missing-key tests pass.

## Constraints

- Environment-based resolution depends on the provider row's committed
  environment-variable name.

## Out of Scope

- Persisting configuration and making network requests.
