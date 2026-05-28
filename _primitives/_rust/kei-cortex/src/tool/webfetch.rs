//! `webfetch` tool — URL ingestion with HTML→text stripping + SSRF guard.
//!
//! Composition: validate URL scheme → resolve host to IPs ONCE → reject
//! any private/loopback/link-local/CGNAT range → pin reqwest to the
//! resolved IP via `Client::resolve` so the connect uses the same
//! address the validator saw → check bounded LRU cache → GET with
//! redirects DISABLED and 30 s timeout → strip HTML → cache + return.
//!
//! SSRF protection (`ip_filter.rs`):
//!   - 127.0.0.0/8 loopback
//!   - 10/8, 172.16/12, 192.168/16 RFC1918
//!   - 169.254.0.0/16 link-local incl. AWS IMDS
//!   - 100.64.0.0/10 CGNAT (Tailscale)
//!   - ::1, fc00::/7, fe80::/10, 0.0.0.0/8, multicast
//!
//! SSRF bypass envs (opt-in, narrow → broad):
//!   - `KEI_WEBFETCH_ALLOW_TAILSCALE=1` — allow ONLY 100.64.0.0/10 CGNAT.
//!     IMDS, RFC1918, loopback remain blocked. Use this for Tailscale.
//!   - `KEI_WEBFETCH_ALLOW_RANGES=10.0.0.0/8,192.168.0.0/16` — allow
//!     specific CIDR ranges. Each call to an allowed range is logged.
//!   - `KEI_WEBFETCH_ALLOW_PRIVATE=1` — full bypass (every blocked range
//!     including IMDS). Emits a startup warning AND logs every call.
//!     Reserved for sealed lab / staging environments.
//!
//! Cache: bounded LRU at 256 entries. The previous unbounded HashMap
//! was a memory-exhaustion vector for long-running daemons.

use super::types::ToolError;
use super::webfetch_policy::is_allowed;
use tracing::{debug, warn};
use lru::LruCache;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroUsize;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use url::Url;

const FETCH_TIMEOUT: Duration = Duration::from_secs(30);
const CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const CACHE_CAPACITY: usize = 256;

/// Logs the full-bypass startup warning at most once per process.
static FULL_BYPASS_WARNING: std::sync::Once = std::sync::Once::new();

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
    let resolved = resolve_and_check(&parsed).await?;
    if let Some(hit) = cache_get(&input.url) {
        return Ok(hit);
    }
    let body = fetch(&input.url, &parsed, &resolved).await?;
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

/// Outcome of the single SSRF resolution: the host as parsed and ALL
/// `SocketAddr`s the host resolved to (each pre-validated). `fetch`
/// pins reqwest to these addresses so the wire-connect uses the same
/// IPs the validator inspected — closing the DNS-rebinding window.
struct Resolved {
    host: String,
    addrs: Vec<SocketAddr>,
}

/// One-shot DNS resolution + SSRF check. If `host` is an IP literal we
/// validate it directly without a DNS round-trip. The returned
/// `Resolved.addrs` is the EXACT list `fetch` will connect to.
///
/// Bypass precedence: `ALLOW_PRIVATE` > `ALLOW_RANGES` > `ALLOW_TAILSCALE`.
/// Tailscale only opens 100.64.0.0/10. IMDS / RFC1918 / loopback stay
/// blocked unless `ALLOW_PRIVATE=1` or covered by `ALLOW_RANGES`.
async fn resolve_and_check(parsed: &Url) -> Result<Resolved, ToolError> {
    let host = parsed
        .host_str()
        .ok_or_else(|| ToolError::InvalidInput("url has no host".into()))?;
    let port = parsed.port_or_known_default().unwrap_or(80);

    let full_bypass = std::env::var("KEI_WEBFETCH_ALLOW_PRIVATE").as_deref() == Ok("1");
    if full_bypass {
        FULL_BYPASS_WARNING.call_once(|| {
            warn!(
                "kei-cortex webfetch: WARNING KEI_WEBFETCH_ALLOW_PRIVATE=1 disables \
                 ALL SSRF guards (including IMDS 169.254.169.254). Restrict via \
                 KEI_WEBFETCH_ALLOW_TAILSCALE=1 or KEI_WEBFETCH_ALLOW_RANGES instead."
            );
        });
        debug!("kei-cortex webfetch: bypass call host={host} port={port}");
    }

    // If host is an IP literal, validate without DNS.
    if let Ok(ip) = host.parse::<IpAddr>() {
        if !is_allowed(ip, full_bypass) {
            return Err(ToolError::PathDenied(format!(
                "ssrf-blocked ip {ip} for host {host}"
            )));
        }
        return Ok(Resolved {
            host: host.to_string(),
            addrs: vec![SocketAddr::new(ip, port)],
        });
    }

    let addrs: Vec<SocketAddr> = tokio::net::lookup_host((host, port))
        .await
        .map_err(|e| ToolError::InvalidInput(format!("host lookup: {e}")))?
        .collect();
    if addrs.is_empty() {
        return Err(ToolError::InvalidInput(format!(
            "host lookup returned no addresses for {host}"
        )));
    }
    for sock in &addrs {
        if !is_allowed(sock.ip(), full_bypass) {
            return Err(ToolError::PathDenied(format!(
                "ssrf-blocked ip {} for host {host}",
                sock.ip()
            )));
        }
    }
    Ok(Resolved { host: host.to_string(), addrs })
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

/// Issue a GET with redirects disabled and a wall-clock cap, pinning
/// the connect to the IPs that already passed SSRF validation. Without
/// `resolve_to_addrs`, reqwest would do its own DNS lookup right before
/// the syscall — a TOCTOU window the attacker can exploit (DNS rebind
/// to 169.254.169.254 between our validate and the connect).
async fn fetch(url: &str, parsed: &Url, resolved: &Resolved) -> Result<String, ToolError> {
    let mut builder = reqwest::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none());
    // Pin the host -> SocketAddr mapping. resolve_to_addrs takes &[SocketAddr]
    // and bypasses reqwest's internal resolver for this host only.
    builder = builder.resolve_to_addrs(&resolved.host, &resolved.addrs);
    let _ = parsed; // Reserved for future per-host TLS pinning.
    let client = builder
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
        let res = resolve_and_check(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    #[tokio::test]
    async fn ssrf_blocks_aws_imds() {
        let url = Url::parse("http://169.254.169.254/latest/meta-data/").unwrap();
        let res = resolve_and_check(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    #[tokio::test]
    async fn ssrf_blocks_rfc1918_via_literal() {
        let url = Url::parse("http://10.0.0.5/x").unwrap();
        let res = resolve_and_check(&url).await;
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }

    // Policy-layer tests (Cidr, is_tailscale_cgnat, is_allowed, allow_tailscale,
    // full_bypass) moved to webfetch_policy.rs in v0.51.1.
}
