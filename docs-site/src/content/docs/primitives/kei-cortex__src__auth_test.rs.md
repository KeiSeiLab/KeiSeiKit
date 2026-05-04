---
title: auth_test.rs
path: kei-cortex/src/auth_test.rs
dna_hash: sha256:6a483d6ac6140108
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-cortex/src/auth_test.rs

Inline unit tests for `auth.rs`.

Coverage:
- `tokens_match` is case-insensitive (MISS-6 fix).
- `validate_hex` accepts both cases, rejects bad length / non-hex.
- `generate_token` round-trips through validate + match.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
