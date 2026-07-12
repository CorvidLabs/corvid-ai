---
spec: config.spec.md
---

## Test Plan

### Unit Tests

- Exercise defaults and every caller override.
- Treat empty strings as unset.
- Cover required-key, missing-model, and unknown-provider errors.

### Integration Tests

- Resolve representative Anthropic, Gemini, hosted OpenAI-compatible, and
  keyless local-provider settings without making network requests.
