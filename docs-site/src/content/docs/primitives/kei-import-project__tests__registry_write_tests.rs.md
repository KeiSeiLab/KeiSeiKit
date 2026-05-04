---
title: registry_write_tests.rs
path: kei-import-project/tests/registry_write_tests.rs
dna_hash: sha256:b077a3384248685a
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-import-project/tests/registry_write_tests.rs

Integration tests for registry_writer: register, idempotency, supersede.

Uses tempfile for ephemeral SQLite + ephemeral repo trees. No live I/O.

## Public API

- Build a synthetic Rust mono-repo with 3 named crates.

## Related

- parent: `kei-import-project/tests`
- imports: kei_import_project, kei_registry, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
