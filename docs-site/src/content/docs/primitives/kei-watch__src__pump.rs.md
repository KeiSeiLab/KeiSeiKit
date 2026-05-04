---
title: pump.rs
path: kei-watch/src/pump.rs
dna_hash: sha256:c2f282d15e94f470
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-watch/src/pump.rs

Single-threaded pump: reads `Result<notify::Event>` from notify's
channel, maps + debounces, pushes canonical [`Event`] to the output
channel consumed by `next_event` / `drain`.

Exactly one thread is spawned per [`crate::Watcher`] instance. The
thread exits cleanly when notify's sender is dropped (closing the
input channel), which happens when the `notify::RecommendedWatcher`
is dropped inside [`crate::Watcher::drop`].

## Public API

- `pub fn spawn` — Spawn the pump thread.

## Related

- parent: `kei-watch/Cargo.toml`
- imports: crate, notify, std

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
