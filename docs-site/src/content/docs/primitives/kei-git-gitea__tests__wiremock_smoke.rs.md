---
title: wiremock_smoke.rs
path: kei-git-gitea/tests/wiremock_smoke.rs
dna_hash: sha256:eaf769f1131ba061
language: rust
size_loc: 126
generated: by-keidocs
---

# kei-git-gitea/tests/wiremock_smoke.rs

Wiremock-only integration tests. No live HTTP, no `git` CLI calls.
Covers: repo_exists 200/404, create_user_repo 201, ensure_repo
end-to-end (404 → POST 201).

## Related

- parent: `kei-git-gitea/tests`
- imports: kei_git_gitea, kei_runtime_core, serde_json, wiremock

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
