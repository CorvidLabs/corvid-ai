---
id: CHG-0004-make-the-four-existing-corvid-ai-public-api-tables-parser-complete-without-chang
state: accepted
type: documentation
base_commit: d8a5f794531105eb75b0c4e836794fb11e6aea0c
---

# Make the four existing corvid-ai public API tables parser-complete without changing their contracts

## Intent

Make the four existing corvid-ai public API tables parser-complete without changing their contracts

## Affected Canonical Specs

- `config`
- `error`
- `provider`
- `registry`

## Acceptance Criteria

- Released SpecSync 5.0.1 documents every detected export in config; error; provider; and registry with zero strict warnings while preserving the existing contract text and Rust source

## No-spec Rationale

Not applicable
