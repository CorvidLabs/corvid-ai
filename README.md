# corvid-ai

> Tiny, synchronous, multi-provider LLM client for the CorvidLabs stack.

A leaf crate (`ureq` + `serde`, no async runtime, no CLI shell-out) that talks to
any LLM API through three HTTP wire shapes:

- **Anthropic** Messages API (`x-api-key`)
- **OpenAI-compatible** Chat Completions (`Authorization: Bearer`) — the
  workhorse: OpenAI, OpenRouter, Groq, DeepSeek, Mistral, xAI, Together, local
  servers, and Ollama
- **Google Gemini** `generateContent` (`x-goog-api-key`)

Adding an OpenAI-compatible gateway is **one registry row** — no new HTTP code.

It is designed to be the shared provider layer for sync CorvidLabs tools
(`fledge`, `spec-sync`). The dependency arrow points **one way**: those tools
depend on `corvid-ai`; `corvid-ai` depends on nothing of ours.

## Usage

```rust
use corvid_ai::{Settings, Completion};

// Provider + key resolved from Settings and the environment.
let settings = Settings::provider("anthropic");        // ANTHROPIC_API_KEY from env
let answer = corvid_ai::complete(&settings, &Completion::new("Say hello."))?;

// Any OpenAI-compatible gateway is just a name + model (key from OPENROUTER_API_KEY):
let s = Settings::provider("openrouter").model("anthropic/claude-sonnet-4-6");

// Or point at a custom endpoint:
let s = Settings::provider("openai")
    .base_url("http://localhost:1234/v1")
    .model("local-model");
```

`Completion::new(prompt).system("…").max_tokens(8192)` builds a request; the
default `provider()` is `anthropic` and the default timeout is 600s.

## Resolution order

1. `provider` name → registry row (default: `anthropic`)
2. `model` override, else the registry default (error if neither)
3. `api_key`, else `<PROVIDER>_API_KEY` env var
4. `base_url` override, else the registry default
5. `timeout_secs`, else 600

Anthropic and Gemini require a key; OpenAI-compatible providers may run keyless
(local servers / Ollama).

## Development

This repo dogfoods the CorvidLabs toolchain (these are **dev tools**, not Cargo
dependencies):

```bash
fledge run test     # cargo test
fledge run lint     # cargo clippy --all-targets -- -D warnings
fledge run ci       # lint + fmt + test
spec-sync check     # specs are the source of truth
```

## License

MIT © CorvidLabs
