//! # corvid-ai
//!
//! A tiny, **synchronous**, multi-provider LLM client for the CorvidLabs stack.
//! Three HTTP wire shapes cover the field:
//!
//! - **Anthropic** Messages API (`x-api-key`)
//! - **OpenAI-compatible** Chat Completions (`Authorization: Bearer`) — the
//!   workhorse: OpenAI, OpenRouter, Groq, DeepSeek, Mistral, xAI, Together,
//!   local servers, and Ollama
//! - **Google Gemini** `generateContent` (`x-goog-api-key`)
//!
//! Adding an OpenAI-compatible gateway is one [registry](crate::registry) row —
//! no new HTTP code. There is **no CLI shell-out and no async runtime**: this is
//! a leaf crate (`ureq` + `serde`) meant to be shared by sync tools like
//! `fledge` and `spec-sync`.
//!
//! ```no_run
//! use corvid_ai::{Settings, Completion};
//!
//! // Resolve a provider from settings (+ env), then run one completion.
//! let settings = Settings::provider("anthropic"); // ANTHROPIC_API_KEY from env
//! let answer = corvid_ai::complete(&settings, &Completion::new("Say hello."))?;
//! println!("{answer}");
//! # Ok::<(), corvid_ai::Error>(())
//! ```

mod config;
pub mod error;
pub mod provider;
mod redact;
pub mod registry;

pub use config::{DEFAULT_PROVIDER, DEFAULT_TIMEOUT_SECS, Settings, resolve};
pub use error::{Error, Result};
pub use provider::{Completion, DEFAULT_MAX_TOKENS, Provider};
pub use registry::{Kind, ProviderSpec, REGISTRY, known_names};

/// Resolve [`Settings`] and run a single [`Completion`] in one call.
///
/// Equivalent to [`resolve`] followed by [`Provider::complete`].
pub fn complete(settings: &Settings, completion: &Completion) -> Result<String> {
    let (provider, timeout) = resolve(settings)?;
    provider.complete(completion, timeout)
}
