//! Built-in provider registry.
//!
//! Every entry maps a provider *name* to a wire protocol ([`Kind`]), a default
//! `base_url`, an optional default model, and the environment variable that
//! supplies its API key. Adding an OpenAI-compatible gateway is **one row** —
//! no new HTTP code.
//!
//! Model defaults are best-effort and drift over time; treat them as sane
//! starting points, not guarantees. Override with an explicit model.

/// The HTTP wire protocol a provider speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Anthropic Messages API — `x-api-key`, `POST {base_url}/v1/messages`.
    Anthropic,
    /// OpenAI-compatible Chat Completions — `Authorization: Bearer`,
    /// `POST {base_url}/chat/completions`. The workhorse: OpenAI, OpenRouter,
    /// Groq, DeepSeek, Mistral, xAI, Together, local servers, Ollama.
    OpenAiCompatible,
    /// Google Gemini — `x-goog-api-key`,
    /// `POST {base_url}/models/{model}:generateContent`.
    Gemini,
}

/// A built-in provider definition.
#[derive(Debug, Clone, Copy)]
pub struct ProviderSpec {
    /// Lowercase provider name (the value users put in config / `--provider`).
    pub name: &'static str,
    /// Which wire protocol this provider speaks.
    pub kind: Kind,
    /// Default endpoint base URL (no trailing slash).
    pub base_url: &'static str,
    /// Default model, or `None` when the user must pick one explicitly.
    pub default_model: Option<&'static str>,
    /// Environment variable that supplies the API key.
    pub env_var: &'static str,
}

/// The built-in providers. New OpenAI-compatible gateways are one row each.
pub const REGISTRY: &[ProviderSpec] = &[
    ProviderSpec {
        name: "anthropic",
        kind: Kind::Anthropic,
        base_url: "https://api.anthropic.com",
        default_model: Some("claude-sonnet-4-6"),
        env_var: "ANTHROPIC_API_KEY",
    },
    ProviderSpec {
        name: "openai",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.openai.com/v1",
        default_model: None,
        env_var: "OPENAI_API_KEY",
    },
    ProviderSpec {
        name: "openrouter",
        kind: Kind::OpenAiCompatible,
        base_url: "https://openrouter.ai/api/v1",
        default_model: None,
        env_var: "OPENROUTER_API_KEY",
    },
    ProviderSpec {
        name: "groq",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.groq.com/openai/v1",
        default_model: Some("llama-3.3-70b-versatile"),
        env_var: "GROQ_API_KEY",
    },
    ProviderSpec {
        name: "deepseek",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.deepseek.com/v1",
        default_model: Some("deepseek-chat"),
        env_var: "DEEPSEEK_API_KEY",
    },
    ProviderSpec {
        name: "mistral",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.mistral.ai/v1",
        default_model: Some("mistral-large-latest"),
        env_var: "MISTRAL_API_KEY",
    },
    ProviderSpec {
        name: "xai",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.x.ai/v1",
        default_model: Some("grok-3"),
        env_var: "XAI_API_KEY",
    },
    ProviderSpec {
        name: "together",
        kind: Kind::OpenAiCompatible,
        base_url: "https://api.together.xyz/v1",
        default_model: None,
        env_var: "TOGETHER_API_KEY",
    },
    ProviderSpec {
        // Ollama's OpenAI-compatible endpoint. Local by default; no key needed.
        name: "ollama",
        kind: Kind::OpenAiCompatible,
        base_url: "http://localhost:11434/v1",
        default_model: Some("llama3.3"),
        env_var: "OLLAMA_API_KEY",
    },
    ProviderSpec {
        name: "gemini",
        kind: Kind::Gemini,
        base_url: "https://generativelanguage.googleapis.com/v1beta",
        default_model: Some("gemini-2.5-flash"),
        env_var: "GEMINI_API_KEY",
    },
];

/// Look up a provider spec by (case-insensitive) name.
pub fn lookup(name: &str) -> Option<&'static ProviderSpec> {
    let lower = name.trim().to_ascii_lowercase();
    REGISTRY.iter().find(|s| s.name == lower)
}

/// Comma-separated list of all registered provider names (for error messages).
pub fn known_names() -> String {
    REGISTRY
        .iter()
        .map(|s| s.name)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_is_case_insensitive_and_trims() {
        assert_eq!(lookup("anthropic").unwrap().name, "anthropic");
        assert_eq!(lookup("  Anthropic ").unwrap().name, "anthropic");
        assert_eq!(lookup("OPENAI").unwrap().kind, Kind::OpenAiCompatible);
        assert!(lookup("nope").is_none());
    }

    #[test]
    fn known_names_lists_everything() {
        let names = known_names();
        assert!(names.contains("anthropic"));
        assert!(names.contains("openrouter"));
        assert!(names.contains("gemini"));
    }

    #[test]
    fn every_anthropic_and_gemini_has_a_default_model() {
        for s in REGISTRY {
            if matches!(s.kind, Kind::Anthropic | Kind::Gemini) {
                assert!(
                    s.default_model.is_some(),
                    "{} should have a default",
                    s.name
                );
            }
        }
    }
}
