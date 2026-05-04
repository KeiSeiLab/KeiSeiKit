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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
