---
module: provider
version: 1
status: active
files:
  - src/provider.rs

db_tables: []
depends_on:
  - error
---

# Provider

## Purpose

Holds the three HTTP wire shapes the crate speaks and the `Provider` that drives
them. A `Provider` is a fully-resolved target (key, model, base URL); building
one is the job of `config::resolve`. Every request is synchronous (`ureq`) with
no async runtime and no CLI shell-out. Request-body construction and
response-parsing are split into pure functions so they can be unit tested
without a network.

## Public API

### Structs & Enums

| Type | Description |
|------|-------------|
| `Provider` | Resolved target. Variants: `Anthropic`, `OpenAiCompatible` (key optional), `Gemini` |
| `Completion` | A prompt plus optional system text and a `max_tokens` ceiling |

### Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `Completion::new` | `(impl Into<String>) -> Completion` | Build a completion with default `max_tokens` |
| `Completion::system` | `(self, impl Into<String>) -> Completion` | Builder: set system text |
| `Completion::max_tokens` | `(self, u32) -> Completion` | Builder: set the token ceiling |
| `Provider::complete` | `(&self, &Completion, Duration) -> Result<String>` | Send one completion, return concatenated text |
| `Provider::model` | `(&self) -> &str` | The model id this provider uses |
| `Provider::kind` | `(&self) -> &'static str` | Wire-protocol name: `anthropic`, `openai`, `gemini` |

### Constants

| Const | Description |
|-------|-------------|
| `DEFAULT_MAX_TOKENS` | Token ceiling used when a `Completion` does not set one (4096) |

## Invariants

1. Anthropic posts to `{base_url}/v1/messages` with `x-api-key` and `anthropic-version: 2023-06-01`; the text is the concatenation of `content[]` blocks whose `type == "text"`.
2. OpenAI-compatible posts to `{base_url}/chat/completions`; the `Authorization: Bearer` header is added only when a non-empty key is present (so local servers and Ollama work keyless). System text becomes a leading `system` message; the text is `choices[0].message.content`.
3. Gemini posts to `{base_url}/models/{model}:generateContent` with `x-goog-api-key`; the text is the concatenation of `candidates[0].content.parts[].text`. System text becomes `systemInstruction`.
4. A trailing slash on `base_url` is trimmed before the path is appended.
5. Body-building (`anthropic_body`, `openai_body`, `gemini_body`) and parsing (`parse_anthropic`, `parse_openai`, `parse_gemini`) are pure and network-free.
6. A response that decodes but yields only whitespace is an `Error::Empty`.
7. The request timeout is applied as `ureq`'s global timeout.

## Behavioral Examples

```
Given a resolved Provider::Anthropic and Completion::new("hi").system("be terse")
When complete() is called
Then a POST to {base_url}/v1/messages carries model, max_tokens, a user message, and a system field
And the returned String is the trimmed concatenation of text blocks
```

```
Given a resolved Provider::OpenAiCompatible with api_key = None (e.g. local Ollama)
When complete() is called
Then no Authorization header is sent and the request still succeeds against a keyless endpoint
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| `Error::Status` | Endpoint returns a non-2xx HTTP status | Carries provider, status, url |
| `Error::Transport` | Connection, timeout, TLS, or DNS failure | Message is redacted of any token |
| `Error::Decode` | Body is not valid JSON of the expected shape | Carries url and the decode detail |
| `Error::Empty` | Response decodes but has no text content | Carries url |

## Dependencies

- `error` (this crate)
- `ureq`, `serde`, `serde_json` (third-party)

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-06-07 | Initial spec: three wire shapes, pure body/parse split, keyless OpenAI-compatible support |
