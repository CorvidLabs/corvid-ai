//! The three HTTP wire shapes and the [`Provider`] that drives them.
//!
//! A [`Provider`] is a fully-resolved target (key + model + base URL). Build one
//! with [`crate::resolve`] from loose [`crate::Settings`], then call
//! [`Provider::complete`]. All requests are synchronous (`ureq`); there is no
//! async runtime and no CLI shell-out.

use std::time::Duration;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::redact::redact;

const USER_AGENT: &str = concat!("corvid-ai/", env!("CARGO_PKG_VERSION"));

/// Default per-response token ceiling when a [`Completion`] doesn't set one.
pub const DEFAULT_MAX_TOKENS: u32 = 4096;

/// A single prompt to complete, plus optional system text and a token ceiling.
#[derive(Debug, Clone)]
pub struct Completion {
    /// Optional system / instruction text.
    pub system: Option<String>,
    /// The user prompt.
    pub prompt: String,
    /// Maximum tokens to generate.
    pub max_tokens: u32,
}

impl Completion {
    /// A completion for `prompt` with no system text and the default token cap.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            system: None,
            prompt: prompt.into(),
            max_tokens: DEFAULT_MAX_TOKENS,
        }
    }

    /// Set the system / instruction text (builder style).
    #[must_use]
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set the maximum tokens to generate (builder style).
    #[must_use]
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

/// A fully-resolved provider target. Construct via [`crate::resolve`].
#[derive(Debug, Clone)]
pub enum Provider {
    /// Anthropic Messages API.
    Anthropic {
        /// API key (`x-api-key`).
        api_key: String,
        /// Model id, e.g. `claude-sonnet-4-6`.
        model: String,
        /// Endpoint base URL.
        base_url: String,
    },
    /// OpenAI-compatible Chat Completions. Key is optional (local servers / Ollama).
    OpenAiCompatible {
        /// API key (`Authorization: Bearer`); `None`/empty omits the header.
        api_key: Option<String>,
        /// Model id.
        model: String,
        /// Endpoint base URL.
        base_url: String,
    },
    /// Google Gemini `generateContent`.
    Gemini {
        /// API key (`x-goog-api-key`).
        api_key: String,
        /// Model id, e.g. `gemini-2.5-flash`.
        model: String,
        /// Endpoint base URL.
        base_url: String,
    },
}

impl Provider {
    /// The model id this provider will use.
    pub fn model(&self) -> &str {
        match self {
            Provider::Anthropic { model, .. }
            | Provider::OpenAiCompatible { model, .. }
            | Provider::Gemini { model, .. } => model,
        }
    }

    /// A stable, lowercase name for the wire protocol (`anthropic` / `openai` / `gemini`).
    pub fn kind(&self) -> &'static str {
        match self {
            Provider::Anthropic { .. } => "anthropic",
            Provider::OpenAiCompatible { .. } => "openai",
            Provider::Gemini { .. } => "gemini",
        }
    }

    /// Send a single completion and return the concatenated text response.
    pub fn complete(&self, completion: &Completion, timeout: Duration) -> Result<String> {
        let agent = build_agent(timeout);
        match self {
            Provider::Anthropic {
                api_key,
                model,
                base_url,
            } => anthropic(&agent, api_key, model, base_url, completion),
            Provider::OpenAiCompatible {
                api_key,
                model,
                base_url,
            } => openai(&agent, api_key.as_deref(), model, base_url, completion),
            Provider::Gemini {
                api_key,
                model,
                base_url,
            } => gemini(&agent, api_key, model, base_url, completion),
        }
    }
}

fn build_agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .build()
        .into()
}

// ---- Anthropic ----------------------------------------------------------

/// Build the Anthropic Messages request body. Pure — unit tested without a network.
pub(crate) fn anthropic_body(model: &str, c: &Completion) -> serde_json::Value {
    let mut body = serde_json::json!({
        "model": model,
        "max_tokens": c.max_tokens,
        "messages": [{ "role": "user", "content": c.prompt }],
    });
    if let Some(system) = &c.system {
        body["system"] = serde_json::Value::String(system.clone());
    }
    body
}

#[derive(Deserialize)]
struct AnthropicResp {
    content: Vec<AnthropicBlock>,
}

#[derive(Deserialize)]
struct AnthropicBlock {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

/// Parse an Anthropic Messages response into concatenated text. Pure.
pub(crate) fn parse_anthropic(text: &str, url: &str) -> Result<String> {
    let parsed: AnthropicResp = serde_json::from_str(text).map_err(|e| Error::Decode {
        url: url.to_string(),
        message: e.to_string(),
    })?;
    let out: String = parsed
        .content
        .into_iter()
        .filter(|b| b.kind == "text")
        .filter_map(|b| b.text)
        .collect();
    finish(out, url)
}

fn anthropic(
    agent: &ureq::Agent,
    api_key: &str,
    model: &str,
    base_url: &str,
    c: &Completion,
) -> Result<String> {
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));
    let body = anthropic_body(model, c).to_string();
    let resp = agent
        .post(&url)
        .header("content-type", "application/json")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("user-agent", USER_AGENT)
        .send(body.as_bytes());
    let text = read(resp, "anthropic", &url)?;
    parse_anthropic(&text, &url)
}

// ---- OpenAI-compatible --------------------------------------------------

/// Build the OpenAI Chat Completions request body. Pure.
pub(crate) fn openai_body(model: &str, c: &Completion) -> serde_json::Value {
    let mut messages = Vec::new();
    if let Some(system) = &c.system {
        messages.push(serde_json::json!({ "role": "system", "content": system }));
    }
    messages.push(serde_json::json!({ "role": "user", "content": c.prompt }));
    serde_json::json!({
        "model": model,
        "messages": messages,
        "max_tokens": c.max_tokens,
    })
}

#[derive(Deserialize)]
struct OpenAiResp {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Deserialize)]
struct OpenAiMessage {
    content: Option<String>,
}

/// Parse an OpenAI Chat Completions response into text. Pure.
pub(crate) fn parse_openai(text: &str, url: &str) -> Result<String> {
    let parsed: OpenAiResp = serde_json::from_str(text).map_err(|e| Error::Decode {
        url: url.to_string(),
        message: e.to_string(),
    })?;
    let out = parsed
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .unwrap_or_default();
    finish(out, url)
}

fn openai(
    agent: &ureq::Agent,
    api_key: Option<&str>,
    model: &str,
    base_url: &str,
    c: &Completion,
) -> Result<String> {
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
    let body = openai_body(model, c).to_string();
    let mut req = agent
        .post(&url)
        .header("content-type", "application/json")
        .header("user-agent", USER_AGENT);
    if let Some(key) = api_key
        && !key.is_empty()
    {
        req = req.header("authorization", &format!("Bearer {key}"));
    }
    let text = read(req.send(body.as_bytes()), "openai", &url)?;
    parse_openai(&text, &url)
}

// ---- Gemini -------------------------------------------------------------

/// Build the Gemini `generateContent` request body. Pure.
pub(crate) fn gemini_body(c: &Completion) -> serde_json::Value {
    let mut body = serde_json::json!({
        "contents": [{ "parts": [{ "text": c.prompt }] }],
    });
    if let Some(system) = &c.system {
        body["systemInstruction"] = serde_json::json!({ "parts": [{ "text": system }] });
    }
    body
}

#[derive(Deserialize)]
struct GeminiResp {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    #[serde(default)]
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

/// Parse a Gemini `generateContent` response into concatenated text. Pure.
pub(crate) fn parse_gemini(text: &str, url: &str) -> Result<String> {
    let parsed: GeminiResp = serde_json::from_str(text).map_err(|e| Error::Decode {
        url: url.to_string(),
        message: e.to_string(),
    })?;
    let out: String = parsed
        .candidates
        .into_iter()
        .next()
        .map(|c| {
            c.content
                .parts
                .into_iter()
                .filter_map(|p| p.text)
                .collect::<String>()
        })
        .unwrap_or_default();
    finish(out, url)
}

fn gemini(
    agent: &ureq::Agent,
    api_key: &str,
    model: &str,
    base_url: &str,
    c: &Completion,
) -> Result<String> {
    let url = format!(
        "{}/models/{}:generateContent",
        base_url.trim_end_matches('/'),
        model
    );
    let body = gemini_body(c).to_string();
    let resp = agent
        .post(&url)
        .header("content-type", "application/json")
        .header("x-goog-api-key", api_key)
        .header("user-agent", USER_AGENT)
        .send(body.as_bytes());
    let text = read(resp, "gemini", &url)?;
    parse_gemini(&text, &url)
}

// ---- shared HTTP plumbing ----------------------------------------------

/// Turn a `ureq` send result into a response body string, mapping errors.
fn read(
    result: std::result::Result<ureq::http::Response<ureq::Body>, ureq::Error>,
    provider: &str,
    url: &str,
) -> Result<String> {
    let mut resp = match result {
        Ok(resp) => resp,
        Err(ureq::Error::StatusCode(status)) => {
            return Err(Error::Status {
                provider: provider.to_string(),
                status,
                url: url.to_string(),
            });
        }
        Err(e) => {
            return Err(Error::Transport {
                url: url.to_string(),
                message: redact(&e.to_string()),
            });
        }
    };
    resp.body_mut()
        .read_to_string()
        .map_err(|e| Error::Transport {
            url: url.to_string(),
            message: redact(&e.to_string()),
        })
}

/// Trim and reject empty completions.
fn finish(out: String, url: &str) -> Result<String> {
    let trimmed = out.trim();
    if trimmed.is_empty() {
        return Err(Error::Empty {
            url: url.to_string(),
        });
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Completion {
        Completion::new("hello").system("be terse").max_tokens(64)
    }

    #[test]
    fn anthropic_body_has_messages_and_system() {
        let body = anthropic_body("claude-sonnet-4-6", &sample());
        assert_eq!(body["model"], "claude-sonnet-4-6");
        assert_eq!(body["max_tokens"], 64);
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["messages"][0]["content"], "hello");
        assert_eq!(body["system"], "be terse");
    }

    #[test]
    fn openai_body_prepends_system_message() {
        let body = openai_body("gpt-x", &sample());
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][1]["role"], "user");
        assert_eq!(body["messages"][1]["content"], "hello");
    }

    #[test]
    fn openai_body_omits_system_when_absent() {
        let body = openai_body("gpt-x", &Completion::new("hi"));
        assert_eq!(body["messages"].as_array().unwrap().len(), 1);
        assert_eq!(body["messages"][0]["role"], "user");
    }

    #[test]
    fn gemini_body_uses_system_instruction() {
        let body = gemini_body(&sample());
        assert_eq!(body["contents"][0]["parts"][0]["text"], "hello");
        assert_eq!(body["systemInstruction"]["parts"][0]["text"], "be terse");
    }

    #[test]
    fn parse_anthropic_concatenates_text_blocks() {
        let json = r#"{"content":[{"type":"text","text":"foo "},{"type":"thinking","text":"x"},{"type":"text","text":"bar"}]}"#;
        assert_eq!(parse_anthropic(json, "u").unwrap(), "foo bar");
    }

    #[test]
    fn parse_openai_takes_first_choice() {
        let json = r#"{"choices":[{"message":{"role":"assistant","content":" hi there "}}]}"#;
        assert_eq!(parse_openai(json, "u").unwrap(), "hi there");
    }

    #[test]
    fn parse_gemini_concatenates_parts() {
        let json = r#"{"candidates":[{"content":{"parts":[{"text":"a"},{"text":"b"}]}}]}"#;
        assert_eq!(parse_gemini(json, "u").unwrap(), "ab");
    }

    #[test]
    fn empty_response_is_an_error() {
        let json = r#"{"content":[]}"#;
        assert!(matches!(
            parse_anthropic(json, "u"),
            Err(Error::Empty { .. })
        ));
    }

    #[test]
    fn malformed_json_is_a_decode_error() {
        assert!(matches!(
            parse_openai("not json", "u"),
            Err(Error::Decode { .. })
        ));
    }
}
