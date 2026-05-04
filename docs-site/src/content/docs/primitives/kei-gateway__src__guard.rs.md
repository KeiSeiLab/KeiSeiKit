---
title: guard.rs
path: kei-gateway/src/guard.rs
dna_hash: sha256:e3d1c4377ab0d5cd
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-gateway/src/guard.rs

Per-session run guard (port of Hermes asyncio.Event pattern).

Two messages arriving for the same `session_key` while an agent is mid-run
must serialise — Hermes uses an `asyncio.Event` per session; we use a
`tokio::sync::Notify` keyed on the session_key, with stale-lock heal.

## Public API

- Internal record for one session's lock.
- When the active permit was issued, for stale detection.
- Tracks one in-flight agent run per session_key.
- Acquire the lock for `session_key`. Blocks until any concurrent run on
- `pub fn is_held` — True if any session is currently held. Test helper.
- `pub struct SessionLock` — RAII handle. Dropping releases the permit and notifies waiters.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: dashmap, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
