//! Bearer token + Origin validation for WebSocket upgrades.
//!
//! Token is loaded from `~/.keisei/cortex.token` (same file as kei-cortex).
//! Origin allowlist: localhost and 127.0.0.1 on any port, plus the literal
//! string "null" (used by some browsers for file:// origins).

use std::path::PathBuf;

/// Error returned when auth fails.
#[derive(Debug)]
pub enum AuthError {
    TokenLoad(String),
    BearerMissing,
    BearerInvalid,
    OriginForbidden,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenLoad(e) => write!(f, "token load: {e}"),
            Self::BearerMissing => write!(f, "Sec-WebSocket-Protocol bearer token missing"),
            Self::BearerInvalid => write!(f, "bearer token mismatch"),
            Self::OriginForbidden => write!(f, "Origin not in allowlist"),
        }
    }
}

/// Load the expected bearer token from `~/.keisei/cortex.token`.
pub fn load_expected_token() -> Result<String, AuthError> {
    let path = token_path();
    std::fs::read_to_string(&path)
        .map(|s| s.trim().to_string())
        .map_err(|e| AuthError::TokenLoad(format!("{}: {e}", path.display())))
}

fn token_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".keisei/cortex.token")
}

/// Extract the bearer token from `Sec-WebSocket-Protocol: bearer,<token>`.
pub fn extract_bearer(protocol_header: Option<&str>) -> Result<&str, AuthError> {
    let hdr = protocol_header.ok_or(AuthError::BearerMissing)?;
    for part in hdr.split(',') {
        let part = part.trim();
        if let Some(tok) = part.strip_prefix("bearer ") {
            return Ok(tok.trim());
        }
        // Also accept bare token after "bearer" as sole segment
        if part != "bearer" && !part.is_empty() {
            // Skip non-bearer segments
        }
    }
    // Try: "bearer,<token>" — token is second comma-segment
    let mut parts = hdr.splitn(2, ',');
    if parts.next().map(str::trim) == Some("bearer") {
        if let Some(tok) = parts.next() {
            let tok = tok.trim();
            if !tok.is_empty() {
                return Ok(tok);
            }
        }
    }
    Err(AuthError::BearerMissing)
}

/// Validate `Origin` is in the local allowlist.
/// Allows: `http://localhost:<port>`, `http://127.0.0.1:<port>`, `null`.
pub fn validate_origin(origin: Option<&str>) -> Result<(), AuthError> {
    let o = origin.ok_or(AuthError::OriginForbidden)?;
    if o == "null" {
        return Ok(());
    }
    if is_local_origin(o) {
        return Ok(());
    }
    Err(AuthError::OriginForbidden)
}

fn is_local_origin(o: &str) -> bool {
    let stripped = o
        .strip_prefix("http://localhost")
        .or_else(|| o.strip_prefix("http://127.0.0.1"));
    match stripped {
        None => false,
        Some("") => true,
        Some(rest) => rest.starts_with(':'),
    }
}

/// Constant-time comparison (length-gated xor fold).
pub fn tokens_match(expected: &str, got: &str) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    let exp = expected.to_ascii_lowercase();
    let got = got.to_ascii_lowercase();
    let mut diff: u8 = 0;
    for (a, b) in exp.bytes().zip(got.bytes()) {
        diff |= a ^ b;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_origins_accepted() {
        assert!(validate_origin(Some("http://localhost:8201")).is_ok());
        assert!(validate_origin(Some("http://127.0.0.1:8201")).is_ok());
        assert!(validate_origin(Some("null")).is_ok());
    }

    #[test]
    fn remote_origins_rejected() {
        assert!(validate_origin(Some("http://evil.com")).is_err());
        assert!(validate_origin(Some("https://localhost:8201")).is_err());
        assert!(validate_origin(None).is_err());
    }

    #[test]
    fn bearer_extracted() {
        assert_eq!(extract_bearer(Some("bearer,abc123")).unwrap(), "abc123");
    }

    #[test]
    fn bearer_missing_returns_err() {
        assert!(extract_bearer(None).is_err());
        assert!(extract_bearer(Some("other")).is_err());
    }

    #[test]
    fn tokens_match_works() {
        assert!(tokens_match("abc", "abc"));
        assert!(tokens_match("ABC", "abc"));
        assert!(!tokens_match("abc", "xyz"));
        assert!(!tokens_match("abc", "ab"));
    }
}
