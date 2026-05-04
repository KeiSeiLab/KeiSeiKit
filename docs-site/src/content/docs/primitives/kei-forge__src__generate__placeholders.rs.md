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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
