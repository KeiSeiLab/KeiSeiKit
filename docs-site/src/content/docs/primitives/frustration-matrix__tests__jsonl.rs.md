---
title: jsonl.rs
path: frustration-matrix/tests/jsonl.rs
dna_hash: sha256:7bbd78fc722fab2b
language: rust
size_loc: 118
generated: by-keidocs
---

# frustration-matrix/tests/jsonl.rs

Integration tests for `jsonl` cube.

Constructor Pattern: one scenario per test. We mount `jsonl.rs` via
`#[path]` (same pattern as `integration.rs`) so no library crate
surface is required. Fixtures are written to `tempfile::TempDir` —
nothing persists after the test.

## Related

- parent: `frustration-matrix/tests`
- imports: jsonl, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
