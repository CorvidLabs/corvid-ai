---
module: redaction
version: 2
status: active
files:
  - src/redact.rs
db_tables: []
depends_on: []
---

# Redaction

## Purpose

Provides the internal secret-scrubbing boundary used before transport and decode error details are exposed. It removes token material following the supported authorization and query markers without introducing a regular-expression dependency.

## Public API

### Exported Functions

| Function | Signature | Description |
|---|---|---|
| `redact` | `(&str) -> String` | Replace token text following supported secret markers with `[REDACTED]`. |

The containing module is private to the crate; `redact` is consumed by the provider implementation and is not re-exported from the crate root.

## Invariants

1. Token text following `Bearer `, `bearer `, `key=`, or `api_key=` never survives in returned text.
2. The marker itself remains present and is followed by `[REDACTED]` when a token exists.
3. Input containing none of the supported markers is returned unchanged.
4. Redaction ends at the next whitespace boundary or at end of input.

## Behavioral Examples

Given `Authorization: Bearer sk-secret nope`, when `redact` is called, then the result contains `Bearer [REDACTED] nope` and not the token.

Given a clean connection error, when `redact` is called, then the returned text equals the input.

## Error Cases

Redaction is total and returns a `String`; it does not return errors. A marker without following token text is retained without adding a placeholder.

## Dependencies

- Rust standard-library string operations only.

## Change Log

| Version | Date | Changes |
|---|---|---|
| 1 | 2026-07-13 | Record the existing marker-based secret redaction behavior. |
| 2 | 2026-07-14 | CHG-0003-complete-the-canonical-corvid-ai-contract-for-the-crate-facade-and-secret-redact: Complete the canonical corvid-ai contract for the crate facade and secret redaction module |
