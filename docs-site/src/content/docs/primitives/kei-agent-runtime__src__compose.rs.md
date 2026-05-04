---
title: compose.rs
path: kei-agent-runtime/src/compose.rs
dna_hash: sha256:14a4a33c4b4c547a
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-agent-runtime/src/compose.rs

Compose capability-fragment prompt for an agent invocation.

Flow:
1. Parse `task.toml` → `TaskSpec` (caller does this).
2. Resolve `_roles/<task.role>.toml` via `role::resolve_role`
(handles `extends` / `relaxes` / cycle detection).
3. For each capability in the resolved required list, read the
`_capabilities/<category>/<slug>/text.md` fragment.
4. Concatenate fragments with `\n\n---\n\n`.
5. Append `task.body.text`.

## Public API

- `pub fn compose_prompt` — Compose prompt text. `kit_root` is the repo root that holds `_roles/`

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
