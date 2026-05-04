---
title: atom_index.rs
path: kei-sage/src/atom_index.rs
dna_hash: sha256:67ac488352a30f71
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-sage/src/atom_index.rs

Persist discovered atoms into the kei-sage Store as Units + typed edges.

Unit-type = `"atom"`; `vault_path` = atom full_id (e.g. `kei-task::create`).
Edge-type = `"atom_related"` for wikilinks between atoms. Idempotent:
re-ingesting the same corpus replaces existing rows by vault_path.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate

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
