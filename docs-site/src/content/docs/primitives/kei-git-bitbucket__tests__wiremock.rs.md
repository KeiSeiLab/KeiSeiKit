---
title: wiremock.rs
path: kei-git-bitbucket/tests/wiremock.rs
dna_hash: sha256:dfe75af38b01fa83
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-git-bitbucket/tests/wiremock.rs

wiremock integration tests for BitbucketClient + BitbucketBackend.

Required surface (per Wave 5 spec):
- repo_exists 200
- repo_exists 404
- create_repo 200
- ensure_repo end-to-end (404 then POST)

## Related

- parent: `kei-git-bitbucket/tests`
- imports: kei_git_bitbucket, kei_runtime_core, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
