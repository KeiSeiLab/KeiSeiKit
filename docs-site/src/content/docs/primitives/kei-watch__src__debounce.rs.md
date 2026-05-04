---
title: debounce.rs
path: kei-watch/src/debounce.rs
dna_hash: sha256:54428616b34da358
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-watch/src/debounce.rs

Coarse debounce: collapse duplicate `(path, kind)` events fired
within [`DEBOUNCE_WINDOW`] of the previous one.

Intent: swallow FS-level bursts (editor-write double-fire, compiler
rewrite patterns). NOT a replacement for notify-debouncer-full — we
don't do event reordering or close/write correlation, just per-key
rate-limiting.

## Public API

- `pub const DEBOUNCE_WINDOW` — Collapse window for duplicate `(path, kind)` pairs.
- `pub struct Debouncer` — Per-key last-seen state. Small enough to live in a `HashMap` — pruned
- `pub fn accept` — Should this event pass through?
- Drop entries older than 10× the debounce window. Called

## Related

- parent: `kei-watch/Cargo.toml`
- imports: crate, std

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
