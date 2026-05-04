---
title: path.rs
path: kei-diff/src/path.rs
dna_hash: sha256:6396e2dafbb98835
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-diff/src/path.rs

RFC 6901 JSON Pointer path builder.

Root is `""`. Segments join with `/`. Inside a segment, `~` encodes as
`~0` and `/` encodes as `~1`. Order matters: `~` must be escaped first
when encoding, and `~1` must be decoded before `~0`.

## Public API

- Incremental pointer builder. Use `push`/`pop` during recursive traversal;
- `pub fn push_key` — Push an object key. Performs RFC 6901 escaping.
- `pub fn push_index` — Push an array index. Always emitted as decimal digits.
- `pub fn as_string` — Current pointer as a String. Empty string if at root.
- `pub fn parse_pointer` — Parse an RFC 6901 pointer into decoded segments. `""` → `[]`.

## Related

- parent: `kei-diff/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
