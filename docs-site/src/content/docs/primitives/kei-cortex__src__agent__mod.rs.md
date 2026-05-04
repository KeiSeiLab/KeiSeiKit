---
title: mod.rs
path: kei-cortex/src/agent/mod.rs
dna_hash: sha256:e6bbaff9c5767f1c
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-cortex/src/agent/mod.rs

Agent-side helpers — periodic memory review, nudge scheduling, and
the prompt template used by the background reviewer.

Constructor Pattern: each cube is a single-responsibility module.
Public surface is intentionally narrow — only the scheduler trigger
and the review-task entry point are reachable from outside.

Frozen-snapshot invariant: nothing in this module mutates the
parent agent's in-flight system prompt. Background reviews write
exclusively to disk-backed memory stores. The next session picks
up the new snapshot via the normal load path; the running session
is left undisturbed (preserves Anthropic prefix-cache hits).

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
