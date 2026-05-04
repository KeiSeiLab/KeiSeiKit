---
title: chat_token.rs
path: kei-cortex/src/handlers/chat_token.rs
dna_hash: sha256:8f9ca50198d26887
language: rust
size_loc: 93
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_token.rs

Token-event recording side-channel for the chat handlers.

Constructor Pattern split (cleanup follow-up to Phase 2 wiring): the
`kei-token-tracker` write path was originally inlined in
`chat_cost.rs`, pushing that file to 263 LOC. This cube extracts the
token-event concern (TokenWrite bundle + record_token_event +
spawn_record_token_event + build_token_event helper) so each file
stays under the 200-LOC ceiling and the cost vs token concerns are
cleanly separated:
- `chat_cost.rs` — kei-ledger cost writes (existed pre-Phase 2)
- `chat_token.rs` (this file) — kei-token-tracker event writes

Failure policy: every helper here is fire-and-forget. Tracker write
failures log to stderr; they never surface to the chat caller.

## Public API

- Bundle for one [`TokenEvent`] write. Held by the post-Done callback
- `pub fn record_token_event` — Record one token-event into the tracker. Fire-and-forget: any error
- Build a [`TokenEvent`] from the per-turn `TokenWrite`. Saturating
- `pub fn spawn_record_token_event` — Spawn a blocking task that performs the tracker write off the async
- Local clock helper. Mirrors `chat_cost::unix_now` so this cube has

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_token_tracker, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
