---
title: unused_deps.rs
path: kei-cleanup/src/scanners/unused_deps.rs
dna_hash: sha256:6f21a20bf780fbdb
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-cleanup/src/scanners/unused_deps.rs

Unused dependencies scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
Appendix A. For each workspace member, parse Cargo.toml
`[dependencies]`, then grep `src/` for `use <crate>` or `<crate>::`.
Flag any dep that is not referenced.

## Public API

- `pub fn scan` — Public scanner entry — see Appendix A row "unused_deps".
- Extract dep names + line numbers from `[dependencies]` (and `[dev-dependencies]`).
- Naive but workable: capture top-level idents in `use X::...;`,
- Convert `serde-json` style names to `serde_json` Rust ident form.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
