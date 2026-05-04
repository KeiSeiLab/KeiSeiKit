---
title: rules_rebuild.rs
path: kei-decompose/src/rules_rebuild.rs
dna_hash: sha256:886d35d6522b4ba2
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-decompose/src/rules_rebuild.rs

`decompose-rules --rebuild-fragments` one-time migration helper.

Re-extracts fragment bodies for all active Rule-type registry rows that
were registered with the old `"<file>::<section>"` logical-key format.
Writes each body to `<frags_dir>/<rule>__<section>.md` and updates the
`path` column in the registry to point at the real filesystem path.

Constructor Pattern: this cube owns the rebuild loop only.
Registry open + list lives in kei_registry. Fragment writing is reused
from `rules_cmd::write_fragment_file` and `rules_cmd::fragment_path`.

## Public API

- `pub fn run` — Re-extract all active Rule-type rows to canonical fragment files and update
- Rebuild one block from its legacy `"<file>::<section>"` path.
- Split `"<file_path>::<section>"` into its two components.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, kei_registry, std

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
