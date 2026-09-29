## MODIFIED

### REQUIREMENT REQ-crate-api-001

The crate root SHALL expose the existing configuration, provider, error, and registry surface from one import location.

Acceptance Criteria
- The existing crate doctest compiles through the root facade.

### REQUIREMENT REQ-crate-api-002

The `complete` convenience function SHALL resolve `Settings` and call `Provider::complete` with the resolved timeout.

Acceptance Criteria
- Existing module tests and direct delegation inspection verify the one-call behavior without changing Rust source.
