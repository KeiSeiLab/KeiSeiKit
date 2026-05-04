---
title: dna.rs
path: kei-shared/src/dna.rs
dna_hash: sha256:e09beaa484268209
language: rust
size_loc: 124
generated: by-keidocs
---

# kei-shared/src/dna.rs

DNA wire format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`.

SSoT for the substrate identity string. Any format-level change lands
here and propagates to consumers (kei-agent-runtime, kei-dna-index)
through re-export, not duplication.

## Public API

- Parsed DNA fields. Widths on hash segments are validated by `parse_dna`.
- Strict parse errors. Consumers that need looser semantics (e.g. legacy
- `pub fn is_hex8` — `true` iff `s` is exactly 8 ASCII hex characters.
- `pub fn parse_dna` — Strict parse. Requires 4 `::` segments, `<body>-<nonce>` tail, and
- `pub fn compose_dna` — Render the canonical wire format. Deterministic — no I/O, no randomness.

## Related

- parent: `kei-shared/Cargo.toml`
- imports: serde, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
