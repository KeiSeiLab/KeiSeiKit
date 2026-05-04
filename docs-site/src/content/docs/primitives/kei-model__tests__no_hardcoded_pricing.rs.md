---
title: no_hardcoded_pricing.rs
path: kei-model/tests/no_hardcoded_pricing.rs
dna_hash: sha256:49bf08e0b068f29c
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-model/tests/no_hardcoded_pricing.rs

RULE 0.4 guardrail: scan `src/**/*.rs` and reject any hardcoded pricing
literal. Pricing belongs in `data/models.toml`, never in Rust source.

Two heuristic patterns:
1. `\d+_per_mtok` — direct mention of a numeric rate.
2. A digit run of 4+ within ~20 chars of the word "pricing" — covers
structured-literal smuggling.

## Related

- parent: `kei-model/tests`
- imports: regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
