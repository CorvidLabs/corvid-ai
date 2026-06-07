---
module: error
version: 1
status: active
files:
  - src/error.rs

db_tables: []
depends_on: []
---

# Error

## Purpose

The crate's typed error surface and `Result` alias. Errors are precise enough
for a host tool to react programmatically (unknown provider, missing key,
missing model) and carry enough context (provider name, status, url) for a
useful message, without ever leaking an API key.

## Public API

### Structs & Enums

| Type | Description |
|------|-------------|
| `Error` | Typed error enum (via `thiserror`) for resolution and request failures |

### Type Aliases

| Alias | Description |
|-------|-------------|
| `Result<T>` | `std::result::Result<T, Error>` |

### Error Variants

| Variant | Fields | Meaning |
|---------|--------|---------|
| `UnknownProvider` | `name`, `known` | Provider name is not in the registry |
| `MissingApiKey` | `provider`, `env_var` | A key-requiring provider has no key |
| `MissingModel` | `provider` | No model override and no registry default |
| `Status` | `provider`, `status`, `url` | Endpoint returned a non-2xx status |
| `Transport` | `url`, `message` | Connection/timeout/TLS/DNS failure (message redacted) |
| `Decode` | `url`, `message` | Body was not valid JSON of the expected shape |
| `Empty` | `url` | Response decoded but contained no text |

## Invariants

1. `Error` implements `std::error::Error` and `Display` (via `thiserror`).
2. No variant's `Display` output contains an API key; keys ride in request headers and `Transport` messages are passed through redaction.
3. `Status` carries the numeric HTTP status; `Transport`/`Decode`/`Empty` carry the request URL for context.

## Behavioral Examples

```
Given a resolve() call for an unregistered provider "foo"
When it fails
Then it returns Error::UnknownProvider { name: "foo", known: "<list>" }
```

```
Given a transport failure whose message echoes "Bearer sk-secret"
When the error is formatted
Then the rendered message shows "Bearer [REDACTED]"
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| (this module defines the errors) | n/a | Other modules construct and return these variants |

## Dependencies

- `thiserror` (third-party)

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-06-07 | Initial spec: seven typed variants, key-safe Display |
