---
title: backend.rs
path: kei-git-gitlab/src/backend.rs
dna_hash: sha256:84588c3fcb879d6b
language: rust
size_loc: 196
generated: by-keidocs
---

# kei-git-gitlab/src/backend.rs

`GitlabBackend` — `GitBackend` trait impl over GitLab REST v4 + git CLI.
API: existence / auto-create / branch-SHA. Heavy ops (clone/push/mirror)
shell out to system `git`. Auth: PRIVATE-TOKEN for API; git CLI uses URL
credentials or the user's git credential helper.

## Public API

- `pub fn from_env` — Convenience: build from `GITLAB_URL` + `GITLAB_TOKEN`.

## Related

- parent: `kei-git-gitlab/Cargo.toml`
- imports: crate, kei_runtime_core, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
