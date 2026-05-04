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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
