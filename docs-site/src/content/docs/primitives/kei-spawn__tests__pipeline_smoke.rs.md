---
title: pipeline_smoke.rs
path: kei-spawn/tests/pipeline_smoke.rs
dna_hash: sha256:311a4b91df18bfe0
language: rust
size_loc: 179
generated: by-keidocs
---

# kei-spawn/tests/pipeline_smoke.rs

pipeline_smoke — integration tests for `spawn --pipeline` end-to-end.

Same pattern as spawn_smoke.rs: minimal tempdir kit, role + capability
fixtures, then call the library surface and assert on-disk artefacts.
`KEI_SPAWN_LEDGER_NOOP=1` keeps the ledger subprocess a no-op so tests
do not depend on a real kei-ledger binary.

## Related

- parent: `kei-spawn/tests`
- imports: kei_spawn, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
