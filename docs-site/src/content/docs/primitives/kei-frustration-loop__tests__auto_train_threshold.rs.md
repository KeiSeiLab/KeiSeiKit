---
title: auto_train_threshold.rs
path: kei-frustration-loop/tests/auto_train_threshold.rs
dna_hash: sha256:bca3364a04fd0279
language: rust
size_loc: 122
generated: by-keidocs
---

# kei-frustration-loop/tests/auto_train_threshold.rs

Threshold trigger logic. Populate N-1 feedback rows → assert
`should_retrain == false`. Add one more → assert `true`. Then run the
actual `auto_train` and verify a new firmware file was written.

## Related

- parent: `kei-frustration-loop/tests`
- imports: kei_frustration_loop, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
