//! Typed errors for the corvid-ai client.

/// Errors returned when resolving a provider or running a completion.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The requested provider name is not in the [`crate::registry`].
    #[error("unknown provider {name:?}; known providers: {known}")]
    UnknownProvider {
        /// The name that was requested.
        name: String,
        /// Comma-separated list of registered provider names.
        known: String,
    },

    /// No API key was found for a provider that requires one (config or env var).
    #[error("missing API key for {provider:?}: set the {env_var} env var or configure an API key")]
    MissingApiKey {
        /// The provider name.
        provider: String,
        /// The environment variable that would supply the key.
        env_var: String,
    },

    /// The provider needs an explicit model and none was supplied (no built-in default).
    #[error("missing model for {provider:?}: set a model (this provider has no built-in default)")]
    MissingModel {
        /// The provider name.
        provider: String,
    },

    /// The endpoint returned a non-success HTTP status.
    #[error("{provider} endpoint returned HTTP {status} from {url}")]
    Status {
        /// The provider name.
        provider: String,
        /// The HTTP status code.
        status: u16,
        /// The request URL (never contains the API key — keys ride in headers).
        url: String,
    },

    /// Transport-level failure (connection refused, timeout, TLS, DNS, ...).
    #[error("request to {url} failed: {message}")]
    Transport {
        /// The request URL.
        url: String,
        /// A redacted description of the underlying failure.
        message: String,
    },

    /// The response body could not be decoded into the expected shape.
    #[error("could not decode response from {url}: {message}")]
    Decode {
        /// The request URL.
        url: String,
        /// The decode error detail.
        message: String,
    },

    /// The response decoded successfully but contained no text content.
    #[error("no text content in response from {url}")]
    Empty {
        /// The request URL.
        url: String,
    },
}

/// Convenience alias for results returned by this crate.
pub type Result<T> = std::result::Result<T, Error>;
