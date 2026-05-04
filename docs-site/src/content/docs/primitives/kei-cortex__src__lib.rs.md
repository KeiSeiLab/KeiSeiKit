---
title: lib.rs
path: kei-cortex/src/lib.rs
dna_hash: sha256:5891280660da7d62
language: rust
size_loc: 41
generated: by-keidocs
---

# kei-cortex/src/lib.rs

kei-cortex — local HTTP daemon exposing cortex state for UI consumption.

Constructor Pattern: one module = one responsibility. This crate wires up:
`auth` (bearer-token lifecycle), `config` (CLI/env binding), `error`
(typed JSON responses), `state` (shared handler state), `routes` (router
+ middleware), `handlers` (endpoint implementations).

The daemon is intended to serve a single user on `127.0.0.1:9797` and
is fronted by a bearer token read from `~/.keisei/cortex.token`. CORS is
locked to a single origin provided at startup.

## Related

- parent: `kei-cortex/Cargo.toml`

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
