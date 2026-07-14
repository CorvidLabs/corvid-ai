---
spec: crate-api.spec.md
---

# Crate API Testing

- REQ-crate-api-001 is evidenced by the existing crate doctest and compilation of callers through the root re-exports.
- REQ-crate-api-002 is evidenced by the existing unit coverage of `resolve` and `Provider::complete` plus direct inspection of the one-line delegation in `src/lib.rs`.

Run `fledge lanes run verify` for formatting, Clippy, 23 unit tests, and the crate doctest.
