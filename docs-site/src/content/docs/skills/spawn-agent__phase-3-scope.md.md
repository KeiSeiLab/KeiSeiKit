---
title: phase-3-scope
path: spawn-agent/phase-3-scope.md
dna_hash: sha256:3696ac3de26977b3
language: markdown
size_loc: 163
generated: by-keidocs
---

# spawn-agent/phase-3-scope.md

## Public API

- `Phase 3 — Scope (files whitelist + optional denylist)` — > Goal: produce a concrete `WHITELIST` (glob patterns the agent may touch)
- `3.a — First AskUserQuestion: scope preset` — Send ONE `AskUserQuestion` call. Presets cover ≥80% of real invocations;
- `3.b — Resolve preset to `WHITELIST`` — - **Single crate (Rust)** → follow up with ONE free-text prompt: `Crate name?`
- `3.c — Glob validation rules` — Apply to every pattern in `WHITELIST`:
- `3.d — Second AskUserQuestion: explicit denylist?` — Send the second `AskUserQuestion` call:
- `3.e — Verify criterion` — - `WHITELIST` is a non-empty list (length ≥ 1).
- `3.f — Failure paths (NO DOWNGRADE)` — If the user cannot choose a preset and Custom produces invalid globs twice:

## Related

- parent: `spawn-agent`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
