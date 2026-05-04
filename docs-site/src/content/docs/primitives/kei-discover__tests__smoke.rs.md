---
title: smoke.rs
path: kei-discover/tests/smoke.rs
dna_hash: sha256:cd287c3cb806b2c6
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-discover/tests/smoke.rs

Integration smoke tests for the kei-discover public API.

Covers the 6 behaviours enumerated in task.toml:
1. register returns id + increments count
2. list_available excludes installed
3. mark_installed flips flag
4. search matches slug and description via FTS
5. register rejects duplicate slug
6. stats counts

All tests use `open_memory()` so they neither touch nor contend with
the on-disk default DB path.

## Related

- parent: `kei-discover/tests`
- imports: kei_discover

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
