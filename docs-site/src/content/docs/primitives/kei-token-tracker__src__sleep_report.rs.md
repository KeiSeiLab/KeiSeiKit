---
title: sleep_report.rs
path: kei-token-tracker/src/sleep_report.rs
dna_hash: sha256:33ed1b02fc08e4b3
language: rust
size_loc: 93
generated: by-keidocs
---

# kei-token-tracker/src/sleep_report.rs

Phase D nightly markdown sleep-report rendering.

Pure function: takes a sorted `Vec<ModelAggregate>` + a date string
and emits a deterministic markdown payload. No filesystem side-effects
here — the CLI dispatcher writes the bytes.

## Public API

- `pub fn render` — Render the markdown report. `date` is the report header (e.g.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: crate

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
