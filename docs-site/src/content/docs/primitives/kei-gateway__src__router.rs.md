---
title: router.rs
path: kei-gateway/src/router.rs
dna_hash: sha256:bf746f5ebda2ee10
language: rust
size_loc: 87
generated: by-keidocs
---

# kei-gateway/src/router.rs

Outbound delivery router.

Mirrors the Hermes `_resolve_delivery_target` / `_deliver_result` flow
(cron/scheduler.py:150-484): a job's output is dispatched to the source
channel, a configured home channel, or an explicit `platform:chat_id` ref.

## Public API

- Where to deliver a response.
- Reply on the same channel the message came from.
- Local-only — write to file, no platform send.
- Explicit destination override.
- Routes [`OutboundMessage`] to the right [`PlatformAdapter`].
- `pub fn register` — Register an adapter for `platform`. Replaces any existing entry.
- `pub fn adapter_count` — Number of adapters wired in. Test / observability helper.
- Dispatch `msg` to the resolved `target`. Returns the adapter's result,

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
