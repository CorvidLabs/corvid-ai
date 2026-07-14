---
spec: redaction.spec.md
---

# Redaction Testing

- REQ-redaction-001 maps to `redact::tests::scrubs_bearer_token`.
- REQ-redaction-002 maps to `redact::tests::scrubs_key_query_param` and direct inspection confirms `api_key=` uses the same tested helper.
- REQ-redaction-003 maps to `redact::tests::passes_clean_text_through`.

Run `fledge lanes run verify`; these tests execute inside the crate's existing unit-test suite.
