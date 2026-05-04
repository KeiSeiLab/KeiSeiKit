---
title: test_loc_check.rs
path: kei-cleanup/tests/test_loc_check.rs
dna_hash: sha256:dacaa457dcfb46b7
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-cleanup/tests/test_loc_check.rs

Integration test for the loc_check scanner.

Builds a temporary workspace with one oversized file and verifies
the scanner emits a `LocFile` finding plus an over-30-LOC `LocFunction`
finding for the embedded function.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
