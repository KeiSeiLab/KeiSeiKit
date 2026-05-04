//! HTTP status evidence with hardened SSRF guard.
//!
//! Security fix A (SSRF via DNS): the original implementation only blocked
//! IP literals in the URL. Hostnames like `evil.example` that resolve to
//! 169.254.169.254 (AWS metadata) bypassed the check entirely. The
//! resolver now expands every hostname to its `SocketAddr` set BEFORE the
//! request is dispatched and rejects on the first unsafe address.

use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use url::{Host, Url};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(2);
const TOTAL_TIMEOUT: Duration = Duration::from_secs(8);

/// Reject loopback / private / link-local / unspecified / multicast literal IPs.
fn ip_is_safe(ip: &IpAddr) -> bool {
    if ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() {
        return false;
    }
    match ip {
        IpAddr::V4(v4) => !v4.is_private() && !v4.is_link_local() && !v4.is_broadcast(),
        IpAddr::V6(v6) => {
            // Stable Rust lacks v6.is_unique_local()/is_unicast_link_local();
            // approximate via prefix bits. ULA = fc00::/7, link-local = fe80::/10.
            let segs = v6.segments();
            let is_ula = (segs[0] & 0xfe00) == 0xfc00;
            let is_link_local = (segs[0] & 0xffc0) == 0xfe80;
            !is_ula && !is_link_local
        }
    }
}

/// Validate URL scheme + host literal IPs. Hostnames pass through here
/// and are checked again post-DNS in `resolve_and_check`.
pub(super) fn validate_url(raw: &str) -> Result<Url, String> {
    let u = Url::parse(raw).map_err(|e| format!("invalid url `{}`: {}", raw, e))?;
    if u.scheme() != "http" && u.scheme() != "https" {
        return Err(format!("scheme `{}` not allowed (http/https only)", u.scheme()));
    }
    match u.host() {
        Some(Host::Ipv4(v4)) => {
            if !ip_is_safe(&IpAddr::V4(v4)) {
                return Err(format!("ipv4 {} blocked (private/loopback/link-local)", v4));
            }
        }
        Some(Host::Ipv6(v6)) => {
            if !ip_is_safe(&IpAddr::V6(v6)) {
                return Err(format!("ipv6 {} blocked (ula/link-local/loopback)", v6));
            }
        }
        Some(Host::Domain(_)) => {}
        None => return Err("url missing host".to_string()),
    }
    Ok(u)
}

/// Resolve `url`'s host via the OS resolver and verify EVERY returned
/// address is safe. Bypasses the literal-only guard in `validate_url`.
pub(super) fn resolve_and_check(url: &Url) -> Result<(), String> {
    let host = url.host_str().ok_or_else(|| "no host".to_string())?;
    // url::Host::Ipv{4,6} were already checked by `validate_url`; only
    // Domain reaches here in the live path. Resolve regardless to keep
    // the check uniform in case validate_url is bypassed in tests.
    let port = url.port_or_known_default().unwrap_or(443);
    let addrs = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("DNS resolve {}: {}", host, e))?;
    let mut count = 0u32;
    for addr in addrs {
        count += 1;
        if !ip_is_safe(&addr.ip()) {
            return Err(format!(
                "rejected by SSRF guard: {} -> {} blocked",
                host,
                addr.ip()
            ));
        }
    }
    if count == 0 {
        return Err(format!("DNS resolve {}: no addresses returned", host));
    }
    Ok(())
}

fn build_client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(TOTAL_TIMEOUT)
        .build()
        .map_err(|e| format!("http client build: {}", e))
}

pub fn check(url: &str, expected: &[u16]) -> (bool, String) {
    let parsed = match validate_url(url) {
        Ok(u) => u,
        Err(e) => return (false, e),
    };
    if let Err(e) = resolve_and_check(&parsed) {
        return (false, e);
    }
    let client = match build_client() {
        Ok(c) => c,
        Err(e) => return (false, e),
    };
    let resp = match client.get(parsed.as_str()).send() {
        Ok(r) => r,
        Err(e) => return (false, format!("GET {} failed: {}", url, e)),
    };
    let status = resp.status().as_u16();
    if expected.contains(&status) {
        (true, String::new())
    } else {
        (false, format!("status {} not in {:?}", status, expected))
    }
}
