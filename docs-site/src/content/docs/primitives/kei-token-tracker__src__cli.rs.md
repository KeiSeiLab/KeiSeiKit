---
title: cli.rs
path: kei-token-tracker/src/cli.rs
dna_hash: sha256:ce22156ca395caaa
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-token-tracker/src/cli.rs

Clap dispatch for the `kei-token-tracker` CLI binary.

## Public API

- Print total event count.
- List most recent events (default 20).
- Aggregate by model since N days ago.
- Render the Phase D nightly markdown report.
- Optional output path; default = stdout.
- `pub fn run` — Entry point; returns a process exit code so the bin can exit cleanly.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: chrono, clap, crate, std

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
