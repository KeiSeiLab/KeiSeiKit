---
title: dna.rs
path: kei-runtime-core/src/dna.rs
dna_hash: sha256:c57b6ea7498dd67b
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-runtime-core/src/dna.rs

[`Dna`] — newtype wrapping the validated wire string from
`kei_shared::dna`. Construction goes through [`DnaBuilder`] which
computes scope_sha + body_sha deterministically and rolls a fresh
random nonce per call.

## Public API

- A validated DNA serial. Always parseable by [`parse_dna`].
- `pub fn parse` — Wrap an existing string. Errors if not parseable.
- `pub fn as_str` — Borrow the wire-format string.
- `pub fn parsed` — Re-parse into `ParsedDna` view (cheap; the wire format is the SSoT).
- `pub struct DnaBuilder` — Build a fresh DNA from semantic inputs.
- `pub trait HasDna` — Trait every registerable entity must implement.

## Related

- parent: `kei-runtime-core/Cargo.toml`
- imports: kei_shared, rand, serde, sha2

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
