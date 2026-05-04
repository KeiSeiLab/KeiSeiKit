---
title: config_migrate.rs
path: keisei/src/config_migrate.rs
dna_hash: sha256:7c42f34740628f31
language: rust
size_loc: 114
generated: by-keidocs
---

# keisei/src/config_migrate.rs

Schema-migration logic for the attach marker.

Constructor Pattern: single responsibility — own the `WireRecord`
enum and its v1/v2/v3 → v4 lift. Extracted from `config.rs` in v0.22
so `config.rs` stays under the 200-LOC ceiling.

Serde's `untagged` discrimination picks the first variant that
deserializes cleanly. Order: v4 first (strictest — carries
`schema_version` field), then v1/v2/v3 legacy shapes.

## Public API

- Best-effort classification of the legacy shape. v1 = flat

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, serde

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
