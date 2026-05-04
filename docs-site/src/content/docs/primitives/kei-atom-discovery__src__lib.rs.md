---
title: lib.rs
path: kei-atom-discovery/src/lib.rs
dna_hash: sha256:83828a4ccce456c0
language: rust
size_loc: 26
generated: by-keidocs
---

# kei-atom-discovery/src/lib.rs

kei-atom-discovery — shared substrate-atom discovery primitives.

Single authoritative implementation of:
- `AtomMeta` / `AtomKind` / `SideEffect` — locked frontmatter schema
- `parse_frontmatter` — YAML split with 64 KiB cap (billion-laughs guard)
- `discover_atoms` — walks `<root>/*/atoms/*.md`, symlink-safe
- `parse_wikilink` — strict `[[target]]` matcher
- `safe_join` — path-traversal-safe base+rel join

Both `kei-sage` and `kei-runtime` consume this crate — no parallel
frontmatter structs, no parallel YAML parsers.

## Related

- parent: `kei-atom-discovery/Cargo.toml`

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
