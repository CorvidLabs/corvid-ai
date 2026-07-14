---
spec: redaction.spec.md
---

# Redaction Requirements

## Acceptance Criteria

### REQ-redaction-001

Redaction SHALL replace non-whitespace token text after `Bearer ` and `bearer ` with `[REDACTED]` while retaining the marker.

Acceptance Criteria
- The existing bearer-token test passes.

### REQ-redaction-002

Redaction SHALL replace non-whitespace token text after `key=` and `api_key=` with `[REDACTED]` while retaining the marker.

Acceptance Criteria
- The existing query-key test and shared-helper inspection pass.

### REQ-redaction-003

Redaction SHALL preserve text that contains none of the supported secret markers.

Acceptance Criteria
- The existing clean-text test passes.

