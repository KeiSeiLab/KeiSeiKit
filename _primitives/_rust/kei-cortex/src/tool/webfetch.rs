//! `webfetch` tool — URL ingestion with HTML→text stripping + SSRF guard.
//!
//! Composition: validate URL scheme → resolve host to IPs → reject any
//! private/loopback/link-local/CGNAT range → check bounded LRU cache →
//! reqwest GET with redirects DISABLED (each hop re-validated by the
//! caller if needed) and 30 s timeout → strip HTML → cache + return.
//!
//! SSRF protection (`ip_filter.rs`):
//!   - 127.0.0.0/8 loopback
//!   - 10/8, 172.16/12, 192.168/16 RFC1918
//!   - 169.254.0.0/16 link-local incl. AWS IMDS
//!   - 100.64.0.0/10 CGNAT (Tailscale)
//!   - ::1, fc00::/7, fe80::/10, 0.0.0.0/8, multicast
//!
//! Cache: bounded LRU at 256 entries. The previous unbounded HashMap
//! was a memory-exhaustion vector for long-running daemons.

use super::ip_filter::is_blocked_ip;
use super::types::ToolError;
use lru::LruCache;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use url::Url;

const FETCH_TIMEOUT: Duration = Duration::from_secs(30);
const CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const CACHE_CAPACITY: usize = 256;

/// Bounded LRU; size capped to prevent unbounded growth in long-lived
/// daemons. Entries past TTL are dropped on lookup.
static CACHE: Lazy<Mutex<LruCache<String, (Instant, String)>>> = Lazy::new(|| {
    Mutex::new(LruCache::new(NonZeroUsize::new(CACHE_CAPACITY).unwrap()))
});

#[derive(Debug, Deserialize)]
struct Input {
    url: String,
    #[serde(default)]
    #[allow(dead_code)]
    prompt: Option<String>,
}

pub async fn run(raw: Value) -> Result<String, ToolError> {
    let input: Input = serde_json::from_value(raw)
        .map_err(|e| ToolError::InvalidInput(e.to_string()))?;
    validate_url(&input.url)?;
    let parsed = Url::parse(&input.url)
        .map_err(|e| ToolError::InvalidInput(format!("url parse: {e}")))?;
    enforce_ssrf_guard(&parsed).await?;
    if let Some(hit) = cache_get(&input.url) {
        return Ok(hit);
    }
    let body = fetch(&input.url).await?;
    let text = strip_html(&body);
    cache_put(&input.url, &text);
    Ok(text)
}

/// Reject anything that isn't http(s).
pub(crate) fn validate_url(url: &str) -> Result<(), ToolError> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(ToolError::InvalidInput(format!(
            "url must be http(s): {url}"
        )));
    }
    Ok(())
}

/// Resolve `parsed.host` to its IP addresses; refuse the call if ANY
/// resolution falls in a blocked range. Honors
/// `KEI_WEBFETCH_ALLOW_PRIVATE=1` opt-in for Tailscale / lab networks.
async fn enforce_ssrf_guard(parsed: &Url) -> Result<(), ToolError> {
    if std::env::var("KEI_WEBFETCH_ALLOW_PRIVATE").as_deref() == Ok("1") {
        return Ok(());
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| ToolError::InvalidInput("url has no host".into()))?;
    let port = parsed.port_or_known_default().unwrap_or(80);
    let addrs = tokio::net::lookup_host((host, port))
        .await
        .map_err(|e| ToolError::InvalidInput(format!("host lookup: {e}")))?;
    for sock in addrs {
        if is_blocked_ip(sock.ip()) {
            return Err(ToolError::PathDenied(format!(
                "ssrf-blocked ip {} for host {host}",
                sock.ip()
            )));
        }
    }
    Ok(())
}

/// Lookup a fresh cache entry; expired entries are dropped.
fn cache_get(url: &str) -> Option<String> {
    let mut map = CACHE.lock().ok()?;
    let still_fresh = match map.get(url) {
        Some((when, _)) => when.elapsed() < CACHE_TTL,
        None => return None,
    };
    if still_fresh {
        // Borrow released by matches!; reacquire to clone the body.
        return map.get(url).map(|(_, text)| text.clone());
    }
    map.pop(url);
    None
}

/// Insert; LRU evicts oldest at capacity automatically.
fn cache_put(url: &str, text: &str) {
    if let Ok(mut map) = CACHE.lock() {
        map.put(url.to_string(), (Instant::now(), text.to_string()));
    }
}

/// Issue a GET with redirects disabled and a wall-clock cap.
async fn fetch(url: &str) -> Result<String, ToolError> {
    let client = reqwest::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| ToolError::Internal(e.to_string()))?;
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| ToolError::Internal(e.to_string()))?;
    let status = resp.status();
    if status.is_redirection() {
        return Err(ToolError::PathDenied(format!(
            "redirects disabled (got {status}); re-issue with the resolved URL"
        )));
    }
    if !status.is_success() {
        return Err(ToolError::Internal(format!("upstream {}", status)));
    }
    resp.text()
        .await
        .map_err(|e| ToolError::Internal(e.to_string()))
}

/// Strip script/style blocks, then all tags, then collapse whitespace.
pub(crate) fn strip_html(html: &str) -> String {
    static SCRIPT: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?is)<(script|style)[^>]*>.*?</\s*(script|style)\s*>").unwrap()
    });
    static TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());
    static WS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    let no_scripts = SCRIPT.replace_all(html, " ");
    let no_tags = TAG.replace_all(&no_scripts, " ");
    WS.replace_all(&no_tags, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_file_scheme() {
        assert!(matches!(
            validate_url("file:///etc/passwd"),
            Err(ToolError::InvalidInput(_))
        ));
    }

    #[test]
    fn accepts_https() {
        assert!(validate_url("https://example.com").is_ok());
    }

    #[test]
    fn strips_basic_html() {
        let h = "<p>hello <b>world</b></p>";
        assert_eq!(strip_html(h), "hello world");
    }

    #[test]
    fn strips_script_block() {
        let h = "<p>visible</p><script>var x = 1;</script><p>also</p>";
        let out = strip_html(h);
        assert!(out.contains("visible"));
        assert!(out.contains("also"));
        assert!(!out.contains("var x"));
    }

    #[tokio::test]
    async fn ssrf_blocks_localhost() {
        let url = Url::parse("http://127.0.0.1:8080/x").unwrap();
        let res = enforce_ssrf_guard(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    #[tokio::test]
    async fn ssrf_blocks_aws_imds() {
        let url = Url::parse("http://169.254.169.254/latest/meta-data/").unwrap();
        let res = enforce_ssrf_guard(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    #[tokio::test]
    async fn ssrf_blocks_rfc1918_via_literal() {
        let url = Url::parse("http://10.0.0.5/x").unwrap();
        let res = enforce_ssrf_guard(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }
}
