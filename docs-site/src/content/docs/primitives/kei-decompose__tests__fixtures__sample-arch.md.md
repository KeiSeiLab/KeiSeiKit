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
