---
title: wiremock.rs
path: kei-git-bitbucket/tests/wiremock.rs
dna_hash: sha256:dfe75af38b01fa83
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-git-bitbucket/tests/wiremock.rs

wiremock integration tests for BitbucketClient + BitbucketBackend.

Required surface (per Wave 5 spec):
- repo_exists 200
- repo_exists 404
- create_repo 200
- ensure_repo end-to-end (404 then POST)

## Related

- parent: `kei-git-bitbucket/tests`
- imports: kei_git_bitbucket, kei_runtime_core, wiremock

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
