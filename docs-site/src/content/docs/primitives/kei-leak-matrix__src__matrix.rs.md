---
title: matrix.rs
path: kei-leak-matrix/src/matrix.rs
dna_hash: sha256:171197d1297d85c0
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-leak-matrix/src/matrix.rs

Matrix loader — parses leak-matrix.toml, compiles every regex upfront.

Pattern strings are IP. Never echoed outside the in-memory regex.
Public-facing fields: id, category, severity, scope, rationale, added.

## Public API

- One compiled rule. `pattern` is private — only `regex` is exposed.
- `pub fn default_matrix_path` — Default matrix path: $KEI_LEAK_MATRIX_PATH or ~/Projects/KeiSeiKit/security/leak-matrix.toml
- `pub fn cmd_list` — Handler: print rules as a markdown table; optional category filter.
- `pub fn cmd_lint` — Handler: lint — does any existing rule already cover the candidate input?

## Related

- parent: `kei-leak-matrix/Cargo.toml`
- imports: anyhow, regex, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
