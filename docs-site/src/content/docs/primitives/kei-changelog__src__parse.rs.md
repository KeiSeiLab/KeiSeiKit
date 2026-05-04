---
title: parse.rs
path: kei-changelog/src/parse.rs
dna_hash: sha256:d362bdff33173b0c
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-changelog/src/parse.rs

Conventional-commit subject parser.

Shape: `type(scope)!: subject` — scope and `!` optional.
Returns `(kind, scope, subject, breaking)`. Malformed → `Other` kind with
the full subject as `subject`.

## Public API

- Parse a commit subject line.

## Related

- parent: `kei-changelog/Cargo.toml`
- imports: crate, regex, std

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
