---
title: dispatch.rs
path: kei-cortex/src/tool/dispatch.rs
dna_hash: sha256:45e759a9a44a8a1c
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-cortex/src/tool/dispatch.rs

Per-turn outcome dispatcher.

`dispatch_outcome` translates a `TurnOutcome` (model said: text-only,
tool-use, or invoker-error) into a flat `Vec<LoopEvent>` the outer
`inner_loop` can yield in order. This file is split out from
`loop_driver.rs` so each cube stays inside the Constructor Pattern
200-LOC file ceiling.

## Public API

- Translate one `TurnOutcome` into a flat sequence of events. When the
- Build the event list for a `Final` outcome.
- Build the event list for a `Continue` outcome and stash tool results

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
