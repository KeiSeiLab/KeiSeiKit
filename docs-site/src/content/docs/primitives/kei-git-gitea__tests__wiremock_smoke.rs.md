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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
