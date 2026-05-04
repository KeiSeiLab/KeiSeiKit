---
title: main.rs
path: kei-machine-probe/src/main.rs
dna_hash: sha256:9ee87c528c2e3a26
language: rust
size_loc: 12
generated: by-keidocs
---

# kei-machine-probe/src/main.rs

kei-machine-probe — CLI entry.

Constructor Pattern: `main` does parse + dispatch only. All subcommand
bodies live in `cli.rs`; all detection lives in the library. ≤30 LOC.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: clap, kei_machine_probe, std

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
