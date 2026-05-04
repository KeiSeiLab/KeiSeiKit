---
title: wiremock_api.rs
path: kei-git-gitlab/tests/wiremock_api.rs
dna_hash: sha256:16f1bc0943f78a50
language: rust
size_loc: 162
generated: by-keidocs
---

# kei-git-gitlab/tests/wiremock_api.rs

Integration tests for `GitlabClient` and `GitlabBackend::ensure_repo`
against a wiremock-served GitLab API. NO live HTTP.

## Public API

- End-to-end `ensure_repo`: project absent (404) → backend creates it (201).
- `ensure_repo` short-circuits when project already exists (no POST).

## Related

- parent: `kei-git-gitlab/tests`
- imports: kei_git_gitlab, kei_runtime_core, wiremock

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
