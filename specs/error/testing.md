---
spec: error.spec.md
---

## Test Plan

### Unit Tests

- Format every error variant with its identifying context.
- Verify secrets and bearer tokens are absent after redaction.

### Integration Tests

- Exercise errors produced by configuration resolution and response parsing.
