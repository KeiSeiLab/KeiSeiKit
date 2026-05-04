---
title: cli_subcommands.rs
path: kei-frustration-loop/tests/cli_subcommands.rs
dna_hash: sha256:3eafcdb24b36ec3d
language: rust
size_loc: 109
generated: by-keidocs
---

# kei-frustration-loop/tests/cli_subcommands.rs

Clap parser smoke tests for the 5 v0.40 loop subcommands. The new crate
exposes the `cli` module via `kei_frustration_loop`, so we can import
the parser directly — no `#[path]` mounting needed.

We only assert that clap accepts the flag combinations we documented;
actual side-effects are covered by the per-cube tests.

## Related

- parent: `kei-frustration-loop/tests`
- imports: clap, kei_frustration_loop

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
