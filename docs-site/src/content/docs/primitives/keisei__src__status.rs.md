---
title: status.rs
path: keisei/src/status.rs
dna_hash: sha256:06f312cff8dbf5f3
language: rust
size_loc: 107
generated: by-keidocs
---

# keisei/src/status.rs

`keisei status` implementation.

Constructor Pattern: single responsibility — read the
`attached.toml` SSoT (v1..v4), verify each brain + its mcp binary
still exists, print a human-readable summary with per-client health.

v0.22: marker is v4 (per-attachment brain fields) so status groups the
output by brain: one header per unique `brain_path`, then the list of
`(client, scope, config_path)` attached to it, then a health check.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, std

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
