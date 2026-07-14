## MODIFIED

### REQUIREMENT REQ-config-001

Resolution SHALL apply provider, model, API key, base URL, and timeout values in the precedence order documented by the canonical spec.

Acceptance Criteria
- Existing precedence and override tests pass.

### REQUIREMENT REQ-config-002

Empty model, API-key, and base-URL strings SHALL be treated as unset.

Acceptance Criteria
- Existing resolution tests preserve fallback behavior for empty overrides.

### REQUIREMENT REQ-config-003

Anthropic and Gemini SHALL require an API key, while OpenAI-compatible providers SHALL permit a missing key.

Acceptance Criteria
- Existing required-key and keyless Ollama tests pass.

### REQUIREMENT REQ-config-004

Unknown providers, missing models, and missing required keys SHALL return the corresponding typed error with provider context.

Acceptance Criteria
- Existing unknown-provider, missing-model, and missing-key tests pass.
