---
title: collector.rs
path: kei-hibernate/src/collector.rs
dna_hash: sha256:ece50c2c3b5c1fa0
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-hibernate/src/collector.rs

File collector — enumerates what goes into a hibernate bundle.

Rules:
1. All `*.sqlite` under `~/.claude/agents/` and `~/.claude/memory/`
(when those paths live inside `kit_root`).
2. Entire trees under `_capabilities/`, `_roles/`, `_blocks/`,
`_agents/`, `skills/`, `hooks/` at `kit_root`.

Anything else is excluded. Symlinks are not followed.

## Public API

- `pub const KIT_SUBTREES` — Top-level directories inside `kit_root` that are included wholesale.
- `pub const SQLITE_SUBPATHS` — Sub-paths inside `~/.claude/` that contribute `*.sqlite` files.
- `pub fn collect` — Walk `kit_root`, returning every file eligible for the bundle.
- Recursively collect every file under `kit_root/subtree_name`.
- Collect only `*.sqlite` files under `kit_root/sub`.
- Generic recursive walker. `filter` decides inclusion per file.
- Convert absolute path → bundle-relative (forward slash) and push.

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
