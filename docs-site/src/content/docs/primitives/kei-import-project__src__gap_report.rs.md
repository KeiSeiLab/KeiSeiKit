---
title: gap_report.rs
path: kei-import-project/src/gap_report.rs
dna_hash: sha256:a47da2956deea712
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-import-project/src/gap_report.rs

gap_report — produce a markdown gap report from module match analyses.

Three sections: confident matches (≥0.5), weak signals (0.3–0.5),
unmatched modules (no trait fits). Sorted descending by confidence
within sections; unmatched modules alphabetic.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- `pub struct ModuleAnalysis` — Per-module analysis combining module metadata with matcher output.
- Number of source files in this module (for unmatched section display).
- Estimated LOC (0 if unknown).
- Match scores from `match_module`, already sorted desc by confidence.
- `pub fn render_gap_report` — Produce a markdown gap report covering all modules.
- Best confident match (confidence ≥ 0.5), if any.
- Best weak match (0.3 ≤ confidence < 0.5) ONLY when there's no confident match.
- True when NO match meets even the weak threshold.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
