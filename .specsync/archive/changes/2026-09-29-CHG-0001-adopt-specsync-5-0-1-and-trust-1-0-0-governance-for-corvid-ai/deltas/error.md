## MODIFIED

### REQUIREMENT REQ-error-001

The crate SHALL expose typed variants for unknown providers, missing keys, missing models, HTTP status failures, transport failures, decode failures, and empty responses.

Acceptance Criteria
- The typed enum compiles and existing error-path tests pass.

### REQUIREMENT REQ-error-002

Formatted errors SHALL NOT expose API keys or bearer tokens.

Acceptance Criteria
- Existing redaction unit tests pass before error text is exposed.

### REQUIREMENT REQ-error-003

Request failures SHALL preserve the provider, status, or URL context specified by the canonical spec.

Acceptance Criteria
- Existing provider response and failure tests pass with typed context.
