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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
