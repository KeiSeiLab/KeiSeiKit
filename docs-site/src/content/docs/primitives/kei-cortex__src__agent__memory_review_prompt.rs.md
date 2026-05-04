---
title: memory_review_prompt.rs
path: kei-cortex/src/agent/memory_review_prompt.rs
dna_hash: sha256:e81abe76eee8cce7
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-cortex/src/agent/memory_review_prompt.rs

Background-review prompt template.

Constructor Pattern: a single immutable string + a small builder
adapting it to KeiSei voice. Ported from Hermes
`run_agent.py:3147-3156`. The template intentionally bounds the
review agent to two questions and one short-circuit phrase
(`"Nothing to save."`) — that bound is what makes the background
pass cheap and reliably terminating.

## Public API

- `pub const REVIEW_PROMPT` — Verbatim review-prompt body. Adapted from Hermes
- `pub const NOTHING_TO_SAVE` — Short-circuit phrase the review agent emits when the conversation
- `pub fn is_nothing_to_save` — True when the agent's reply is the recognised short-circuit.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
