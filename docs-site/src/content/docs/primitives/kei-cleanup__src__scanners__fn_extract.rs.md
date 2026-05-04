---
title: fn_extract.rs
path: kei-cleanup/src/scanners/fn_extract.rs
dna_hash: sha256:1b3cda8aea8a2a6b
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-cleanup/src/scanners/fn_extract.rs

Function-body extractor used by `loc_check`.

Manual brace-depth scan. We avoid `proc_macro2::Span::start()`
(feature-gated) so the scanner stays portable on stable.

## Public API

- `pub fn scan_fn_bodies` — Scan a Rust source for `fn <name>(` declarations and return for
- If `bytes[i..]` begins with `fn <ident>(`, return (ident, idx_after_ident).
- Returns Some(new_depth) when a `{` or `}` was consumed at top

## Related

- parent: `kei-cleanup/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
