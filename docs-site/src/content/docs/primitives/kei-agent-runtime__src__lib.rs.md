---
title: lib.rs
path: kei-agent-runtime/src/lib.rs
dna_hash: sha256:4744f04a290822a9
language: rust
size_loc: 27
generated: by-keidocs
---

# kei-agent-runtime/src/lib.rs

kei-agent-runtime — Agent substrate v1 runtime.

Modules:
- `capability` — Capability trait + context structs + result enums
- `registry`   — static &str → &'static dyn Capability lookup for all 14 impls
- `gates`      — 6 PreToolUse gate capabilities
- `verifies`   — 8 on-return verify capabilities
- `compose`    — task.toml + role + capabilities → prompt.md
- `spawn`      — prepare tasks/<agent-id>/prompt.md + ledger row
- `prepare`    — orchestrator-facing `AgentInvocation` bundle (ergonomics)
- `verify`     — run all verify capabilities against agent's return
- `simulated_merge` — orchestrator-side worktree → apply diff → verify

Per `docs/AGENT-SUBSTRATE-SCHEMA.md` (LOCKED 2026-04-23).

## Related

- parent: `kei-agent-runtime/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
