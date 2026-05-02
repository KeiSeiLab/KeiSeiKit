//! SSRF mitigation for fal.ai outbound requests.
//!
//! `validate_fal_url` rejects any URL whose scheme is not `https://` or
//! whose host does not match the fal.ai domain allowlist before the URL
//! is ever passed to `reqwest`.

use crate::fal::Error;
use once_cell::sync::Lazy;
use regex::Regex;

/// Compiled allowlist for fal.ai host names.
/// Accepted: `<one-or-more-subdomains>.fal.ai`, `.fal.media`, `.fal.run`.
/// Example: `rest.alpha.fal.ai`, `queue.fal.run`, `cdn.fal.media`.
static FAL_HOST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-z0-9-]+\.)+fal\.(ai|media|run)$").unwrap());

/// Reject URLs that do not match the fal.ai HTTPS allowlist.
///
/// Returns `Err(Error::NonFalUrl)` when:
/// - the scheme is not `https://`, or
/// - the host does not match `^[a-z0-9-]+\.fal\.(ai|media|run)$`.
pub(crate) fn validate_fal_url(url: &str) -> Result<(), Error> {
    let stripped = url.strip_prefix("https://").ok_or_else(|| {
        Error::NonFalUrl(url.chars().take(80).collect())
    })?;
    let host = stripped.split('/').next().unwrap_or("");
    if !FAL_HOST_RE.is_match(host) {
        return Err(Error::NonFalUrl(url.chars().take(80).collect()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_fal_urls() {
        assert!(validate_fal_url("https://rest.alpha.fal.ai/storage/upload/initiate").is_ok());
        assert!(validate_fal_url("https://queue.fal.run/fal-ai/flux-pro/v1.1-ultra").is_ok());
        assert!(validate_fal_url("https://cdn.fal.media/image.png").is_ok());
    }

    #[test]
    fn rejects_http_scheme() {
        assert!(matches!(
            validate_fal_url("http://queue.fal.run/foo"),
            Err(Error::NonFalUrl(_))
        ));
    }

    #[test]
    fn rejects_non_fal_domain() {
        assert!(matches!(
            validate_fal_url("https://evil.example.com/steal"),
            Err(Error::NonFalUrl(_))
        ));
    }

    #[test]
    fn rejects_ssrf_internal_ip() {
        assert!(matches!(
            validate_fal_url("https://169.254.169.254/latest/meta-data/"),
            Err(Error::NonFalUrl(_))
        ));
    }
}
