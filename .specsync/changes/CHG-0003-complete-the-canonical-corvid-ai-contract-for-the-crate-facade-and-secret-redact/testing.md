---
change: CHG-0003-complete-the-canonical-corvid-ai-contract-for-the-crate-facade-and-secret-redact
artifact: testing
---

# Testing

- REQ-crate-api-001 and REQ-crate-api-002 map to the crate doctest, module unit tests, and direct facade delegation.
- REQ-redaction-001, REQ-redaction-002, and REQ-redaction-003 map to the three existing redaction unit tests.

Run the native Fledge lane and released SpecSync 5.0.1 at strict 100% coverage. Confirm no `src/` diff exists.
