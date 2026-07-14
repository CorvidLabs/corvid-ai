---
spec: config.spec.md
---

## Context

Host tools need one deterministic place to combine caller overrides, registry
defaults, and provider-specific environment variables.

## Related Modules

- `registry` supplies provider metadata.
- `provider` is the resolved request target.
- `error` defines typed resolution failures.

## Design Decisions

- Empty strings behave as missing values so serialized configuration cannot
  accidentally suppress useful defaults.
- Key requirements follow the wire protocol: Anthropic and Gemini require a
  key, while OpenAI-compatible local endpoints may be keyless.
