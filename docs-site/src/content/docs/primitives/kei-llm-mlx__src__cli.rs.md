---
title: cli.rs
path: kei-llm-mlx/src/cli.rs
dna_hash: sha256:425aa840992f5bed
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-llm-mlx/src/cli.rs

clap CLI shapes — five subcommands.

Constructor Pattern: this cube holds parser structs + the dispatch
table only. Per-subcommand bodies live in sibling cubes
(`platform`, `discovery`, `models`, `generate`, `server`). Every
handler checks the platform gate FIRST and exits with code 2 + a
stable JSON payload when unsupported.

## Public API

- Probe: platform gate + discover mlx_lm binaries.
- List MLX-quantised models cached under HuggingFace hub.
- Run a single non-streaming generation.
- Spawn `mlx_lm.server` for an OpenAI-compat local HTTP endpoint.
- Print version metadata for both the wrapper and discovered mlx_lm.

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: clap, crate, serde_json, std

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
