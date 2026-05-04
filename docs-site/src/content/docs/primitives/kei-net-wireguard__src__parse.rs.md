---
title: parse.rs
path: kei-net-wireguard/src/parse.rs
dna_hash: sha256:5e8637f4da6a4dde
language: rust
size_loc: 125
generated: by-keidocs
---

# kei-net-wireguard/src/parse.rs

Parser for `wg show <iface> dump` output.

Format (tab-separated, no header):
Line 1: interface row — `private_key  public_key  listen_port  fwmark`
Line 2+: peer rows —
`pubkey  preshared_key  endpoint  allowed_ips  latest_handshake_unix  rx_bytes  tx_bytes  persistent_keepalive`

We only project the columns required by `kei_runtime_core::PeerStatus`:
* col 3 endpoint            -> `addr`
* col 5 latest_handshake_s  -> `last_seen_ms` (= s * 1000; 0 stays 0)
* col 6 rx_bytes            -> `bytes_rx`
* col 7 tx_bytes            -> `bytes_tx`

Malformed numeric fields default to 0 rather than error: `wg` itself
emits `0` for "no handshake yet" / "no traffic", so a handshake of `0`
is normal and must round-trip cleanly. Anything else unparseable is
treated the same way (handshake/byte counters are advisory).

## Public API

- `pub fn parse_wg_dump` — Parse `wg show <iface> dump` stdout into a list of [`PeerStatus`].
- Empty input → empty Vec (no interface row, no peers).
- One interface row + one peer row → one PeerStatus, all fields
- Multiple peers parse independently and preserve order.
- `wg` emits `0` for "no handshake yet" — must round-trip to 0

## Related

- parent: `kei-net-wireguard/Cargo.toml`
- imports: kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
