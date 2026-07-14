---
spec: registry.spec.md
---

## Context

Provider selection needs a small, reviewable table that separates endpoint and
credential metadata from the three reusable HTTP wire implementations.

## Related Modules

- `config` consumes registry rows during settings resolution.
- `provider` implements each row's selected wire protocol.

## Design Decisions

- Normalize user input during lookup rather than duplicating aliases.
- Allow selected OpenAI-compatible rows to omit a default model so callers
  make an explicit model choice.
