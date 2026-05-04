---
title: client.rs
path: kei-git-gitlab/src/client.rs
dna_hash: sha256:abcfb2b2264d58ee
language: rust
size_loc: 198
generated: by-keidocs
---

# kei-git-gitlab/src/client.rs

Thin REST v4 client. PRIVATE-TOKEN header auth.
Project identity: numeric `project_id` OR url-encoded `namespace/name`.
We always url-encode so callers can pass either form transparently.

## Public API

- `pub fn with_url` — Construct from explicit base URL (used by wiremock tests + self-hosted).
- `pub fn from_env` — Construct from `GITLAB_URL` (default https://gitlab.com) + `GITLAB_TOKEN`.
- `path_with_namespace` is the "owner/repo" form (NOT url-encoded — we
- Create a private project under the authenticated user's namespace.
- Branch SHA. `id_or_path` accepts numeric `id` OR `owner/repo`.
- `pub fn parse_owner_repo` — Parse `owner/repo` from a remote URL. Accepts https://, http://, scp-style

## Related

- parent: `kei-git-gitlab/Cargo.toml`
- imports: crate, serde

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
