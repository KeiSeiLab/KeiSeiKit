---
title: jsonmcp.rs
path: keisei/src/adapters/jsonmcp.rs
dna_hash: sha256:7911a87239e0af44
language: rust
size_loc: 107
generated: by-keidocs
---

# keisei/src/adapters/jsonmcp.rs

Shared JSON MCP-server merge helpers for JSON-keyed adapters.

Constructor Pattern: single responsibility — own the "load → upsert /
remove under a named outer key → atomic write" pipeline that every
JSON-keyed adapter (claude-code, cursor, zed) was duplicating in
~95%-identical form. Continue is YAML-based and does NOT use this.

Error surfacing is uniform across the three callers: JSON parse
failures flow through `Error::ConfigParseError` rather than the raw
serde_json error (zed was already doing this before the dedup; the
other two silently converted via `#[from]` and lost the config path).

## Public API

- `pub fn load_json_or_empty` — Load a JSON document from disk, returning `{}` for a missing or
- `pub fn build_mcp_entry` — Build the MCP-server entry shape used by every JSON-keyed adapter.
- `pub fn upsert_under_key` — Upsert `{entry_key: new_entry}` under `doc[outer_key]`. If the outer
- `pub fn remove_under_key` — Remove `doc[outer_key][entry_key]` and prune `outer_key` when it's
- `pub fn persist` — Atomically persist the document to the target path.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
