---
title: integration.rs
path: kei-projects-index/tests/integration.rs
dna_hash: sha256:6768a3fd62e9e231
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-projects-index/tests/integration.rs

Integration tests for kei-projects-index.

Constructor Pattern: each test = one scenario, one assertion target.
Uses `tempfile::tempdir` for per-test isolated working trees so the
tests are stable on a developer machine with a populated `~/Projects/`.

## Public API

- Init repo at `dir`, write README.md, stage + commit. Returns commit SHA.

## Related

- parent: `kei-projects-index/tests`
- imports: git2, kei_projects_index, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
