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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
