---
title: eval.rs
path: frustration-matrix/src/eval.rs
dna_hash: sha256:3efd2425a6bc440a
language: rust
size_loc: 115
generated: by-keidocs
---

# frustration-matrix/src/eval.rs

Eval — compare regex-based (v1) vs firmware-based (v2) classification on
a hand-labelled gold set. Reports per-category precision / recall / f1,
overall accuracy and macro-f1, plus two confusion matrices.

This module consumes APIs from firmware.rs (Z1) and classifier.rs (Z2).
If those modules have different method names at orchestrator-merge time,
update the call sites here — the eval LOGIC is independent of the
internal firmware representation.

Constructor Pattern: this file holds only the public types + the
`evaluate` orchestrator. Helpers live in sibling cubes:

* `eval_gold`    — parse labelled JSONL, filter quality=gold
* `eval_predict` — `CategoryPredictor` trait + regex / firmware impls
* `eval_metrics` — pure precision / recall / f1 math
* `eval_report`  — CSV write + stdout summary

Purity: every mathematical step in eval_metrics is a pure function of
two integer vectors (true + predicted). Predictors are behind a trait
so tests can inject `MockClassifier` without Z1/Z2 on disk.

## Public API

- `pub struct EvalInput` — CLI input bundle — from `main.rs` eval subcommand.
- `pub struct EvalReport` — Full report produced by `evaluate`.
- `pub struct Metrics` — Overall + per-category metrics for one classifier.
- `pub struct PerCategoryMetric` — Per-category precision / recall / f1 / support, sklearn convention.
- `pub fn evaluate` — Run the full eval pipeline: load gold, run both classifiers, compute
- Core eval loop over gold rows + two predictors — the pure-function
- One parsed gold row — shared input type for predictors + metrics.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
