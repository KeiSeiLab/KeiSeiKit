---
title: hmac.rs
path: kei-auth/src/hmac.rs
dna_hash: sha256:898261f91d2af934
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-auth/src/hmac.rs

HMAC-SHA256 signer for token bodies.

## Public API

- `pub fn sign` — Sign `body` with `key`. Returns URL-safe base64 MAC.
- `pub fn verify` — Verify `body` against MAC. Returns Err if mismatch.

## Related

- parent: `kei-auth/Cargo.toml`
- imports: anyhow, base64, sha2

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
