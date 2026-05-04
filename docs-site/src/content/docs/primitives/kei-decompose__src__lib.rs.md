---
title: lib.rs
path: kei-decompose/src/lib.rs
dna_hash: sha256:aa90450fe50a5ac3
language: rust
size_loc: 34
generated: by-keidocs
---

# kei-decompose/src/lib.rs

kei-decompose — UNIVERSAL decomposition layer.

Supersedes the format-specific kei-decision (Wave 51) by treating it as
one adapter among many. Closes Wave 50 META-finding: kit has 6+ MD-output
formats (research / wave-audit / sleep / architecture / new-project /
compose-solution), but only `research` had a path to action via
kei-decision. kei-decompose unifies the decomposition layer.

Pipeline:
ANY MD output  →  detect  →  parser_registry  →  Action[]
↓
emit
↓
task.toml[] for kei-spawn
↓
dispatch
↓
kei-spawn (fork + ledger)

Constructor Pattern: each module owns one responsibility, ≤ 200 LOC,
≤ 30 LOC per fn. No async, no network, no md crate (regex-only).

## Related

- parent: `kei-decompose/Cargo.toml`

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
