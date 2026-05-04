---
title: cmd_down.rs
path: kei-migrate/src/cmd_down.rs
dna_hash: sha256:7635ad63aa64b955
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-migrate/src/cmd_down.rs

`kei-migrate down [n]` — revert the last N applied migrations.

Requires a sibling `<version>_<name>.down.sql` for each target. Missing
down-file = hard error — we don't guess reversals.

## Public API

- Revert the last `n` applied migrations in reverse order.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, crate, sqlx, std

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
