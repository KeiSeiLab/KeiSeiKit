---
title: runner.rs
path: kei-gateway/src/runner.rs
dna_hash: sha256:c390ba30b90bd224
language: rust
size_loc: 131
generated: by-keidocs
---

# kei-gateway/src/runner.rs

Orchestrates the full inbound → agent → delivery loop.

Hermes equivalent: `gateway/run.py:_handle_message_event`.

## Public API

- Type-erased agent runner.
- Process an inbound event and return the agent's outbound text. Empty
- Configuration for [`GatewayRunner`].
- Channel buffer for inbound events.
- `pub struct GatewayRunner` — Top-level gateway runtime.
- Process a single inbound event end-to-end.
- `pub fn start` — Spawn the inbound consume loop on the current Tokio runtime. Returns the
- Helper for tests: drop the agent cache. Mostly for debug.
- Build an [`DeliveryTarget::Origin`] from the inbound event source.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, crate, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
