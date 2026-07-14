---
module: crate-api
version: 2
status: active
files:
  - src/lib.rs
db_tables: []
depends_on:
  - config
  - error
  - provider
  - registry
---

# Crate API

## Purpose

Defines the public root of the synchronous `corvid-ai` library. The facade exposes configuration, provider, registry, and error types from their canonical modules and supplies the one-call completion convenience function.

## Public API

### Exported Functions

| Function | Signature | Description |
|---|---|---|
| `complete` | `(&Settings, &Completion) -> Result<String>` | Resolve settings and synchronously execute one provider completion. |

### Exported Modules and Re-exports

| Export | Description |
|---|---|
| `error` | Public typed error module. |
| `provider` | Public completion and provider module. |
| `registry` | Public built-in provider registry module. |
| `DEFAULT_PROVIDER` | Re-exported default provider name. |
| `DEFAULT_TIMEOUT_SECS` | Re-exported default request timeout. |
| `Settings` | Re-exported loose configuration type. |
| `resolve` | Re-exported settings resolution function. |
| `Error` | Re-exported typed error enum. |
| `Result` | Re-exported result alias. |
| `Completion` | Re-exported completion request type. |
| `DEFAULT_MAX_TOKENS` | Re-exported default response token ceiling. |
| `Provider` | Re-exported resolved provider type. |
| `Kind` | Re-exported wire-protocol enum. |
| `ProviderSpec` | Re-exported provider registry row type. |
| `REGISTRY` | Re-exported built-in provider table. |
| `known_names` | Re-exported provider-name listing function. |

## Invariants

1. The crate remains synchronous and introduces no CLI shell-out or async runtime.
2. `complete` is behaviorally equivalent to `resolve` followed by `Provider::complete` using the resolved timeout.
3. Provider, registry, configuration, and error behavior remains owned by their dedicated modules.

## Behavioral Examples

Given settings for a known provider and a completion prompt, when `complete` is called, then settings are resolved and the selected provider executes the request with the resolved timeout.

## Error Cases

| Condition | Behavior |
|---|---|
| Settings cannot resolve | `complete` returns the corresponding typed configuration error. |
| The provider request fails | `complete` returns the typed provider error unchanged. |

## Dependencies

- `config` for settings resolution.
- `provider` for completion execution.
- `error` for the shared result surface.
- `registry` for public provider metadata re-exports.

## Change Log

| Version | Date | Changes |
|---|---|---|
| 1 | 2026-07-13 | Record the existing crate facade and one-call completion behavior. |
| 2 | 2026-07-14 | CHG-0003-complete-the-canonical-corvid-ai-contract-for-the-crate-facade-and-secret-redact: Complete the canonical corvid-ai contract for the crate facade and secret redaction module |
