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
