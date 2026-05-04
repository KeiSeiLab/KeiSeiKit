---
title: loop_terminates_on_max_turns.rs
path: kei-cortex/src/tool/tests/loop_terminates_on_max_turns.rs
dna_hash: sha256:a0371b8ee9d4e702
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-cortex/src/tool/tests/loop_terminates_on_max_turns.rs

Validates the loop's hard termination guarantees:
- infinite tool-use stream stops at MAX_TURNS with an Error event
- cancellation via CancellationToken causes early Done
- clean end_turn stop_reason terminates immediately

Wave 44c (F-HIGH-5): cancel migrated from `oneshot::Receiver<()>`
to `tokio_util::sync::CancellationToken`.

## Public API

- Build an invoker that ALWAYS asks for one tool_use, simulating a
- A registry where `noop` always succeeds. Keeps the loop fed.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, futures, std, tokio_util

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
