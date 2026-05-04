---
title: mod.rs
path: kei-decompose/src/parsers/mod.rs
dna_hash: sha256:26f045b5cc39cd37
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-decompose/src/parsers/mod.rs

FormatParser trait + ordered registry.

`Vec<Box<dyn FormatParser>>` (NOT a HashMap) preserves detection order
across calls: the first parser to claim wins. Ties resolve by registration
order, never by HashMap iteration.

## Public API

- Detection confidence — exact-match vs header-only vs ambiguous.
- `pub trait FormatParser` — One adapter per MD output format.
- `pub fn registry` — Standard parser registry — order = detection priority.
- Detection result: best-matching parser plus the full per-parser scoreboard.
- `pub fn detect_format` — Run every parser's `detect`, return best score (ties → first registered).
- `pub fn parser_by_name` — Lookup parser by lowercase name; returns None if not registered.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
