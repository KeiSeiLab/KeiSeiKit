---
title: role.rs
path: kei-agent-runtime/src/role.rs
dna_hash: sha256:00c96b40e8c3db66
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-agent-runtime/src/role.rs

Role expression resolver — Layer E.

Parses `_roles/<name>.toml` and resolves `extends` chains with `relaxes`
subtraction, emitting a flat `ResolvedRole` for downstream consumers
(`compose`, `prepare`, `verify`, `dna`).

Semantics:
- `extends` — optional parent role slug; loaded recursively.
- `required` (local) — merged on top of parent's resolved required.
- `relaxes` — slugs in parent's resolved required to DROP. A warning is
collected in `ResolvedRole::warnings` if a relaxed cap wasn't present
in the inherited set (caller decides how to surface).
- Cycle detection — visited set passed down the recursion; an error
with a clear path is returned when a cycle is found.
- Depth cap — `extends` chains deeper than `MAX_DEPTH = 16` are
refused (`RoleError::MaxDepthExceeded`) to prevent stack overflow
on malformed/hostile role trees.
- Name validation — role slug must match `^[a-z][a-z0-9-]{0,63}$`,
blocks `../../etc/passwd` path traversal before the `join`.

Constructor Pattern: one cube = one responsibility (role expression only).
No I/O beyond `std::fs::read_to_string`. Dispatched from `compose::load_role`
and `verify::load_role_capabilities` so both share the same semantics.

## Public API

- `pub const MAX_DEPTH` — Max depth for `extends` chain traversal. Guards against stack overflow
- Role / capability slug pattern. Lowercase start, `[a-z0-9-]` body,
- Structured errors from role resolution.
- Flattened role ready for downstream composition.
- Ordered capability names after `extends` merge + `relaxes` subtraction.
- Non-fatal advisories surfaced during resolution (e.g. relaxed cap
- Deserialized role file (raw shape, pre-resolution).
- `pub fn validate_name` — Validate a role-or-capability slug; returns typed error if malformed.
- `pub fn resolve_role` — Resolve a role by slug; read role file, walk `extends`, apply `relaxes`.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, once_cell, regex, serde, std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
