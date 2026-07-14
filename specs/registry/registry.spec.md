---
module: registry
version: 3
status: active
files:
  - src/registry.rs

db_tables: []
depends_on: []
---

# Registry

## Purpose

The built-in table of known providers. Each row maps a provider name to a wire
protocol (`Kind`), a default `base_url`, an optional default model, and the
environment variable that supplies its API key. Adding an OpenAI-compatible
gateway is one row, with no new HTTP code. Model defaults are best-effort and
drift over time, so they are documented as starting points rather than
guarantees.

## Public API

### Exported Functions

| Export | Description |
|--------|-------------|
| `lookup` | Find a `ProviderSpec` by case-insensitive, trimmed name |
| `known_names` | Comma-separated list of every registered name (for errors) |

### Exported Types

| Type | Description |
|------|-------------|
| `Kind` | Wire protocol: `Anthropic`, `OpenAiCompatible`, `Gemini` |
| `ProviderSpec` | One registry row: `name`, `kind`, `base_url`, `default_model`, `env_var` |

### Exported Constants

| Const | Description |
|-------|-------------|
| `REGISTRY` | The slice of built-in `ProviderSpec` rows |

## Invariants

1. Names are lowercase; `lookup` trims and lowercases its argument before matching.
2. Registered providers include `anthropic`, `openai`, `openrouter`, `groq`, `deepseek`, `mistral`, `xai`, `together`, `ollama`, and `gemini`.
3. Every `Anthropic` and `Gemini` row has a `default_model`; some `OpenAiCompatible` rows intentionally leave it `None` so the caller must choose.
4. `base_url` values carry no trailing slash.
5. Each row names the `<PROVIDER>_API_KEY` environment variable that supplies its key.

## Behavioral Examples

```
Given the name "  Anthropic "
When lookup is called
Then it returns the row whose name is "anthropic" (trimmed, case-insensitive)
```

```
Given the name "does-not-exist"
When lookup is called
Then it returns None and known_names lists the valid choices
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| (none) | The registry has no fallible operations | `lookup` returns `Option`; callers map a miss to `Error::UnknownProvider` |

## Dependencies

- None

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-06-07 | Initial spec: 10 built-in providers across three wire protocols |
| 2 | 2026-07-14 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-ai: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for corvid-ai |
| 3 | 2026-07-14 | CHG-0004-make-the-four-existing-corvid-ai-public-api-tables-parser-complete-without-chang: Make the four existing corvid-ai public API tables parser-complete without changing their contracts |
