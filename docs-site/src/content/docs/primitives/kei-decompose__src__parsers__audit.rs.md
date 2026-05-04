---
title: audit.rs
path: kei-decompose/src/parsers/audit.rs
dna_hash: sha256:555e7714cd00dc30
language: rust
size_loc: 184
generated: by-keidocs
---

# kei-decompose/src/parsers/audit.rs

/wave-audit adapter.

Detects the Wave-audit MD shape:
- `## Wave N` heading (or "Audit Report" / similar)
- `## Priority Matrix` section (table of findings)
- `## Apply Plan` section (actionable next steps)

Extracts each Priority Matrix row as one `Action`. Header columns:
`| # | Severity | Finding | Fix | Complexity | Blast | Score | [E] |`
plus tolerated header variants.

## Public API

- Locate the `## Priority Matrix` heading, then return the index of the

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
