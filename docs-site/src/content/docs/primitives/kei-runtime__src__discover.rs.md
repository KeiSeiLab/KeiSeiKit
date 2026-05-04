---
title: discover.rs
path: kei-runtime/src/discover.rs
dna_hash: sha256:9f3f334348f62523
language: rust
size_loc: 20
generated: by-keidocs
---

# kei-runtime/src/discover.rs

Atom discovery — thin façade over `kei-atom-discovery`.

Re-exports `AtomMeta` and `AtomKind` from the shared crate so all runtime
modules share exactly one frontmatter-parser implementation.

## Public API

- `pub fn walk_atoms` — Walk `<root>/*/atoms/*.md`. Delegates to `kei-atom-discovery::discover_atoms`.
- `pub fn extract_frontmatter` — Backwards-compatible split — returns the frontmatter YAML body (no body

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: kei_atom_discovery, std

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
