---
title: client.rs
path: kei-git-forgejo/src/client.rs
dna_hash: sha256:dccd70318826ec09
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-git-forgejo/src/client.rs

Thin async REST client for a public Forgejo (Codeberg-style) instance.

Forgejo's HTTP surface is intentionally Gitea-compatible, so the same
`/api/v1` endpoints work against Forgejo, Gitea, and Codeberg without
special-casing. We hit only what `GitBackend::ensure_repo` needs:
`repo_exists`, `create_user_repo`, `get_default_branch_sha`.

## Public API

- Repo metadata as returned by `GET /api/v1/repos/{owner}/{name}`.
- Branch metadata as returned by `GET /api/v1/repos/{o}/{n}/branches/{br}`.
- Async REST client for a Forgejo/Codeberg/Gitea instance.
- `pub fn from_env` — Build from `FORGEJO_URL` (defaults to `https://codeberg.org`) and
- `pub fn with_url` — Explicit-URL constructor — used by `wiremock` tests and any caller
- `GET /api/v1/repos/{owner}/{name}` → `Ok(true)` on 200, `Ok(false)`
- `POST /api/v1/user/repos` — create under the authenticated user.
- `GET /api/v1/repos/{o}/{n}/branches/{br}` — used for sanity-checking
- `pub fn base_url` — Borrow the configured base URL (used by the backend to embed PAT
- `pub fn token` — Borrow the configured token (used by the backend to embed PAT into

## Related

- parent: `kei-git-forgejo/Cargo.toml`
- imports: crate, reqwest, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
