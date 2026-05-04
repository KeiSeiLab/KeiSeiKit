---
title: drive.rs
path: kei-spawn/src/drive.rs
dna_hash: sha256:e5fa35544ab271a6
language: rust
size_loc: 148
generated: by-keidocs
---

# kei-spawn/src/drive.rs

drive — driver trait + shared types + ManualDriver for `kei-spawn drive`.

The `drive` subcommand is the one-call replacement for the current
two-step dance (`kei-spawn spawn` → orchestrator pastes Agent invocation).

Two drivers live here:
- `ManualDriver` — always returns `NotImplemented` (v0.1 default path).
- `HttpDriver`   — real impl lives in `drive_http` behind feature
`http-driver`; without the feature a stub returning
`NotImplemented` preserves the v0.1 API surface.

Exit-code contract (mirrors `kei-runtime::InvokeError::NotImplemented`):
- 64 (EX_USAGE range) when the driver yields `NotImplemented`
- 1 on spawn failure (same as `kei-spawn spawn`)
- 0 only when a real driver returns Ok

Constructor Pattern: one trait + two zero-state impls + one helper fn.

## Public API

- Success envelope for the `HttpDriver` (and the contract
- Errors surfaced from driver invocation.
- `pub trait AnthropicDriver` — Abstraction over "how does an agent invocation actually happen."
- `pub struct ManualDriver` — v0.1 driver — returns `NotImplemented` unconditionally.
- Stub `HttpDriver` used when the `http-driver` feature is OFF.
- Re-export real `HttpDriver` when feature is ON.
- `pub fn not_implemented_message` — Canonical stderr message for the v0.1 stub.
- `pub fn drive_with` — Drive helper — orchestrator-facing entry that dispatches to a driver.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
