---
title: trait_matcher_fixtures.rs
path: kei-import-project/tests/trait_matcher_fixtures.rs
dna_hash: sha256:d74f3c8627ef9457
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-import-project/tests/trait_matcher_fixtures.rs

A2.1 integration tests: validate trait-pattern matching against real sibling crates.

Each positive fixture asserts that match_module() detects the expected
TraitKind with confidence >= 0.5 when given a real crate's source files.
Negative fixtures assert that utility crates produce no confident matches.

## Related

- parent: `kei-import-project/tests`
- imports: kei_import_project, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
