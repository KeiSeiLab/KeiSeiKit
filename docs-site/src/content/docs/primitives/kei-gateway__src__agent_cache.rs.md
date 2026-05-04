---
title: agent_cache.rs
path: kei-gateway/src/agent_cache.rs
dna_hash: sha256:70fa8368a22b9390
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-gateway/src/agent_cache.rs

LRU-cached AIAgent handles, keyed by session.

Hermes pattern: each session_key owns a long-lived agent process. The cache
is bounded (memory pressure) and TTL-aware (idle eviction).

## Public API

- `pub struct CachedAgent` — Cached agent record. The `agent_handle` is intentionally type-erased — the
- Opaque handle the gateway forwards to the runner. The actual type is
- Hash / fingerprint of the (model, system prompt, toolset) tuple. Used
- Bounded LRU agent cache with idle TTL.
- Insert or replace an agent for `session_key`.
- Fetch a fresh-enough agent. Returns `None` if missing OR stale.
- Compare a stored agent's `config_signature` against `expected`. If they
- Drop every entry whose `last_used` exceeds `ttl`. Returns count purged.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: lru, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
