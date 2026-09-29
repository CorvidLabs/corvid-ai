---
change: CHG-0002-record-the-exact-portable-specsync-5-0-1-and-trust-1-0-0-governance-migration-fo
artifact: testing
---

# Testing

Run all-agent status, `fledge lanes run verify`, released `specsync check --strict --require-coverage 100 --force`, `fledge trust doctor`, and `fledge trust verify`. Hosted Rust CI, CodeQL, and Trust must pass on the exact pull-request head.
