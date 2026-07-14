## MODIFIED

### REQUIREMENT REQ-error-001

The crate SHALL expose typed variants for unknown providers, missing keys, missing models, HTTP status failures, transport failures, decode failures, and empty responses.

Acceptance Criteria
- Existing error-path tests pass and the parser-complete Public API table documents both detected error exports.
