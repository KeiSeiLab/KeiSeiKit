---
title: pattern_gate.rs
path: kei-agent-runtime/src/gates/pattern_gate.rs
dna_hash: sha256:0752219c845449be
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-agent-runtime/src/gates/pattern_gate.rs

Generic pattern-based gate (Layer C convergence, 2026-04-23). Absorbs
5 of 6 gate impls as `PatternGate` consts. `tools::deny-tools` stays
separate (tool-name match). Hardening (audit 2026-04-23): H1 regex
cache `Mutex`→`RwLock` (per-pattern `Lazy` needs sibling-gate edits,
out of scope). H2 `compile_checked()` → `Result`, no panics.
H3 `AllowIfMatch`+task-scope fails closed. S4 `char`-based truncation.
L2 single-pass template render; no replace-chain bleed.

## Public API

- How the gate decides on a match.
- Static regex list or dynamic glob list pulled from TaskSpec scope.
- `pub struct PatternGate` — Generic pattern-driven PreToolUse gate.
- First matching raw pattern, or `Err(Deny)` on compile failure (H2).
- Single-pass template render (L2). Tokens: `{name}` `{path}` `{cmd}`
- Compile + cache a regex (H1 + H2). `RwLock` cache: read-lock fast
- Truncate to `max` chars (S4). Safe for multi-byte code points.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, once_cell, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
