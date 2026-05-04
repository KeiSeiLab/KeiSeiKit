---
title: placeholders.rs
path: kei-forge/src/generate/placeholders.rs
dna_hash: sha256:b9b7844b2604070b
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-forge/src/generate/placeholders.rs

Placeholder substitution for atom-template rendering.

Pure string replace — six tokens, one pass per token. Called by
`super::generate_atom` for each of the five template files.

Order matters: `__CRATE_SNAKE__` must be replaced BEFORE `__CRATE__`
(the latter is a substring of the former). Same for `__VERB_SNAKE__`
vs `__VERB__`. The implementation does longer-tokens-first.

## Public API

- `pub fn substitute` — Apply all six substitutions to `src`. Longer tokens first so that

## Related

- parent: `kei-forge/Cargo.toml`
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
