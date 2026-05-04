---
title: fal_ssrf.rs
path: kei-cortex/src/fal_ssrf.rs
dna_hash: sha256:a0f053a222dd0bb1
language: rust
size_loc: 67
generated: by-keidocs
---

# kei-cortex/src/fal_ssrf.rs

SSRF mitigation for fal.ai outbound requests.

`validate_fal_url` rejects any URL whose scheme is not `https://` or
whose host does not match the fal.ai domain allowlist before the URL
is ever passed to `reqwest`.

## Public API

- Compiled allowlist for fal.ai host names.
- Reject URLs that do not match the fal.ai HTTPS allowlist.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, once_cell, regex

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
