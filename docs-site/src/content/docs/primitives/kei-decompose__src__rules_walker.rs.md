---
title: rules_walker.rs
path: kei-decompose/src/rules_walker.rs
dna_hash: sha256:06b527953a3478ec
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-decompose/src/rules_walker.rs

Directory walker for rule `.md` files.

Walks `<rules-dir>/*.md`, `specialty/*.md`, and `projects/*.md` (depth
≤ 2). Skips files starting with `_` and the registry index (`RULES.md`).

Constructor Pattern: this cube owns the walk + eligibility filter only.

## Public API

- `pub fn collect_rule_files` — Collect all eligible rule `.md` files from `rules_dir` and its known

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
