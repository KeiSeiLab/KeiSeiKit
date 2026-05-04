---
title: claims.rs
path: kei-auth-apple/src/claims.rs
dna_hash: sha256:c6e45758aaaa2876
language: rust
size_loc: 47
generated: by-keidocs
---

# kei-auth-apple/src/claims.rs

Apple id_token claim types deserialized from the JWT payload.

## Public API

- Subset of standard OIDC + Apple-specific claims we read.
- `aud` can be a single string or an array — Apple sends a single string.

## Related

- parent: `kei-auth-apple/Cargo.toml`
- imports: serde

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
