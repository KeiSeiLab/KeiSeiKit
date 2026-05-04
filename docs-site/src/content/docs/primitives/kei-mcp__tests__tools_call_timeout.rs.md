---
title: tools_call_timeout.rs
path: kei-mcp/tests/tools_call_timeout.rs
dna_hash: sha256:4a3b251aebb4b4a9
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-mcp/tests/tools_call_timeout.rs

Integration test — `tools/call` enforces the 60s timeout (MISS-4).

Strategy: write a fake binary that sleeps significantly longer than the
cap. Use `tokio::test(start_paused = true)` so virtual time can be
advanced past the 60s deadline without the test thread blocking. The
real child is reaped by `kill_on_drop(true)` set in `spawn_and_collect`.

Real wall-clock spent on this test: a couple of milliseconds — the
`sleep 90` child is spawned but killed before it finishes its first
second.

## Related

- parent: `kei-mcp/tests`
- imports: kei_mcp, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
