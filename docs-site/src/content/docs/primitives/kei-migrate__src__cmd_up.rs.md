---
title: cmd_up.rs
path: kei-migrate/src/cmd_up.rs
dna_hash: sha256:cbb1a429de22c3d5
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-migrate/src/cmd_up.rs

`kei-migrate up` — apply all pending migrations in version-ASC order.

## Public API

- Apply every migration whose version is not in the applied set.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, chrono, crate, sqlx, std

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
