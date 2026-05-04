---
title: test_naming_consistency.rs
path: kei-cleanup/tests/test_naming_consistency.rs
dna_hash: sha256:d0b88404a256c951
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cleanup/tests/test_naming_consistency.rs

Integration test for the naming_consistency scanner.

Tmp crate contains both `D_INIT` and `DEFAULT_D` constants. cleanup
config declares them as a synonym pair → drift should be flagged.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
