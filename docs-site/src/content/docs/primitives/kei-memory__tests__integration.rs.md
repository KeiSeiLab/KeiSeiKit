---
title: integration.rs
path: kei-memory/tests/integration.rs
dna_hash: sha256:a175348a91168772
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-memory/tests/integration.rs

Integration tests for kei-memory.

Constructor Pattern: each test = one scenario, one assertion target.
Uses tempfile for per-test isolated sqlite file. Imports the
library crate directly (kei-memory now exposes [lib] + [bin]).

## Related

- parent: `kei-memory/tests`
- imports: kei_memory, rusqlite, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
