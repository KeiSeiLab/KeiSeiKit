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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
