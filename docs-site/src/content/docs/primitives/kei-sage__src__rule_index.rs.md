---
title: rule_index.rs
path: kei-sage/src/rule_index.rs
dna_hash: sha256:ab9f51e253ca5793
language: rust
size_loc: 129
generated: by-keidocs
---

# kei-sage/src/rule_index.rs

Rule discovery + indexing for kei-sage.

Walks a flat `<rules-root>/*.md` tree (e.g. `~/.claude/rules/`), extracts
the first `#` heading from each file as the rule name, and uses the file
stem as the rule slug. Rules are persisted as Units with:
- `unit_type = "rule"`
- `vault_path = "rule:<slug>"`
- `title = <heading>`

Edges from atoms that `related:` a `[[rules/...]]` wikilink are persisted
with `edge_type = "rule_ref"` via `index_rule_edges`.

Scope: flat dir only (rules live flat in `~/.claude/rules/`). No recursion.

## Public API

- One discovered rule: slug (file stem), display name (`# heading`), md path.
- `pub fn discover_rules` — Walk `<root>/*.md` (no recursion) and parse each file's first `#` heading.
- Extract the first `# ` heading line, stripping the `#` prefix and trim.
- `pub fn index_rules` — Persist rule units into the store. Returns the number of units indexed.
- `pub fn index_rule_edges` — Walk every atom's `related:` list. For every wikilink that classifies as

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate, kei_atom_discovery, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
