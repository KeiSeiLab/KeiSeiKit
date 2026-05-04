---
title: model.rs
path: kei-ping/src/model.rs
dna_hash: sha256:b15e3e4c3ff37164
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-ping/src/model.rs

Heartbeat record + query filter. One file, no dependencies on backends.

## Public API

- One agent's "I'm alive, doing X" record.
- Only return heartbeats newer than this many seconds (TTL filter).
- Only return heartbeats matching this phase prefix.
- Only return heartbeats with branch matching exactly.

## Related

- parent: `kei-ping/Cargo.toml`
- imports: serde

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
