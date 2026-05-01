//! SSRF protection — IP-range deny-list.
//!
//! Composition: pure predicates over `IpAddr`. The webfetch tool
//! resolves the requested host, then refuses if ANY resolved IP is
//! private / loopback / link-local / CGNAT / IPv6 ULA.
//!
//! Why the host alone is insufficient: DNS rebinding attacks (a name
//! resolves to 169.254.169.254 the second time after the validator
//! checked it once) and CNAME chains require us to filter at IP level
//! immediately before the connect. Combine this with a single-shot
//! resolve + connect via `IpAddr` directly to eliminate the gap.
//!
//! AWS IMDS sits at `169.254.169.254/32` (link-local). Tailscale uses
//! CGNAT `100.64.0.0/10`; intentional reject — escape via opt-in env
//! `KEI_WEBFETCH_ALLOW_PRIVATE=1`.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// True iff `ip` falls in any blocked range.
pub fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_blocked_v4(v4),
        IpAddr::V6(v6) => is_blocked_v6(v6),
    }
}

/// IPv4 ranges blocked by SSRF policy.
fn is_blocked_v4(v4: Ipv4Addr) -> bool {
    let o = v4.octets();
    // 0.0.0.0/8 — "this network"
    if o[0] == 0 {
        return true;
    }
    // 10.0.0.0/8 — RFC1918
    if o[0] == 10 {
        return true;
    }
    // 127.0.0.0/8 — loopback
    if o[0] == 127 {
        return true;
    }
    // 169.254.0.0/16 — link-local incl. AWS IMDS
    if o[0] == 169 && o[1] == 254 {
        return true;
    }
    // 172.16.0.0/12 — RFC1918
    if o[0] == 172 && (16..=31).contains(&o[1]) {
        return true;
    }
    // 192.168.0.0/16 — RFC1918
    if o[0] == 192 && o[1] == 168 {
        return true;
    }
    // 100.64.0.0/10 — CGNAT (Tailscale lives here)
    if o[0] == 100 && (64..=127).contains(&o[1]) {
        return true;
    }
    // 224.0.0.0/4 — multicast (cannot be a meaningful HTTP target)
    if (224..=239).contains(&o[0]) {
        return true;
    }
    false
}

/// IPv6 ranges blocked by SSRF policy.
fn is_blocked_v6(v6: Ipv6Addr) -> bool {
    // ::1 — loopback
    if v6.is_loopback() {
        return true;
    }
    // :: — unspecified
    if v6.is_unspecified() {
        return true;
    }
    let segs = v6.segments();
    // fc00::/7 — Unique Local
    if segs[0] & 0xfe00 == 0xfc00 {
        return true;
    }
    // fe80::/10 — link-local
    if segs[0] & 0xffc0 == 0xfe80 {
        return true;
    }
    // ::ffff:0:0/96 — IPv4-mapped — re-check the embedded v4
    if segs[0] == 0 && segs[1] == 0 && segs[2] == 0 && segs[3] == 0
        && segs[4] == 0 && segs[5] == 0xffff
    {
        let v4 = Ipv4Addr::new(
            (segs[6] >> 8) as u8,
            (segs[6] & 0xff) as u8,
            (segs[7] >> 8) as u8,
            (segs[7] & 0xff) as u8,
        );
        return is_blocked_v4(v4);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn blocks_loopback_v4() {
        assert!(is_blocked_ip("127.0.0.1".parse().unwrap()));
    }

    #[test]
    fn blocks_aws_imds() {
        assert!(is_blocked_ip("169.254.169.254".parse().unwrap()));
    }

    #[test]
    fn blocks_rfc1918() {
        assert!(is_blocked_ip("10.0.0.5".parse().unwrap()));
        assert!(is_blocked_ip("172.16.0.1".parse().unwrap()));
        assert!(is_blocked_ip("172.31.255.255".parse().unwrap()));
        assert!(is_blocked_ip("192.168.1.1".parse().unwrap()));
    }

    #[test]
    fn allows_public_v4() {
        assert!(!is_blocked_ip("8.8.8.8".parse().unwrap()));
        assert!(!is_blocked_ip("1.1.1.1".parse().unwrap()));
        assert!(!is_blocked_ip("172.32.0.1".parse().unwrap()));
        assert!(!is_blocked_ip("172.15.255.255".parse().unwrap()));
    }

    #[test]
    fn blocks_v6_loopback() {
        assert!(is_blocked_ip("::1".parse().unwrap()));
    }

    #[test]
    fn blocks_v6_link_local() {
        assert!(is_blocked_ip("fe80::1".parse().unwrap()));
    }

    #[test]
    fn blocks_v6_ula() {
        assert!(is_blocked_ip("fc00::1".parse().unwrap()));
        assert!(is_blocked_ip("fd00::1".parse().unwrap()));
    }

    #[test]
    fn allows_public_v6() {
        let google = IpAddr::from_str("2001:4860:4860::8888").unwrap();
        assert!(!is_blocked_ip(google));
    }

    #[test]
    fn blocks_v4_mapped_loopback_v6() {
        assert!(is_blocked_ip("::ffff:127.0.0.1".parse().unwrap()));
    }

    #[test]
    fn blocks_cgnat() {
        assert!(is_blocked_ip("100.64.0.1".parse().unwrap()));
        assert!(is_blocked_ip("100.127.255.255".parse().unwrap()));
    }
}
