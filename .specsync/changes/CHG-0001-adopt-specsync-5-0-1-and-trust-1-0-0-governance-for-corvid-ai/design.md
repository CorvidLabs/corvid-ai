---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai
artifact: design
---

# Design

Add one trust job on every pull request and main push. The job checks out full
history, installs the stable Rust toolchain with rustfmt and Clippy, restores
the Cargo cache, and invokes Trust at its immutable v1.0.0 commit.

Trust delegates lifecycle verification to a Fledge lane containing the same
format, lint, and test commands as existing CI. Contract coverage remains 100
percent. Risk blocks at the standard threshold, provenance is soft against the
committed Attest policy, and built-in Atlas is disabled because the existing
Pages workflow remains authoritative.

No runtime API, dependency, provider behavior, secret flow, or release behavior
changes.
