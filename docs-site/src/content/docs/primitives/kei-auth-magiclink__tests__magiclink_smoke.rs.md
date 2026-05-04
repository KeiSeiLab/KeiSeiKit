---
title: magiclink_smoke.rs
path: kei-auth-magiclink/tests/magiclink_smoke.rs
dna_hash: sha256:ca26dafa73fa7215
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-auth-magiclink/tests/magiclink_smoke.rs

Smoke tests for kei-auth-magiclink.

Covers the token codec only — provider tests live behind the same
`MagicLinkProvider::new` constructor and exercise the trait surface
via direct calls (not network).

## Related

- parent: `kei-auth-magiclink/tests`
- imports: kei_auth_magiclink, kei_runtime_core, std

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
