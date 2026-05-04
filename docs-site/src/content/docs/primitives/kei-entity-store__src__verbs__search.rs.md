---
title: search.rs
path: kei-entity-store/src/verbs/search.rs
dna_hash: sha256:a7776a2baa937cf5
language: rust
size_loc: 148
generated: by-keidocs
---

# kei-entity-store/src/verbs/search.rs

`search` verb — FTS5 match over `fts_<table>`, JOIN back to entity
table, ORDER BY rank.

Requires `EntitySchema.fts_columns` to be `Some`.

Security: user input is wrapped in an FTS5 double-quoted phrase so
the FTS5 query grammar (`col:term`, `NEAR/5`, boolean ops, `*`,
parentheses) is treated as LITERAL TEXT. This is a pure keyword
search — attackers cannot address unindexed columns or craft
pathological scan expressions. Embedded `"` chars in the user query
are escaped per FTS5 grammar by doubling (`"" → "`).

Tokenization guard: a query with ZERO searchable tokens (e.g. all
punctuation, only whitespace once trimmed) is rejected with
`InvalidInput` (exit 2) BEFORE reaching SQLite. This preserves the
documented exit-code contract — otherwise the porter/unicode61
tokenizer produces an empty token stream and FTS5 emits an opaque
`fts5: syntax error` that would propagate as `VerbError::Sqlite`
(exit 1).

## Public API

- Wrap a user-supplied string as an FTS5 literal phrase. Doubles any
- True if `raw` contains at least one character the FTS5 porter /

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
