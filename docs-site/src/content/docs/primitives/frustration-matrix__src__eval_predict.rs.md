---
title: eval_predict.rs
path: frustration-matrix/src/eval_predict.rs
dna_hash: sha256:cb648251463dcf8e
language: rust
size_loc: 130
generated: by-keidocs
---

# frustration-matrix/src/eval_predict.rs

Category predictors — trait + two real implementations.

The `CategoryPredictor` trait isolates the eval loop from concrete
classifier internals so tests can inject lightweight mocks (see
`tests/eval.rs`). Two real impls live here:

* `RegexPredictor`     — v1: walk compiled category table, first
matching regex wins, else "uncategorized".
* `FirmwarePredictor`  — v2: delegate to `Classifier::classify`
with the permissive `min_len=0, threshold=-inf`
settings mandated by the spec (we want the
top category even for very short inputs so
the eval never returns None for length).

Constructor Pattern: one file, one responsibility (turn text → label).
All stateless functions except for the two thin predictor structs,
which hold their pre-compiled categories / loaded classifier.

## Public API

- `pub const UNCATEGORIZED` — Shared label for anything a classifier cannot place.
- `pub trait CategoryPredictor` — Category-classification strategy — trait to allow test stubs.
- Return the predicted category label for `text`. Must be a total
- `pub struct RegexPredictor` — Regex-based predictor (v1). Walks categories in seed order and picks
- `pub fn new` — Wrap a pre-compiled category table. Take ownership so the predictor
- `pub struct FirmwarePredictor` — Firmware-based predictor (v2). Delegates to the loaded `Classifier`.
- `pub fn predict_all` — Run `predictor.predict` over every gold row, preserving order.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
