---
title: router.rs
path: kei-router/src/router.rs
dna_hash: sha256:7d6ce0da39d8dd82
language: rust
size_loc: 157
generated: by-keidocs
---

# kei-router/src/router.rs

Router — holds keyword rules, dispatches queries to tool calls.

## Public API

- Canonical route outcome.
- `pub struct Router` — Router holds the static + dynamic keyword rules.
- `pub fn add_dynamic` — Append user-supplied rules at runtime (domain extension).
- `pub fn route` — Route a natural language query. Always returns a result — falls back to search tools.
- `pub fn route_with_hint` — Convenience wrapper — useful for remote MCP forwarders that want a hint.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
