// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Parser for `wg show <iface> dump` output.
//!
//! Format (tab-separated, no header):
//!   Line 1: interface row — `private_key  public_key  listen_port  fwmark`
//!   Line 2+: peer rows —
//!     `pubkey  preshared_key  endpoint  allowed_ips  latest_handshake_unix  rx_bytes  tx_bytes  persistent_keepalive`
//!
//! We only project the columns required by `kei_runtime_core::PeerStatus`:
//!   * col 3 endpoint            -> `addr`
//!   * col 5 latest_handshake_s  -> `last_seen_ms` (= s * 1000; 0 stays 0)
//!   * col 6 rx_bytes            -> `bytes_rx`
//!   * col 7 tx_bytes            -> `bytes_tx`
//!
//! Malformed numeric fields default to 0 rather than error: `wg` itself
//! emits `0` for "no handshake yet" / "no traffic", so a handshake of `0`
//! is normal and must round-trip cleanly. Anything else unparseable is
//! treated the same way (handshake/byte counters are advisory).

use kei_runtime_core::traits::network::PeerStatus;

/// Parse `wg show <iface> dump` stdout into a list of [`PeerStatus`].
///
/// The first non-empty line (interface row) is dropped; every subsequent
/// non-empty line is parsed as a peer. Lines with fewer than 8 columns are
/// skipped (defensive — older `wg` builds always emit 8).
pub fn parse_wg_dump(output: &str) -> Vec<PeerStatus> {
    let mut iter = output.lines().filter(|l| !l.trim().is_empty());

    // Drop the interface row.
    if iter.next().is_none() {
        return Vec::new();
    }

    iter.filter_map(parse_peer_line).collect()
}

fn parse_peer_line(line: &str) -> Option<PeerStatus> {
    let cols: Vec<&str> = line.split('\t').collect();
    if cols.len() < 8 {
        return None;
    }
    // wg show <iface> dump peer row columns:
    //   0 pubkey  1 preshared  2 endpoint  3 allowed_ips
    //   4 latest_handshake  5 rx_bytes  6 tx_bytes  7 persistent_keepalive
    let endpoint = cols[2].trim();
    let last_seen_unix: i64 = cols[4].trim().parse().unwrap_or(0);
    let rx: u64 = cols[5].trim().parse().unwrap_or(0);
    let tx: u64 = cols[6].trim().parse().unwrap_or(0);

    Some(PeerStatus {
        addr: endpoint.to_string(),
        last_seen_ms: last_seen_unix.saturating_mul(1000),
        bytes_rx: rx,
        bytes_tx: tx,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Empty input → empty Vec (no interface row, no peers).
    #[test]
    fn empty_input() {
        let peers = parse_wg_dump("");
        assert!(peers.is_empty(), "expected empty, got {peers:?}");
    }

    /// One interface row + one peer row → one PeerStatus, all fields
    /// projected.
    #[test]
    fn single_peer_full_fields() {
        // iface row (4 cols) + peer row (8 cols), tab-separated
        let dump = "PRIVKEYBASE64=\tIFACEPUBKEY=\t51820\toff\n\
                    PEERPUBKEY=\t(none)\t1.2.3.4:51820\t10.0.0.2/32\t1700000000\t12345\t67890\t25\n";
        let peers = parse_wg_dump(dump);
        assert_eq!(peers.len(), 1);
        let p = &peers[0];
        assert_eq!(p.addr, "1.2.3.4:51820");
        assert_eq!(p.last_seen_ms, 1_700_000_000_000);
        assert_eq!(p.bytes_rx, 12345);
        assert_eq!(p.bytes_tx, 67890);
    }

    /// Multiple peers parse independently and preserve order.
    #[test]
    fn multiple_peers() {
        let dump = "PRIVKEY=\tIPUB=\t51820\toff\n\
                    PEER1=\t(none)\t1.1.1.1:51820\t10.0.0.2/32\t1700000001\t10\t20\t0\n\
                    PEER2=\t(none)\t2.2.2.2:51820\t10.0.0.3/32\t1700000002\t30\t40\t25\n\
                    PEER3=\t(none)\t3.3.3.3:51820\t10.0.0.4/32\t1700000003\t50\t60\t0\n";
        let peers = parse_wg_dump(dump);
        assert_eq!(peers.len(), 3);
        assert_eq!(peers[0].addr, "1.1.1.1:51820");
        assert_eq!(peers[1].addr, "2.2.2.2:51820");
        assert_eq!(peers[2].addr, "3.3.3.3:51820");
        assert_eq!(peers[0].bytes_rx, 10);
        assert_eq!(peers[1].bytes_tx, 40);
        assert_eq!(peers[2].last_seen_ms, 1_700_000_003_000);
    }

    /// `wg` emits `0` for "no handshake yet" — must round-trip to 0
    /// (saturating_mul keeps it stable). Confirms the malformed-number
    /// branch as well by feeding `xxx`.
    #[test]
    fn malformed_handshake_zero() {
        let dump = "PRIV=\tPUB=\t51820\toff\n\
                    PEERA=\t(none)\t(none)\t10.0.0.2/32\t0\t0\t0\t0\n\
                    PEERB=\t(none)\t5.5.5.5:51820\t10.0.0.3/32\txxx\t100\t200\t0\n";
        let peers = parse_wg_dump(dump);
        assert_eq!(peers.len(), 2);
        // First peer: real "no handshake" zero.
        assert_eq!(peers[0].last_seen_ms, 0);
        assert_eq!(peers[0].bytes_rx, 0);
        assert_eq!(peers[0].bytes_tx, 0);
        // Second peer: unparseable handshake column -> 0; bytes still parse.
        assert_eq!(peers[1].last_seen_ms, 0);
        assert_eq!(peers[1].bytes_rx, 100);
        assert_eq!(peers[1].bytes_tx, 200);
        assert_eq!(peers[1].addr, "5.5.5.5:51820");
    }
}
