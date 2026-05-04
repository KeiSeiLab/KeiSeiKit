---
title: firmware.rs
path: frustration-matrix/tests/firmware.rs
dna_hash: sha256:d7557947fe73fdb9
language: rust
size_loc: 124
generated: by-keidocs
---

# frustration-matrix/tests/firmware.rs

Firmware tests — cover training, save/load, multilingual alphabet,
unigram fallback, and size budget (≤50 KB at depth 4 on 1 MB corpus).

Like `tests/integration.rs`, we link source modules via `#[path]` so
the binary crate doesn't need to export a library surface.

## Related

- parent: `frustration-matrix/tests`
- imports: firmware, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
