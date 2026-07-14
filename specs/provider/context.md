---
spec: provider.spec.md
---

## Context

One small synchronous client must support three common LLM wire protocols
without an async runtime or provider-specific host integration.

## Related Modules

- `config` creates fully resolved providers.
- `error` defines request and response failures.

## Design Decisions

- Keep body construction and response parsing pure for deterministic tests.
- Model hosted and local OpenAI-compatible endpoints with the same variant and
  omit authorization entirely when no key is configured.
