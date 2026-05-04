---
title: sqlite_store.rs
path: kei-ping/src/sqlite_store.rs
dna_hash: sha256:bdd2f314e989edb3
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-ping/src/sqlite_store.rs

SQLite-backed PingStore. WAL + busy_timeout for concurrent windows.
1 row per agent_id; UPDATE on every heartbeat (idempotent).

## Related

- parent: `kei-ping/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

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
