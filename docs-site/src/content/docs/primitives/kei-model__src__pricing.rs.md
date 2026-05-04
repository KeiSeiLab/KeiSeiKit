---
title: pricing.rs
path: kei-model/src/pricing.rs
dna_hash: sha256:adf625f1e6e9be89
language: rust
size_loc: 96
generated: by-keidocs
---

# kei-model/src/pricing.rs

`Pricing` struct + `estimate` cost helper.

Every pricing row in the SSoT TOML ships with `status = "placeholder"` and
a `source_url` per RULE 0.4. Real micro-cents/Mtok come from a follow-up
verification commit. Callers should inspect `status` before quoting cost.

## Public API

- Micro-cents per million input tokens.
- Micro-cents per million output tokens.
- ISO-8601 date the row was last verified, e.g. "2026-04-27".
- `pub fn estimate` — Estimate total micro-cents for a (`in_tokens`, `out_tokens`) call.
- `(a * b) / d` with saturating multiply, integer floor.

## Related

- parent: `kei-model/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
