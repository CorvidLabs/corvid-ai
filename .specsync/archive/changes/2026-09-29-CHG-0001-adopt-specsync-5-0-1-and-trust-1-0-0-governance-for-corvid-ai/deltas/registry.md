## MODIFIED

### REQUIREMENT REQ-registry-001

Lookup SHALL trim and case-normalize names and return no row for an unknown provider.

Acceptance Criteria
- Existing case-insensitive lookup tests pass.

### REQUIREMENT REQ-registry-002

The registry SHALL contain the ten providers and three wire-protocol mappings listed by the canonical spec.

Acceptance Criteria
- The committed registry table retains all ten documented rows and three kinds.

### REQUIREMENT REQ-registry-003

Every row SHALL provide a slash-free base URL and API-key environment-variable name, and Anthropic/Gemini rows SHALL provide a default model.

Acceptance Criteria
- Existing registry invariant tests pass and table inspection confirms URL and environment fields.

### REQUIREMENT REQ-registry-004

The known-name list SHALL deterministically expose every registered name for unknown-provider diagnostics.

Acceptance Criteria
- Existing known-name tests pass.
