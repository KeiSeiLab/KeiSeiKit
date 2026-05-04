---
title: classifier.rs
path: frustration-matrix/tests/classifier.rs
dna_hash: sha256:940a0dc0d4d648a0
language: rust
size_loc: 170
generated: by-keidocs
---

# frustration-matrix/tests/classifier.rs

Integration tests for the likelihood-ratio classifier.

Uses `#[path]` to pull the modules under test directly from src/,
matching the pattern used in `tests/integration.rs` (no library
crate surface).

Test fixtures are built via `Firmware::train_from_text` (Z1's
in-memory trainer) so we don't need disk I/O for most cases. The
two `load_from_dir*` tests DO hit disk via `tempfile`.

## Public API

- Build an in-memory `Classifier` with two opinionated categories
- Debug-render scores without impl Debug requirement on CategoryScore.

## Related

- parent: `frustration-matrix/tests`
- imports: classifier, firmware, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
