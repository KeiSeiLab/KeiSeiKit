---
title: coverage.rs
path: kei-arch-derive/src/coverage.rs
dna_hash: sha256:cf79776efa2b4547
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-arch-derive/src/coverage.rs

2-axis coverage metric: presence + agreement.

- **presence** = `|blocks_with_formula| / |blocks_total|`.
- **agreement** = mean Jaccard over blocks with both declared and
inferred effect-sets. For v0.1 PR-3 the inference pass is deferred
to PR-4, so `agreement = 1.0` whenever no inferred sets are supplied
(vacuous truth) — callers planning to gate CI on agreement should
wait for PR-4 to feed real inferred sets in.

Output is a `Coverage` struct serialisable to JSON by the CLI.

## Public API

- 2-axis coverage report for a single registry pass.
- `pub fn compute` — Compute coverage from raw counts and per-block effect-sets.
- `pub fn jaccard` — Jaccard similarity for two effect sets. Empty-vs-empty = 1.0

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: kei_registry, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
