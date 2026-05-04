---
title: mgmt.rs
path: kei-net-openvpn/src/mgmt.rs
dna_hash: sha256:9064d7c522d184f4
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-net-openvpn/src/mgmt.rs

Pure parser for OpenVPN management-interface `status 2` output.

Wire format (CSV-ish, one row per connected client):
```text
CLIENT_LIST,common_name,real_address,virtual_address,virtual_ipv6,\
bytes_received,bytes_sent,connected_since,connected_since_t_t,\
username,client_id,peer_id,data_channel_cipher
```

We surface every CLIENT_LIST row as a [`ClientRow`] and then map
into [`PeerStatus`] in `network.rs`:
* `addr`         ← `virtual_address`
* `last_seen_ms` ← `connected_since_t_t * 1000`  (seconds → ms)
* `bytes_rx`     ← `bytes_received`
* `bytes_tx`     ← `bytes_sent`

The parser is permissive: rows that don't start with `CLIENT_LIST,`
are skipped silently (HEADER, TITLE, TIME, ROUTING_TABLE, GLOBAL_STATS,
END markers are not part of the peer projection).

## Public API

- Raw projection of a single CLIENT_LIST row.
- `pub fn parse_status_output` — Parse the full `status 2` text response. Non-CLIENT_LIST lines are

## Related

- parent: `kei-net-openvpn/Cargo.toml`
- imports: crate, kei_runtime_core

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
