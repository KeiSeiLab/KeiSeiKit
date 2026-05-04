---
title: install.rs
path: kei-discover/src/install.rs
dna_hash: sha256:ae0b8b98cd1c903f
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-discover/src/install.rs

`mark_installed` — flip the `installed` flag from 0 to 1.

Metadata-only: does NOT fetch remote content. A future wave will
add a real fetch-and-verify path, but v0.30 ships the stub contract
so the CLI surface stabilises first.

Uses a direct UPDATE rather than `kei_entity_store::verbs::update`
to keep the transaction small and return a typed `NotFound` when
the id does not exist.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite, std

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
