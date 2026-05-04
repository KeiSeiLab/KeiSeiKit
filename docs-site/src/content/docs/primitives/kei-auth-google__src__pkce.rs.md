---
title: pkce.rs
path: kei-auth-google/src/pkce.rs
dna_hash: sha256:a77f3c6ac4ecba68
language: rust
size_loc: 51
generated: by-keidocs
---

# kei-auth-google/src/pkce.rs

PKCE (RFC 7636) helpers and URL percent-encoder shared by the Google
auth provider.

## Public API

- `pub fn pkce_challenge` — Compute PKCE `code_challenge` = `BASE64URL-no-pad(SHA256(verifier))`.
- Percent-encode a string per RFC 3986 §2.1 (only unreserved chars pass).

## Related

- parent: `kei-auth-google/Cargo.toml`
- imports: base64, sha2

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
