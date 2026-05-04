---
title: builder.rs
path: kei-export-trajectories/src/builder.rs
dna_hash: sha256:cadcbc9cc399560d
language: rust
size_loc: 85
generated: by-keidocs
---

# kei-export-trajectories/src/builder.rs

Convert a hydrated `TrajectoryRecord` into a ShareGPT `Trajectory`.

Constructor Pattern: pure function cube — no I/O, no SQL. Lives in
the library so both the CLI binary and the integration test exercise
the same code path.

Hermes normalization rules applied here:
- every `gpt` turn carries a `<think>` block (empty if no reasoning)
- system prompt is generated at save-time, NOT taken from the
conversation source

## Public API

- `pub const DEFAULT_SYSTEM_PROMPT` — Hermes-style canonical system prompt. Kept short — the real
- Back-compat alias — was a const, now resolves at call-site via env.
- `pub fn system_prompt` — Read `KEI_EXPORT_SYSTEM_PROMPT` env var; fall back to [`DEFAULT_SYSTEM_PROMPT`].
- `pub fn record_to_trajectory` — Synthesize a multi-turn conversation (system + human + N gpt/tool turns)

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
