---
title: markdown.rs
path: kei-machine-probe/src/markdown.rs
dna_hash: sha256:9e02e506e4fc68ac
language: rust
size_loc: 148
generated: by-keidocs
---

# kei-machine-probe/src/markdown.rs

Render `Machine` + `Recommendations` as a markdown / plaintext report.

Constructor Pattern: rendering is one cube. The CLI's `report`
subcommand calls `render_markdown(&m, &r)` (or `render_plain` when
`--markdown` is absent). No formatting logic in `cli.rs`.

## Related

- parent: `kei-machine-probe/Cargo.toml`
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
