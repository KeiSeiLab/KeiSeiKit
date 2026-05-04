---
title: lib.rs
path: kei-git-gitlab/src/lib.rs
dna_hash: sha256:8338f64d623880bb
language: rust
size_loc: 16
generated: by-keidocs
---

# kei-git-gitlab/src/lib.rs

kei-git-gitlab — GitBackend impl for GitLab.com SaaS.

REST API v4 + PRIVATE-TOKEN header auth.
Project identity: url-encoded `namespace/name` (or numeric project_id).
`clone` / `push` shell out to the system `git` CLI; the API is only used
for existence + auto-create + branch-SHA lookups.

## Related

- parent: `kei-git-gitlab/Cargo.toml`

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
