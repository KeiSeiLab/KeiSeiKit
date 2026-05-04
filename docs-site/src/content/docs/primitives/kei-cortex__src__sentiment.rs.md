---
title: sentiment.rs
path: kei-cortex/src/sentiment.rs
dna_hash: sha256:3ca42aaeb330e73f
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-cortex/src/sentiment.rs

Keyword-based sentiment classifier.

Given accumulated assistant text, score each of the 9 allowed tags by
counting case-insensitive whole-word keyword matches. Return the top
tag + confidence in `[0.0, 1.0]`. Default to `neutral @ 0.5` if no
keyword matches. Pure and synchronous — trivial to unit-test.

Wave 45 TODO — Cross-language SSoT for emotion tags:
The set below (`TAGS`) is duplicated in three places:
1. this file — Rust `&[&str]` (sentiment scoring keys).
2. `_ts_packages/cortex-ui/src/lib/emotions.ts` — TS enum.
3. `_ts_packages/cortex-ui/src/lib/chat/types.ts` — TS string union.
When a tag is added/renamed all three must change in lock-step.
Wave 45 should introduce a build-time codegen step (one canonical
source — likely a `_schemas/emotions.json` — and emit the Rust slice
+ the two TS bindings from it). This is out of scope for Wave 44d
because it requires a TS-build hook + workspace build.rs plumbing.

## Public API

- `pub const TAGS` — The exact nine tags permitted on the wire.
- Classification result.
- `pub fn classify` — Classify `text` into one of `TAGS`.
- Score every tag over the lower-cased text.
- Keyword list per tag. Chosen to be cheap and reasonable — not exhaustive.
- Count case-insensitive whole-word occurrences of each keyword.
- Whole-word match for alpha keywords; raw substring match for punctuation ("?").
- Reduce score table to the top tag + normalised confidence.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
