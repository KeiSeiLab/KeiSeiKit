---
title: injection_check_binary.rs
path: kei-pet/src/injection_check_binary.rs
dna_hash: sha256:fa3d5a1e1e14f6c4
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-pet/src/injection_check_binary.rs

Binary / blob rules for `injection_check`.

Constructor Pattern: invisible-codepoint scan + base64-blob length
heuristic. No regex; per-char iteration only.

## Public API

- Detect invisible / bidi unicode codepoints anywhere in `content`.
- Detect a single line >= 1024 chars composed of base64 alphabet.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate

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
