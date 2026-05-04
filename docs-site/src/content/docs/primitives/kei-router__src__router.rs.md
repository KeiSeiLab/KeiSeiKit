---
title: router.rs
path: kei-router/src/router.rs
dna_hash: sha256:7d6ce0da39d8dd82
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-router/src/router.rs

Router — holds keyword rules, dispatches queries to tool calls.

## Public API

- Canonical route outcome.
- `pub struct Router` — Router holds the static + dynamic keyword rules.
- `pub fn add_dynamic` — Append user-supplied rules at runtime (domain extension).
- `pub fn route` — Route a natural language query. Always returns a result — falls back to search tools.
- `pub fn route_with_hint` — Convenience wrapper — useful for remote MCP forwarders that want a hint.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate, serde, std

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
