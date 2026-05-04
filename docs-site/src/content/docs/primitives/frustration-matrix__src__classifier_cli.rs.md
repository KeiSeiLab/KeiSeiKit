---
title: classifier_cli.rs
path: frustration-matrix/src/classifier_cli.rs
dna_hash: sha256:331fc010dfc9443f
language: rust
size_loc: 41
generated: by-keidocs
---

# frustration-matrix/src/classifier_cli.rs

CLI glue for the `classify` subcommand.

Constructor Pattern: main.rs stays dispatch-only; this cube owns the
print-a-ranking layer. Pure function of (dir, message, min_len,
threshold). Splits load + classify + print into three tiny helpers.

## Public API

- `pub fn run` — Entry point called from `main::dispatch`. Load bundle, classify, print.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

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
