---
title: client.rs
path: kei-git-gitea/src/client.rs
dna_hash: sha256:2e8187bd6bea6db7
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-git-gitea/src/client.rs

Typed HTTP client for the Gitea `/api/v1` surface. Three calls are
exposed — repo existence probe, user-repo creation, branch SHA
lookup — which together cover what `GiteaBackend::ensure_repo`
needs. Authentication is a `Bearer <GITEA_TOKEN>` header on every
request. The client takes `base_url` + `token` explicitly so tests
can point it at a wiremock server.

## Public API

- Request body for `POST /api/v1/user/repos`. Field names match the
- Subset of Gitea's repository response we consume. Gitea returns
- Branch endpoint returns `{ commit: { id: "<sha>", ... } }`.
- `pub fn new` — Construct from explicit base URL + bearer token. Use this in
- `pub fn from_env` — Read `GITEA_URL` (default `https://gitea.com`) and `GITEA_TOKEN`.
- `GET /api/v1/repos/{owner}/{repo}` — `Ok(true)` on 200,
- `POST /api/v1/user/repos` — creates a repo owned by the
- `GET /api/v1/repos/{owner}/{repo}/branches/{branch}` — returns

## Related

- parent: `kei-git-gitea/Cargo.toml`
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
