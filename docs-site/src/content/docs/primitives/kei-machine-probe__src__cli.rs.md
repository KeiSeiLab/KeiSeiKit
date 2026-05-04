---
title: cli.rs
path: kei-machine-probe/src/cli.rs
dna_hash: sha256:f61a8ea7fcd4bb13
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-machine-probe/src/cli.rs

clap CLI shapes — three subcommands.

Constructor Pattern: this cube holds parser structs only. Dispatch
happens in `main.rs`; per-subcommand handlers live in this module
and call into the library.

## Public API

- Run all detectors, emit JSON Machine struct.
- Probe + recommend, emit JSON Recommendations struct.
- Probe + recommend, emit human-readable summary.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: clap, crate, std

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
