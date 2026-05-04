---
title: architecture.rs
path: kei-decompose/src/parsers/architecture.rs
dna_hash: sha256:9102377cfd7d7923
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-decompose/src/parsers/architecture.rs

/architecture decisions adapter.

Detects:
- `## Decision` heading (single architectural decision file)
- `## Recommendation` / `## Recommendations` section

Extracts:
- Numbered recommendations under the recommendations heading
(e.g. `1. Adopt X`, `2. Refactor Y`) — one Action per item.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
