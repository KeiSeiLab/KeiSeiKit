---
title: test_dep_drift.rs
path: kei-cleanup/tests/test_dep_drift.rs
dna_hash: sha256:f0a0b334a78ceff9
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cleanup/tests/test_dep_drift.rs

Integration test for the dep_drift scanner.

Builds a fixture workspace whose member crate pins `serde = "1.0.180"`
while `[workspace.dependencies]` declares `serde = "1.0.190"`.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
