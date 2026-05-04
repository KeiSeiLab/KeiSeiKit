---
title: cli_args.rs
path: mock-render/src/cli_args.rs
dna_hash: sha256:1fa7018fd4cbdeab
language: rust
size_loc: 33
generated: by-keidocs
---

# mock-render/src/cli_args.rs

Shared CLI-arg helpers for every mock-render subcommand.

Extracted from `main.rs` in v0.14.1 to keep that dispatcher ≤40 LOC
per Constructor Pattern.

## Public API

- `pub fn flag` — Look up a `--name <value>` pair in the arg slice.
- `pub fn parse_viewport` — Parse `WxH` viewport (e.g. `1280x800`).
- `pub fn require_project_section` — Require `--project` (default `.`) and `--section <existing-file>`.

## Related

- parent: `mock-render/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
