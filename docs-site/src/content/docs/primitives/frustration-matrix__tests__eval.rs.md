---
title: eval.rs
path: frustration-matrix/tests/eval.rs
dna_hash: sha256:19bae5b76f603664
language: rust
size_loc: 135
generated: by-keidocs
---

# frustration-matrix/tests/eval.rs

Integration tests for the `eval` subcommand.

Constructor Pattern: each test = one scenario, ≤ 30 LOC body. Shared
fixtures + helpers live in `tests/eval_helpers/mod.rs` — subdirectory
so Cargo does not compile them as a separate test binary.

We load source modules via `#[path = "../src/X.rs"]` (matches existing
`tests/integration.rs`). The `CategoryPredictor` trait lets each test
wire a `MockPredictor` — Z1/Z2 need not be complete.

## Related

- parent: `frustration-matrix/tests`
- imports: eval, eval_helpers, eval_metrics, eval_report, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
