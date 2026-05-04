---
title: redis_store.rs
path: kei-ping/src/redis_store.rs
dna_hash: sha256:497d5d3d0026e042
language: rust
size_loc: 77
generated: by-keidocs
---

# kei-ping/src/redis_store.rs

Redis-backed PingStore. TTL keys (auto-expire 90s).
Schema: kei-ping:agent:<agent_id> → JSON Heartbeat (EX 90).

## Related

- parent: `kei-ping/Cargo.toml`
- imports: anyhow, crate, redis, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
