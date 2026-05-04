---
title: sample-arch
path: kei-decompose/tests/fixtures/sample-arch.md
dna_hash: sha256:5e43881ba901db31
language: markdown
size_loc: 20
generated: by-keidocs
---

# kei-decompose/tests/fixtures/sample-arch.md

## Public API

- `Architecture Decision Record — sample` — 
- `Decision` — Use a single-trait Parser + Vec<Box<dyn>> registry for kei-decompose, NOT a HashMap.
- `Context` — - HashMap iteration order is non-deterministic and breaks tie-resolution.
- `Recommendations` — 1. Adopt the FormatParser trait + ordered registry pattern.
- `Implementation` — Land in Wave 52, ship as new primitive kei-decompose. kei-decision stays as one adapter.

## Related

- parent: `kei-decompose/tests/fixtures`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
