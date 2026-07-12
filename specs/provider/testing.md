---
spec: provider.spec.md
---

## Test Plan

### Unit Tests

- Verify bodies, headers, URLs, system prompts, token limits, and response text
  extraction for all three protocols.
- Cover malformed, empty, non-success, and redacted transport failures.

### Integration Tests

- Exercise each provider against a local deterministic HTTP fixture; live API
  calls and credentials are not required by the default gate.
