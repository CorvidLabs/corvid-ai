---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai
artifact: testing
---

# Testing

- Run specsync check --strict --require-coverage 100 --force.
- Confirm specsync agents status reports Claude, Cursor, Codex, and Gemini.
- Run fledge lanes run verify for formatting, Clippy, and all Rust tests.
- Run fledge trust doctor and fledge trust verify.
- Confirm the hosted Rust CI job and additive trust job run on the draft PR.
- Confirm the Atlas Pages workflow remains unchanged and independent.
- REQ-config-001, REQ-config-002, REQ-config-003, and REQ-config-004 are evidenced by the existing precedence, empty-override, key-policy, and typed-error config tests.
- REQ-error-001, REQ-error-002, and REQ-error-003 are evidenced by typed error paths, redaction tests, and provider failure-context tests.
- REQ-provider-001, REQ-provider-002, REQ-provider-003, REQ-provider-004, and REQ-provider-005 are evidenced by the pure Anthropic, OpenAI-compatible, and Gemini builder/parser tests plus URL and error tests.
- REQ-registry-001, REQ-registry-002, REQ-registry-003, and REQ-registry-004 are evidenced by registry lookup, table, default-model, and known-name invariant tests.
