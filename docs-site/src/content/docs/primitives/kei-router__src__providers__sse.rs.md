---
title: sse.rs
path: kei-router/src/providers/sse.rs
dna_hash: sha256:c16785df618424f5
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-router/src/providers/sse.rs

Minimal SSE frame parser, shared by all providers.

SSE frames are separated by `\n\n`. Each frame may have multiple lines;
we only care about the `data: ` line. Returns the JSON payload string per
frame (caller decides how to interpret it).

## Public API

- `pub fn push` — Push bytes; return any complete `data:` payloads (one per completed frame).

## Related

- parent: `kei-router/Cargo.toml`
- imports: bytes

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
