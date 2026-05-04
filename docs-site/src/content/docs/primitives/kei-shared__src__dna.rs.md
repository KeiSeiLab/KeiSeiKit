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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
