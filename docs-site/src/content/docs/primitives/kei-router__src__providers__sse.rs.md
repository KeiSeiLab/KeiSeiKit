---
title: sse.rs
path: kei-router/src/providers/sse.rs
dna_hash: sha256:c16785df618424f5
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-router/src/providers/sse.rs

Minimal SSE frame parser, shared by all providers.

SSE frames are separated by `\n\n`. Each frame may have multiple lines;
we only care about the `data: ` line. Returns the JSON payload string per
frame (caller decides how to interpret it).

## Public API

- `pub fn push` — Push bytes; return any complete `data:` payloads (one per completed frame).

## Related

- parent: `kei-router/Cargo.toml`
- imports: bytes

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
