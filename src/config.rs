//! Turning loose [`Settings`] into a concrete [`Provider`].
//!
//! Resolution order (highest precedence first):
//! 1. explicit `provider` name → registry row (default: `anthropic`)
//! 2. `model` override, else the registry's default model (error if neither)
//! 3. `api_key`, else the provider's `<PROVIDER>_API_KEY` env var
//! 4. `base_url` override, else the registry default
//! 5. `timeout_secs`, else [`DEFAULT_TIMEOUT_SECS`]

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::registry::{Kind, known_names, lookup};

/// Default provider name when [`Settings::provider`] is unset.
pub const DEFAULT_PROVIDER: &str = "anthropic";

/// Default request timeout (seconds). Large local models can take minutes.
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;

/// Loose, user-supplied configuration. Every field is optional; sensible
/// defaults come from the [registry](crate::registry). Serde-friendly so callers
/// can deserialize it straight from their own TOML/JSON config.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    /// Provider name (registry key), e.g. `anthropic`, `openai`, `openrouter`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Model id override. Required for providers with no default model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// API key. Falls back to the provider's env var when unset/empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Endpoint base URL override (for self-hosted / proxy gateways).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Request timeout in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

impl Settings {
    /// Start from a provider name.
    pub fn provider(name: impl Into<String>) -> Self {
        Self {
            provider: Some(name.into()),
            ..Self::default()
        }
    }

    /// Set the model (builder style).
    #[must_use]
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the API key (builder style).
    #[must_use]
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the base URL (builder style).
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
}

/// Resolve [`Settings`] into a concrete [`Provider`] plus its request timeout.
///
/// Reads the provider's `<PROVIDER>_API_KEY` env var when no key is configured.
/// Anthropic and Gemini require a key; OpenAI-compatible providers may run
/// keyless (local servers, Ollama).
pub fn resolve(settings: &Settings) -> Result<(Provider, Duration)> {
    let name = settings.provider.as_deref().unwrap_or(DEFAULT_PROVIDER);
    let spec = lookup(name).ok_or_else(|| Error::UnknownProvider {
        name: name.to_string(),
        known: known_names(),
    })?;

    let model = settings
        .model
        .clone()
        .filter(|m| !m.is_empty())
        .or_else(|| spec.default_model.map(str::to_string))
        .ok_or_else(|| Error::MissingModel {
            provider: spec.name.to_string(),
        })?;

    let base_url = settings
        .base_url
        .clone()
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| spec.base_url.to_string());

    let key = settings
        .api_key
        .clone()
        .filter(|k| !k.is_empty())
        .or_else(|| std::env::var(spec.env_var).ok().filter(|k| !k.is_empty()));

    let timeout = Duration::from_secs(settings.timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS));

    let require_key = |provider: &str, env_var: &str| {
        key.clone().ok_or_else(|| Error::MissingApiKey {
            provider: provider.to_string(),
            env_var: env_var.to_string(),
        })
    };

    let provider = match spec.kind {
        Kind::Anthropic => Provider::Anthropic {
            api_key: require_key(spec.name, spec.env_var)?,
            model,
            base_url,
        },
        Kind::Gemini => Provider::Gemini {
            api_key: require_key(spec.name, spec.env_var)?,
            model,
            base_url,
        },
        Kind::OpenAiCompatible => Provider::OpenAiCompatible {
            api_key: key,
            model,
            base_url,
        },
    };

    Ok((provider, timeout))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Env vars are process-global; serialize the few tests that touch them.
    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn defaults_to_anthropic_with_registry_model() {
        let s = Settings::default().api_key("k");
        let (p, t) = resolve(&s).unwrap();
        assert_eq!(p.kind(), "anthropic");
        assert_eq!(p.model(), "claude-sonnet-4-6");
        assert_eq!(t, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }

    #[test]
    fn model_override_wins_over_default() {
        let s = Settings::provider("anthropic").api_key("k").model("opus-x");
        assert_eq!(resolve(&s).unwrap().0.model(), "opus-x");
    }

    #[test]
    fn unknown_provider_errors() {
        let s = Settings::provider("nope");
        assert!(matches!(resolve(&s), Err(Error::UnknownProvider { .. })));
    }

    #[test]
    fn openai_requires_an_explicit_model() {
        let _g = env_lock();
        // openai has no default model; with a key but no model → MissingModel.
        let s = Settings::provider("openai").api_key("k");
        assert!(matches!(resolve(&s), Err(Error::MissingModel { .. })));
        // With a model it resolves.
        let s = s.model("gpt-x");
        assert_eq!(resolve(&s).unwrap().0.kind(), "openai");
    }

    #[test]
    fn anthropic_without_key_errors() {
        let _g = env_lock();
        let prev = std::env::var("ANTHROPIC_API_KEY").ok();
        unsafe { std::env::remove_var("ANTHROPIC_API_KEY") };
        let s = Settings::provider("anthropic");
        let got = resolve(&s);
        if let Some(v) = prev {
            unsafe { std::env::set_var("ANTHROPIC_API_KEY", v) };
        }
        assert!(matches!(got, Err(Error::MissingApiKey { .. })));
    }

    #[test]
    fn openai_compatible_resolves_without_a_key() {
        let _g = env_lock();
        let prev = std::env::var("OLLAMA_API_KEY").ok();
        unsafe { std::env::remove_var("OLLAMA_API_KEY") };
        let s = Settings::provider("ollama");
        let got = resolve(&s);
        if let Some(v) = prev {
            unsafe { std::env::set_var("OLLAMA_API_KEY", v) };
        }
        let (p, _) = got.unwrap();
        match p {
            Provider::OpenAiCompatible { api_key, .. } => assert!(api_key.is_none()),
            other => panic!("expected OpenAiCompatible, got {other:?}"),
        }
    }

    #[test]
    fn env_var_supplies_the_key_when_config_is_empty() {
        let _g = env_lock();
        let prev = std::env::var("ANTHROPIC_API_KEY").ok();
        unsafe { std::env::set_var("ANTHROPIC_API_KEY", "from-env") };
        let got = resolve(&Settings::provider("anthropic"));
        match prev {
            Some(v) => unsafe { std::env::set_var("ANTHROPIC_API_KEY", v) },
            None => unsafe { std::env::remove_var("ANTHROPIC_API_KEY") },
        }
        match got.unwrap().0 {
            Provider::Anthropic { api_key, .. } => assert_eq!(api_key, "from-env"),
            other => panic!("expected Anthropic, got {other:?}"),
        }
    }

    #[test]
    fn base_url_override_is_respected() {
        let s = Settings::provider("openai")
            .api_key("k")
            .model("m")
            .base_url("https://example.test/v1");
        match resolve(&s).unwrap().0 {
            Provider::OpenAiCompatible { base_url, .. } => {
                assert_eq!(base_url, "https://example.test/v1");
            }
            other => panic!("got {other:?}"),
        }
    }
}
