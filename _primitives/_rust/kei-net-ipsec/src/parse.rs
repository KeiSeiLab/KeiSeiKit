// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Parser for `swanctl --list-sas` text output.
//!
//! ## Grammar (informal)
//!
//! Each Security Association occupies a stanza beginning with a header:
//!
//! ```text
//! <conn>: #<id>, ESTABLISHED, <proto>, <spi-stuff>...
//!   local  '<id>' @ <local_ip>[port]
//!   remote '<id>' @ <remote_ip>[port]
//!   <encr_alg=...>
//!   <child_sa_lines>
//!     bytes_i (... , N bytes), packets_i (M packets)
//!     bytes_o (... , N bytes), packets_o (M packets)
//! ```
//!
//! We accept any whitespace lead-in. We only emit a [`PeerStatus`] for
//! stanzas that contain the literal token `ESTABLISHED` — `CONNECTING`,
//! `INSTALLED`, `REKEYING`, etc. are skipped (per spec: "ignore partial
//! SAs").
//!
//! `last_seen_ms`: strongSwan does not surface a clean per-SA last-handshake
//! timestamp in `--list-sas`, so we set it to **the current wall-clock**
//! when the SA reports `ESTABLISHED`, and `0` when the SA is partial /
//! ignored. Documented behaviour, not invented data.
//!
//! Bytes parsing tolerates the human-friendly formatter: strongSwan prints
//! `bytes_i (1.04K, 1067 bytes)`. We pull the integer that immediately
//! precedes the literal `bytes` token; suffix forms like `1.04K` are
//! ignored in favour of the exact byte count.

use kei_runtime_core::traits::network::PeerStatus;
use std::time::{SystemTime, UNIX_EPOCH};

const ESTABLISHED: &str = "ESTABLISHED";

/// Parse the full `swanctl --list-sas` stdout into one [`PeerStatus`] per
/// ESTABLISHED SA. Partial SAs (CONNECTING / REKEYING / etc.) are skipped.
pub fn parse_sas_output(s: &str) -> Vec<PeerStatus> {
    let now_ms = current_ms();
    let lines: Vec<&str> = s.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(end) = stanza_extent(&lines, i) {
            let stanza = &lines[i..end];
            if stanza_is_established(stanza) {
                if let Some(p) = stanza_to_peer(stanza, now_ms) {
                    out.push(p);
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// A new stanza starts at any non-indented line containing `: #` (the
/// `<conn>: #<id>` header). Returns `Some(end_exclusive)` if `idx` is such a
/// header; `None` otherwise.
fn stanza_extent(lines: &[&str], idx: usize) -> Option<usize> {
    let head = lines[idx];
    if !is_stanza_header(head) {
        return None;
    }
    let mut j = idx + 1;
    while j < lines.len() && !is_stanza_header(lines[j]) {
        j += 1;
    }
    Some(j)
}

fn is_stanza_header(line: &str) -> bool {
    // Header starts at column 0 (no leading whitespace) and contains ": #".
    if line.starts_with(' ') || line.starts_with('\t') {
        return false;
    }
    line.contains(": #")
}

fn stanza_is_established(stanza: &[&str]) -> bool {
    stanza.iter().any(|l| l.contains(ESTABLISHED))
}

fn stanza_to_peer(stanza: &[&str], now_ms: i64) -> Option<PeerStatus> {
    let remote = stanza.iter().find_map(|l| extract_remote_ip(l))?;
    let bytes_rx = stanza.iter().find_map(|l| extract_bytes_field(l, "bytes_i")).unwrap_or(0);
    let bytes_tx = stanza.iter().find_map(|l| extract_bytes_field(l, "bytes_o")).unwrap_or(0);
    Some(PeerStatus { addr: remote, last_seen_ms: now_ms, bytes_rx, bytes_tx })
}

/// Extract `<remote_ip>` from a line shaped like
/// `  remote '<id>' @ 198.51.100.7[4500]`.
fn extract_remote_ip(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("remote ") {
        return None;
    }
    let after_at = trimmed.split(" @ ").nth(1)?.trim();
    // Strip any `[port]` suffix.
    let ip = after_at.split('[').next()?.trim();
    if ip.is_empty() {
        return None;
    }
    Some(ip.to_string())
}

/// Extract the integer byte count following the `<key>` token. Tolerant
/// of `bytes_i (1.04K, 1067 bytes)` and the bare `bytes_i=1067` form.
fn extract_bytes_field(line: &str, key: &str) -> Option<u64> {
    let pos = line.find(key)?;
    let tail = &line[pos + key.len()..];
    // Form A: `bytes_i (..., 1067 bytes)` — pick integer before "bytes".
    if let Some(b_pos) = tail.find("bytes") {
        let prefix = &tail[..b_pos];
        if let Some(n) = last_unsigned_in(prefix) {
            return Some(n);
        }
    }
    // Form B: `bytes_i=1067`.
    if let Some(eq) = tail.strip_prefix('=') {
        if let Some(n) = first_unsigned_in(eq) {
            return Some(n);
        }
    }
    None
}

fn last_unsigned_in(s: &str) -> Option<u64> {
    let mut buf = String::new();
    let mut last: Option<u64> = None;
    for c in s.chars() {
        if c.is_ascii_digit() {
            buf.push(c);
        } else if !buf.is_empty() {
            last = buf.parse().ok();
            buf.clear();
        }
    }
    if !buf.is_empty() {
        last = buf.parse().ok();
    }
    last
}

fn first_unsigned_in(s: &str) -> Option<u64> {
    let mut buf = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            buf.push(c);
        } else if !buf.is_empty() {
            break;
        }
    }
    buf.parse().ok()
}

fn current_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

// Unit tests for the parser live in `tests/parse_unit.rs` to keep
// this file under the Constructor Pattern 200-LOC threshold.
