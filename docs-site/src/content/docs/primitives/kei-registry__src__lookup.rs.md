---
title: lookup.rs
path: kei-registry/src/lookup.rs
dna_hash: sha256:d2376bdcb726a2e4
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-registry/src/lookup.rs

Block lookup by id-or-DNA-or-path.

Constructor Pattern: this cube owns the polymorphic CLI lookup. Many
handlers accept "either an integer id or a full DNA string" — and a
few also support a filesystem path — so this helper centralises the
parse-cascade. Order: parse as i64 → DNA exact match → existing path.

## Public API

- `pub fn lookup_block` — Resolve a CLI target to a Block. Returns `None` if no row matches any

## Related

- parent: `kei-registry/Cargo.toml`
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
