---
title: auth_test.rs
path: kei-cortex/src/auth_test.rs
dna_hash: sha256:6a483d6ac6140108
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-cortex/src/auth_test.rs

Inline unit tests for `auth.rs`.

Coverage:
- `tokens_match` is case-insensitive (MISS-6 fix).
- `validate_hex` accepts both cases, rejects bad length / non-hex.
- `generate_token` round-trips through validate + match.

## Related

- parent: `kei-cortex/Cargo.toml`

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
