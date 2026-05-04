---
title: injection_check_binary.rs
path: kei-pet/src/injection_check_binary.rs
dna_hash: sha256:fa3d5a1e1e14f6c4
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-pet/src/injection_check_binary.rs

Binary / blob rules for `injection_check`.

Constructor Pattern: invisible-codepoint scan + base64-blob length
heuristic. No regex; per-char iteration only.

## Public API

- Detect invisible / bidi unicode codepoints anywhere in `content`.
- Detect a single line >= 1024 chars composed of base64 alphabet.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
