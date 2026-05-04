---
title: chat_test.rs
path: kei-cortex/src/handlers/chat_test.rs
dna_hash: sha256:9282901790826eed
language: rust
size_loc: 156
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_test.rs

Inline unit tests for `chat.rs`.

Loop-termination paths are covered exhaustively in
`tool/tests/loop_terminates_on_max_turns.rs`. These tests focus on the
per-event SSE translation, body validation, and provider validation —
the surfaces unique to the handler layer.

After Wave 40 split, the SSE event constructors and `loop_event_to_sse`
moved to sibling cubes (`chat_events.rs`, `chat_stream.rs`); the tests
reach them via the parent `super::chat_*::` paths.

## Related

- parent: `kei-cortex/Cargo.toml`
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
