---
title: mod.rs
path: frustration-matrix/tests/eval_helpers/mod.rs
dna_hash: sha256:5d9605aff7cbf746
language: rust
size_loc: 126
generated: by-keidocs
---

# frustration-matrix/tests/eval_helpers/mod.rs

Shared fixtures for `tests/eval.rs`.

Lives in a subdirectory so Cargo doesn't compile it as its own test
binary. The parent test (`tests/eval.rs`) declares this with:

```ignore
#[path = "eval_helpers/mod.rs"]
mod eval_helpers;
```

The `#[path]` form is used because `super::*` from here needs to resolve
to the eval-types already wired up in the test root via `#[path]`
includes — keeping the wire-up chain confined to the test root.

## Public API

- `pub struct MockPredictor` — Text-to-category lookup predictor — used in every eval test.
- `pub fn perfect_on` — Mock that predicts each gold row's true category — a "perfect"
- `pub fn gold_row` — Shortcut for building `GoldRow` instances.
- `pub fn make_gold_set` — 6-row fixture used by `eval_from_tiny_gold_set`:
- `pub type ParsedRow` — Parsed CSV row: (model, category, precision, recall, f1, support).
- `pub fn parse_csv_body` — Parse the CSV produced by `write_csv`. Minimal RFC-4180 subset:
- `pub fn compare_model_rows` — Assert that for every expected `PerCategoryMetric`, the CSV contains
- `pub fn confusion_key` — (truth, predicted) key for a `HashMap<(String,String), usize>` hit.

## Related

- parent: `frustration-matrix/tests/eval_helpers`
- imports: std

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
