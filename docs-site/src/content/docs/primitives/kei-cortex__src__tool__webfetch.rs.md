---
title: webfetch.rs
path: kei-cortex/src/tool/webfetch.rs
dna_hash: sha256:213829be63f7ee8f
language: rust
size_loc: 211
generated: by-keidocs
---

# kei-cortex/src/tool/webfetch.rs

`webfetch` tool — URL ingestion with HTML→text stripping + SSRF guard.

Composition: validate URL scheme → resolve host to IPs → reject any
private/loopback/link-local/CGNAT range → check bounded LRU cache →
reqwest GET with redirects DISABLED (each hop re-validated by the
caller if needed) and 30 s timeout → strip HTML → cache + return.

SSRF protection (`ip_filter.rs`):
- 127.0.0.0/8 loopback
- 10/8, 172.16/12, 192.168/16 RFC1918
- 169.254.0.0/16 link-local incl. AWS IMDS
- 100.64.0.0/10 CGNAT (Tailscale)
- ::1, fc00::/7, fe80::/10, 0.0.0.0/8, multicast

Cache: bounded LRU at 256 entries. The previous unbounded HashMap
was a memory-exhaustion vector for long-running daemons.

## Public API

- Bounded LRU; size capped to prevent unbounded growth in long-lived
- Reject anything that isn't http(s).
- Resolve `parsed.host` to its IP addresses; refuse the call if ANY
- Lookup a fresh cache entry; expired entries are dropped.
- Insert; LRU evicts oldest at capacity automatically.
- Issue a GET with redirects disabled and a wall-clock cap.
- Strip script/style blocks, then all tags, then collapse whitespace.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: lru, once_cell, regex, serde, serde_json, std, url

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
