---
module: config
version: 1
status: active
files:
  - src/config.rs

db_tables: []
depends_on:
  - registry
  - provider
  - error
---

# Config

## Purpose

Turns loose, user-supplied `Settings` into a concrete `Provider` plus its
request timeout. This is where the registry, environment variables, and
caller-supplied overrides come together under one precedence order. `Settings`
is serde-friendly so a host tool can deserialize it straight from its own
TOML/JSON config.

## Public API

### Exported Functions

| Export | Description |
|--------|-------------|
| `resolve` | Resolve `Settings` into a `(Provider, Duration)` |

### Structs & Enums

| Type | Description |
|------|-------------|
| `Settings` | Loose config: `provider`, `model`, `api_key`, `base_url`, `timeout_secs` (all optional) |

### Constants

| Const | Description |
|-------|-------------|
| `DEFAULT_PROVIDER` | Provider name used when `Settings::provider` is unset (`anthropic`) |
| `DEFAULT_TIMEOUT_SECS` | Default request timeout in seconds (600) |

### Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `Settings::provider` | `(impl Into<String>) -> Settings` | Start from a provider name |
| `Settings::model` | `(self, impl Into<String>) -> Settings` | Builder: set model |
| `Settings::api_key` | `(self, impl Into<String>) -> Settings` | Builder: set key |
| `Settings::base_url` | `(self, impl Into<String>) -> Settings` | Builder: set base URL |
| `resolve` | `(&Settings) -> Result<(Provider, Duration)>` | Apply precedence and build a provider |

## Invariants

1. Resolution order: provider name (default `anthropic`) to registry row; then model override, else the registry default; then `api_key`, else `<PROVIDER>_API_KEY` env; then `base_url` override, else registry default; then `timeout_secs`, else `DEFAULT_TIMEOUT_SECS`.
2. Empty strings for `model`, `api_key`, and `base_url` are treated as unset.
3. `Anthropic` and `Gemini` require a key; a missing key is `Error::MissingApiKey`.
4. `OpenAiCompatible` may resolve without a key (the provider stores `api_key: None`), so local servers and Ollama work.
5. A provider with no default model and no override is `Error::MissingModel`.
6. An unrecognized provider name is `Error::UnknownProvider`, listing the known names.

## Behavioral Examples

```
Given Settings::default() with ANTHROPIC_API_KEY set in the environment
When resolve is called
Then it returns Provider::Anthropic with model claude-sonnet-4-6 and a 600s timeout
```

```
Given Settings::provider("ollama") and no OLLAMA_API_KEY
When resolve is called
Then it returns Provider::OpenAiCompatible with api_key None
```

```
Given Settings::provider("openai") with a key but no model
When resolve is called
Then it returns Error::MissingModel (openai has no built-in default)
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| `Error::UnknownProvider` | Provider name not in the registry | Lists known names |
| `Error::MissingModel` | No model override and no registry default | Names the provider |
| `Error::MissingApiKey` | Anthropic/Gemini with no key in config or env | Names the provider and env var |

## Dependencies

- `registry`, `provider`, `error` (this crate)
- `serde` (third-party)

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-06-07 | Initial spec: precedence order and key requirements per protocol |
