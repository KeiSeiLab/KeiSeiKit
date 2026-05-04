---
title: key.rs
path: kei-cache/src/key.rs
dna_hash: sha256:3a0ad4ff016aa953
language: rust
size_loc: 98
generated: by-keidocs
---

# kei-cache/src/key.rs

Cache key derivation.

Constructor Pattern: one cube = canonical JSON serialisation + SHA-256.
Key = SHA-256(atom_id || '\0' || canonical_json(input)).

Canonical JSON: object keys sorted lexicographically at every depth, no
insignificant whitespace. Ensures semantically-identical inputs hash to
the same bytes regardless of source formatting.

## Public API

- `pub fn canonical_json` — Produce canonical JSON bytes: stable key order, minimal whitespace.
- Recursively canonicalise: sort object keys at every nesting level.
- `pub fn cache_key` — Compute cache key as 64-hex SHA-256 digest of (atom_id \0 canonical_json).
- Hex-encode lowercase without pulling a separate crate.

## Related

- parent: `kei-cache/Cargo.toml`
- imports: serde_json, sha2

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
