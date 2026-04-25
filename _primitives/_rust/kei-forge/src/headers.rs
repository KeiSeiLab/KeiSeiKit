//! Security headers applied to the GET / HTML response.
//!
//! Defence-in-depth layer on top of the Host allow-list and JSON
//! content-type enforcement: these directives limit the blast radius of
//! any reflected-XSS / iframe-embedding attempt against the wizard UI.
//!
//! - `Content-Security-Policy` — inline-script/style only from self, no
//!   external origins, `form-action 'self'` blocks cross-origin form
//!   posts even if the SOP layer is bypassed.
//! - `X-Content-Type-Options: nosniff` — browsers MUST NOT sniff MIME.
//! - `X-Frame-Options: DENY` — cannot be iframe-embedded (clickjacking).
//! - `Referrer-Policy: no-referrer` — don't leak the wizard URL.

use axum::http::{header, HeaderMap, HeaderValue};

/// Populate `headers` with the four security headers. Used by the GET /
/// handler to decorate its HTML response.
pub fn apply_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline'; \
             style-src 'self' 'unsafe-inline'; form-action 'self'; \
             frame-ancestors 'none'",
        ),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_all_four_headers() {
        let mut h = HeaderMap::new();
        apply_security_headers(&mut h);
        assert!(h.contains_key(header::CONTENT_SECURITY_POLICY));
        assert!(h.contains_key(header::X_CONTENT_TYPE_OPTIONS));
        assert!(h.contains_key(header::X_FRAME_OPTIONS));
        assert!(h.contains_key(header::REFERRER_POLICY));
    }

    #[test]
    fn csp_forbids_cross_origin_forms() {
        let mut h = HeaderMap::new();
        apply_security_headers(&mut h);
        let csp = h.get(header::CONTENT_SECURITY_POLICY).unwrap();
        assert!(csp.to_str().unwrap().contains("form-action 'self'"));
    }
}
