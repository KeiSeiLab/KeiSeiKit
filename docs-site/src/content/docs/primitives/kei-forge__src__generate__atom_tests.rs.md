---
title: atom_tests.rs
path: kei-forge/src/generate/atom_tests.rs
dna_hash: sha256:82c5c0b019bf9912
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-forge/src/generate/atom_tests.rs

Integration-flavoured tests for the pure-Rust atom generator.

Uses `tempfile::TempDir` to stand up a miniature copy of the repo
layout and exercises `generate_atom` end-to-end without touching the
real filesystem. Kept in its own file so `generate.rs` stays within
the Constructor-Pattern 200-LOC cap.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: crate, std

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
