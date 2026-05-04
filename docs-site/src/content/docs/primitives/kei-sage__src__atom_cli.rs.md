---
title: atom_cli.rs
path: kei-sage/src/atom_cli.rs
dna_hash: sha256:e71a31d84b867c1f
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-sage/src/atom_cli.rs

CLI handlers for `atoms-*` subcommands — walks, indexes, queries atoms.

Separate from `main.rs` to keep both files under Constructor Pattern
200-LOC limit. `main.rs` wires clap, this module implements the verbs.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate, std

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
