---
title: diff.rs
path: kei-replay/src/diff.rs
dna_hash: sha256:5e847c99e71d7e2c
language: rust
size_loc: 73
generated: by-keidocs
---

# kei-replay/src/diff.rs

Diff — compare two DNAs facet-by-facet.

Pure parser + comparator. No I/O, no ledger lookup. Callers that want
the composed-body text diff can run `replay` on each DNA first and diff
the resulting `composed_prompt` themselves.

## Public API

- Diff report between two DNA strings.
- `pub fn is_identical` — `true` when every facet is identical (same composition, same nonce).
- `pub fn is_same_composition` — `true` when the two DNAs would re-compose to the same prompt body
- `pub fn render` — Human-readable multi-line report.
- `pub fn diff` — Parse both DNAs and emit the facet-level diff.

## Related

- parent: `kei-replay/Cargo.toml`
- imports: anyhow, kei_agent_runtime

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
