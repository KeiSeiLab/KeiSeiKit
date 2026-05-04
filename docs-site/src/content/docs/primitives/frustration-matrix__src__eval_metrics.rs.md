---
title: eval_metrics.rs
path: frustration-matrix/src/eval_metrics.rs
dna_hash: sha256:1c23042db07f1b73
language: rust
size_loc: 162
generated: by-keidocs
---

# frustration-matrix/src/eval_metrics.rs

Metric math — pure functions over two parallel label vectors.

No IO, no predictors, no disk. Every function takes `&[&str]` + returns
numbers or HashMaps. Follows sklearn convention:

* precision_c  = TP_c / (TP_c + FP_c)       (0 if denominator 0)
* recall_c     = TP_c / (TP_c + FN_c)       (0 if denominator 0)
* f1_c         = 2 · P · R / (P + R)        (0 if denominator 0)
* support_c    = number of gold rows with true=c
* accuracy     = correct / total            (0 if total=0)

Macro-F1 is computed in `eval_report`; it is the arithmetic mean of
per-category f1 scores over categories WITH support > 0.

Zero-support categories: we still emit a row (precision=recall=f1=0),
so the report can show them — matches the spec test
`per_category_metrics_handle_zero_support`.

## Public API

- `pub fn compute_metrics` — Compute full metrics bundle from parallel truth / prediction vectors.
- Overall accuracy — correct predictions over total rows.
- One `PerCategoryMetric` per category that appears in EITHER vector.
- Sorted set of every category label seen in truth OR pred.
- Compute precision / recall / f1 / support for ONE category label.
- TP / FP / FN counts for one category under one-vs-rest framing.
- `pub fn build_confusion` — Build a (true, predicted) → count confusion matrix.
- `pub fn macro_f1` — Macro-F1 = arithmetic mean of per-category f1 over categories with

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
