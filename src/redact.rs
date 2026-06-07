//! Minimal secret redaction for error text.
//!
//! API keys ride in headers, not URLs or bodies we surface, so the blast radius
//! is small, but transport errors can occasionally echo a `Bearer <token>` or a
//! `?key=<token>` query param. This scrubs those without pulling in a regex dep.

/// Replace `Bearer <token>` and `key=<token>` occurrences with a placeholder.
pub fn redact(input: &str) -> String {
    let mut out = redact_marker(input, "Bearer ");
    out = redact_marker(&out, "bearer ");
    out = redact_marker(&out, "key=");
    out = redact_marker(&out, "api_key=");
    out
}

/// Redact the run of non-whitespace characters that follows `marker`.
fn redact_marker(input: &str, marker: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(pos) = rest.find(marker) {
        let (before, after) = rest.split_at(pos + marker.len());
        result.push_str(before);
        let token_end = after
            .find(|c: char| c.is_whitespace())
            .unwrap_or(after.len());
        if token_end > 0 {
            result.push_str("[REDACTED]");
        }
        rest = &after[token_end..];
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrubs_bearer_token() {
        let out = redact("auth failed: Authorization: Bearer sk-secret-123 nope");
        assert!(!out.contains("sk-secret-123"), "got: {out}");
        assert!(out.contains("Bearer [REDACTED]"), "got: {out}");
    }

    #[test]
    fn scrubs_key_query_param() {
        let out = redact("GET https://host/v1?key=AIzaTOPSECRET failed");
        assert!(!out.contains("AIzaTOPSECRET"), "got: {out}");
        assert!(out.contains("key=[REDACTED]"), "got: {out}");
    }

    #[test]
    fn passes_clean_text_through() {
        let clean = "connection refused (is the server running?)";
        assert_eq!(redact(clean), clean);
    }
}
