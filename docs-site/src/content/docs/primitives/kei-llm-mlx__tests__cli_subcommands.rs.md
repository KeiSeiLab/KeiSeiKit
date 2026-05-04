---
title: cli_subcommands.rs
path: kei-llm-mlx/tests/cli_subcommands.rs
dna_hash: sha256:4e37ffd367d44cb6
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-llm-mlx/tests/cli_subcommands.rs

clap parses all 5 subcommands.

Pure parser test: we just verify the `Cli` struct accepts the canonical
invocations for `probe`, `models`, `generate`, `server`, `version`. No
dispatch — that path is exercised by the integration smoke during
cargo build.

## Related

- parent: `kei-llm-mlx/tests`
- imports: clap, kei_llm_mlx

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
