---
spec: error.spec.md
---

## Context

Consumers need actionable failure categories without risking credential
exposure in logs or user-facing messages.

## Related Modules

- `config` constructs resolution failures.
- `provider` constructs request and response failures.

## Design Decisions

- Keep provider, status, and URL context typed.
- Redact transport details before constructing displayable errors.
