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
