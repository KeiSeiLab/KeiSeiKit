---
title: emit.rs
path: kei-arch-derive/src/emit.rs
dna_hash: sha256:6033db78ba6ab37f
language: rust
size_loc: 153
generated: by-keidocs
---

# kei-arch-derive/src/emit.rs

PLAN.toml serializer — deterministic, sorted, inline-evidence form.

Modules sorted by `id`. Claims within a module sorted by `id`. Each
claim's evidence is rendered as an inline TOML table to match the
shape of the hand-written `arch/PLAN.toml` shipped in Phase 1.

Constructor Pattern: pure projection from `DerivedPlan` to a string.
Inline-evidence rendering lives in `serialize.rs`. The atomic-write
helper here is the one I/O cube.

## Public API

- Final emit-ready plan: meta header + sorted modules + sorted claims.
- `pub fn derive_plan` — Build a `DerivedPlan` from formula declarations. v0.1 wiring: each
- `pub fn render_plan_string` — Render the plan to a TOML string in the inline-evidence shape used by
- `pub fn emit_plan` — Atomic write to `target`. Writes to a sibling `.<name>.tmp` then

## Related

- parent: `kei-arch-derive/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
