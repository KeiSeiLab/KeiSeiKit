---
title: discover.rs
path: kei-migrate/src/discover.rs
dna_hash: sha256:d39de139fbd7bcaa
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-migrate/src/discover.rs

Filesystem migration discovery.

Convention: `migrations/<version>_<name>.sql` (up) and optional
`migrations/<version>_<name>.down.sql` (down). Version is a monotonic
integer, typically a UTC timestamp like `20260421120000`.

## Public API

- One discovered migration (up-side). `down_path` is `Some` iff the sibling file exists.
- `pub fn scan` — Read every `<version>_<name>.sql` file (ignoring `.down.sql`), sort by version ASC.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, sha2, std

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
