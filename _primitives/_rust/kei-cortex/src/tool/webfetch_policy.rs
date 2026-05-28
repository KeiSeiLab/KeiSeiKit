//! SSRF allow/deny policy cube — pure decision logic, no I/O.
//!
//! v0.51.1 — extracted from `webfetch.rs` (was 448 LOC, Constructor Pattern
//! limit is 200). HTTP/cache/strip-html stay in `webfetch.rs`; this cube
//! owns the env-driven SSRF policy: `KEI_WEBFETCH_ALLOW_TAILSCALE`,
//! `KEI_WEBFETCH_ALLOW_RANGES`, `KEI_WEBFETCH_ALLOW_PRIVATE` and the CIDR
//! arithmetic that backs them.
//!
//! Precedence (matches `webfetch::resolve_and_check`):
//!     `ALLOW_PRIVATE` > `ALLOW_RANGES` > `ALLOW_TAILSCALE`
//!
//! `is_allowed(ip, full_bypass)` is the single entrypoint the HTTP path
//! calls; everything else here is implementation detail.

use super::ip_filter::is_blocked_ip;
use tracing::debug;
use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Parsed user-configured allow-list of CIDR ranges, from
/// `KEI_WEBFETCH_ALLOW_RANGES`. Computed once on first SSRF check.
static ALLOW_RANGES: Lazy<Vec<Cidr>> = Lazy::new(|| {
    let raw = std::env::var("KEI_WEBFETCH_ALLOW_RANGES").unwrap_or_default();
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| match Cidr::parse(s) {
            Some(c) => Some(c),
            None => {
                debug!("kei-cortex webfetch: ignoring invalid CIDR `{s}`");
                None
            }
        })
        .collect()
});

/// Decide whether `ip` passes the SSRF policy given current envs.
/// Returns `true` to allow the connection.
pub(crate) fn is_allowed(ip: IpAddr, full_bypass: bool) -> bool {
    if !is_blocked_ip(ip) {
        return true;
    }
    if full_bypass {
        return true;
    }
    // ALLOW_RANGES — explicit per-CIDR opt-in. Logged.
    for cidr in ALLOW_RANGES.iter() {
        if cidr.contains(ip) {
            debug!("kei-cortex webfetch: allow-range hit ip={ip} cidr={cidr}");
            return true;
        }
    }
    // ALLOW_TAILSCALE — narrow opt-in for 100.64.0.0/10 CGNAT only.
    if std::env::var("KEI_WEBFETCH_ALLOW_TAILSCALE").as_deref() == Ok("1")
        && is_tailscale_cgnat(ip)
    {
        return true;
    }
    false
}

/// True iff `ip` is in the 100.64.0.0/10 CGNAT block used by Tailscale.
/// Other blocked ranges (RFC1918, loopback, IMDS) are NOT opened by
/// this flag.
pub(crate) fn is_tailscale_cgnat(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            o[0] == 100 && (64..=127).contains(&o[1])
        }
        IpAddr::V6(_) => false,
    }
}

/// Minimal CIDR for IPv4 and IPv6. Parses `1.2.3.0/24` / `2001:db8::/32`.
#[derive(Clone)]
pub(crate) struct Cidr {
    base: IpAddr,
    prefix: u8,
}

impl Cidr {
    pub(crate) fn parse(s: &str) -> Option<Self> {
        let (addr, prefix) = s.split_once('/')?;
        let prefix: u8 = prefix.parse().ok()?;
        let base: IpAddr = addr.parse().ok()?;
        match base {
            IpAddr::V4(_) if prefix <= 32 => Some(Self { base, prefix }),
            IpAddr::V6(_) if prefix <= 128 => Some(Self { base, prefix }),
            _ => None,
        }
    }

    pub(crate) fn contains(&self, ip: IpAddr) -> bool {
        match (self.base, ip) {
            (IpAddr::V4(b), IpAddr::V4(i)) => v4_in(b, i, self.prefix),
            (IpAddr::V6(b), IpAddr::V6(i)) => v6_in(b, i, self.prefix),
            _ => false,
        }
    }
}

impl std::fmt::Display for Cidr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.prefix)
    }
}

fn v4_in(base: Ipv4Addr, ip: Ipv4Addr, prefix: u8) -> bool {
    if prefix == 0 {
        return true;
    }
    let mask: u32 = u32::MAX << (32 - prefix);
    let b = u32::from(base) & mask;
    let i = u32::from(ip) & mask;
    b == i
}

fn v6_in(base: Ipv6Addr, ip: Ipv6Addr, prefix: u8) -> bool {
    if prefix == 0 {
        return true;
    }
    let mask: u128 = u128::MAX << (128 - prefix);
    let b = u128::from(base) & mask;
    let i = u128::from(ip) & mask;
    b == i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tailscale_cgnat_detection() {
        assert!(is_tailscale_cgnat("100.64.0.1".parse().unwrap()));
        assert!(is_tailscale_cgnat("100.127.255.254".parse().unwrap()));
        assert!(!is_tailscale_cgnat("100.63.255.255".parse().unwrap()));
        assert!(!is_tailscale_cgnat("100.128.0.0".parse().unwrap()));
        assert!(!is_tailscale_cgnat("169.254.169.254".parse().unwrap()));
        assert!(!is_tailscale_cgnat("10.0.0.1".parse().unwrap()));
    }

    #[test]
    fn cidr_v4_parse_and_contains() {
        let c = Cidr::parse("10.0.0.0/8").unwrap();
        assert!(c.contains("10.0.0.1".parse().unwrap()));
        assert!(c.contains("10.255.255.255".parse().unwrap()));
        assert!(!c.contains("11.0.0.1".parse().unwrap()));
        assert!(!c.contains("::1".parse().unwrap()));
        let c = Cidr::parse("192.168.1.0/24").unwrap();
        assert!(c.contains("192.168.1.50".parse().unwrap()));
        assert!(!c.contains("192.168.2.50".parse().unwrap()));
    }

    #[test]
    fn cidr_v6_parse_and_contains() {
        let c = Cidr::parse("fc00::/7").unwrap();
        assert!(c.contains("fc00::1".parse().unwrap()));
        assert!(c.contains("fd00::1".parse().unwrap()));
        assert!(!c.contains("2001:db8::1".parse().unwrap()));
    }

    #[test]
    fn cidr_rejects_invalid() {
        assert!(Cidr::parse("not-a-cidr").is_none());
        assert!(Cidr::parse("10.0.0.0").is_none());
        assert!(Cidr::parse("10.0.0.0/33").is_none());
        assert!(Cidr::parse("fc00::/129").is_none());
    }

    /// `ALLOW_TAILSCALE` opens 100.64.0.0/10 but NOT IMDS / loopback /
    /// RFC1918. This is the whole reason for splitting the env: a user
    /// who wants Tailscale must NOT get the IMDS attack surface as a
    /// side effect. `ALLOW_RANGES` is process-static (`Lazy`), so test
    /// against `is_allowed` with `full_bypass=false`.
    #[test]
    fn allow_tailscale_does_not_open_imds() {
        std::env::set_var("KEI_WEBFETCH_ALLOW_TAILSCALE", "1");
        assert!(is_allowed("100.64.0.1".parse().unwrap(), false));
        assert!(!is_allowed("169.254.169.254".parse().unwrap(), false));
        assert!(!is_allowed("127.0.0.1".parse().unwrap(), false));
        assert!(!is_allowed("10.0.0.5".parse().unwrap(), false));
        assert!(!is_allowed("192.168.1.1".parse().unwrap(), false));
        std::env::remove_var("KEI_WEBFETCH_ALLOW_TAILSCALE");
    }

    #[test]
    fn full_bypass_allows_everything_blocked() {
        assert!(is_allowed("169.254.169.254".parse().unwrap(), true));
        assert!(is_allowed("127.0.0.1".parse().unwrap(), true));
        assert!(is_allowed("10.0.0.1".parse().unwrap(), true));
    }
}
