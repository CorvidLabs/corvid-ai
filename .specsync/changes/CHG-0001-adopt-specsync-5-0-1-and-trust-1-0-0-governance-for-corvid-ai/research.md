---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai
artifact: research
---

# Research

- The crate is a dependency-free CorvidLabs leaf with four active specs mapped
  one-to-one to its source modules.
- Existing pull-request CI runs formatting, Clippy with warnings denied, and
  the Rust test suite on Ubuntu.
- Atlas coverage publication is a separate push-only Pages workflow with its
  own permissions, cache, retry, and concurrency behavior.
- The standard rollout contract requires SpecSync 5.0.1, all four agent
  integrations, immutable Trust 1.0.0, blocking risk, progressive provenance,
  and Trust-managed Atlas disabled.
