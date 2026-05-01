// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Pure parser for OpenVPN management-interface `status 2` output.
//!
//! Wire format (CSV-ish, one row per connected client):
//! ```text
//! CLIENT_LIST,common_name,real_address,virtual_address,virtual_ipv6,\
//!     bytes_received,bytes_sent,connected_since,connected_since_t_t,\
//!     username,client_id,peer_id,data_channel_cipher
//! ```
//!
//! We surface every CLIENT_LIST row as a [`ClientRow`] and then map
//! into [`PeerStatus`] in `network.rs`:
//!   * `addr`         ← `virtual_address`
//!   * `last_seen_ms` ← `connected_since_t_t * 1000`  (seconds → ms)
//!   * `bytes_rx`     ← `bytes_received`
//!   * `bytes_tx`     ← `bytes_sent`
//!
//! The parser is permissive: rows that don't start with `CLIENT_LIST,`
//! are skipped silently (HEADER, TITLE, TIME, ROUTING_TABLE, GLOBAL_STATS,
//! END markers are not part of the peer projection).

use crate::error::{Error, Result};
use kei_runtime_core::traits::network::PeerStatus;

/// Raw projection of a single CLIENT_LIST row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientRow {
    pub common_name: String,
    pub real_address: String,
    pub virtual_address: String,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub connected_since_t_t: i64,
}

impl ClientRow {
    pub fn into_peer(self) -> PeerStatus {
        PeerStatus {
            addr: self.virtual_address,
            last_seen_ms: self.connected_since_t_t.saturating_mul(1000),
            bytes_rx: self.bytes_received,
            bytes_tx: self.bytes_sent,
        }
    }
}

fn parse_row(line: &str) -> Result<ClientRow> {
    // CLIENT_LIST,common_name,real_address,virtual_address,virtual_ipv6,
    //   bytes_received,bytes_sent,connected_since,connected_since_t_t,
    //   username,client_id,peer_id,data_channel_cipher
    // We need fields [0..9). The trailing fields (username..cipher) may
    // be missing on older OpenVPN builds; we don't read them.
    let fields: Vec<&str> = line.split(',').collect();
    if fields.first().copied() != Some("CLIENT_LIST") {
        return Err(Error::Parse(format!("not a CLIENT_LIST row: {line}")));
    }
    if fields.len() < 9 {
        return Err(Error::Parse(format!(
            "CLIENT_LIST row has {} fields, need >=9: {line}",
            fields.len()
        )));
    }
    let bytes_received: u64 = fields[5]
        .parse()
        .map_err(|_| Error::Parse(format!("bytes_received not u64: {}", fields[5])))?;
    let bytes_sent: u64 = fields[6]
        .parse()
        .map_err(|_| Error::Parse(format!("bytes_sent not u64: {}", fields[6])))?;
    let connected_since_t_t: i64 = fields[8]
        .parse()
        .map_err(|_| Error::Parse(format!("connected_since_t_t not i64: {}", fields[8])))?;
    Ok(ClientRow {
        common_name: fields[1].to_string(),
        real_address: fields[2].to_string(),
        virtual_address: fields[3].to_string(),
        bytes_received,
        bytes_sent,
        connected_since_t_t,
    })
}

/// Parse the full `status 2` text response. Non-CLIENT_LIST lines are
/// skipped. Malformed CLIENT_LIST lines bubble up as `Error::Parse`.
pub fn parse_status_output(s: &str) -> Result<Vec<PeerStatus>> {
    let mut out = Vec::new();
    for line in s.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("CLIENT_LIST,") {
            continue;
        }
        let row = parse_row(trimmed)?;
        out.push(row.into_peer());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_produces_no_peers() {
        let v = parse_status_output("").expect("ok");
        assert!(v.is_empty());
    }

    #[test]
    fn header_only_no_clients() {
        let s = "TITLE,OpenVPN 2.5.5\nTIME,Mon Apr 28 00:00:00 2026,1745798400\n\
                 HEADER,CLIENT_LIST,Common Name,Real Address,Virtual Address,...\n\
                 GLOBAL_STATS,Max bcast/mcast queue length,0\nEND\n";
        let v = parse_status_output(s).expect("ok");
        assert!(v.is_empty(), "no CLIENT_LIST rows in header-only payload");
    }

    #[test]
    fn single_client_row_parses_to_peer_status() {
        let s = "TITLE,OpenVPN 2.5.5\n\
                 CLIENT_LIST,alice,203.0.113.7:49001,10.8.0.2,fe80::1,\
                 1024,2048,Mon Apr 28 00:00:00 2026,1745798400,UNDEF,3,1,AES-256-GCM\nEND\n";
        let v = parse_status_output(s).expect("ok");
        assert_eq!(v.len(), 1);
        let p = &v[0];
        assert_eq!(p.addr, "10.8.0.2");
        assert_eq!(p.bytes_rx, 1024);
        assert_eq!(p.bytes_tx, 2048);
        // 1745798400 sec → 1_745_798_400_000 ms
        assert_eq!(p.last_seen_ms, 1_745_798_400_000);
    }

    #[test]
    fn multiple_client_rows_preserve_order() {
        let s = "TITLE,OpenVPN 2.5.5\n\
CLIENT_LIST,alice,1.2.3.4:1,10.8.0.2,,100,200,Mon,1000,UNDEF,3,1,AES-256-GCM\n\
CLIENT_LIST,bob,5.6.7.8:2,10.8.0.3,,300,400,Mon,2000,UNDEF,4,2,AES-256-GCM\n\
END\n";
        let v = parse_status_output(s).expect("ok");
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].addr, "10.8.0.2");
        assert_eq!(v[1].addr, "10.8.0.3");
        assert_eq!(v[0].last_seen_ms, 1_000_000);
        assert_eq!(v[1].last_seen_ms, 2_000_000);
    }
}
