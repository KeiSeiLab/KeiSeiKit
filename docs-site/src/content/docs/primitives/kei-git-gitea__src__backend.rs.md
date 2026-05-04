---
title: backend.rs
path: kei-git-gitea/src/backend.rs
dna_hash: sha256:8e66736ca64d23f5
language: rust
size_loc: 144
generated: by-keidocs
---

# kei-git-gitea/src/backend.rs

`GiteaBackend` — `GitBackend` impl over [`GiteaClient`]. API calls
handle existence + creation; clone / push / mirror shell out to the
`git` CLI (no libgit2 dependency, mirrors `kei-git-keigit`).

## Public API

- `pub fn from_env` — Construct from `GITEA_URL` + `GITEA_TOKEN` env vars.
- Parse `https://gitea.example/<owner>/<repo>(.git)` → `(owner, repo)`.

## Related

- parent: `kei-git-gitea/Cargo.toml`
- imports: crate, kei_runtime_core, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
