---
title: lint.rs
path: kei-runtime/src/lint.rs
dna_hash: sha256:77d33bba92627cfd
language: rust
size_loc: 213
generated: by-keidocs
---

# kei-runtime/src/lint.rs

`schema-lint` — correctness pass over every `atoms/*.md` under `<root>`.

Checks (from SUBSTRATE-SCHEMA §Validation):
1. Frontmatter has required fields (atom, kind, version, input, output,
side_effects, idempotent, stability).
2. Schema paths resolve to existing JSON files inside the atom's dir
(safe_join — rejects `..` and absolute paths).
3. JSON Schemas declare draft-07 via `$schema`.
4. `kind` ∈ {command, query, stream, transform}.
5. `side_effects` entries are `{op, domain}` objects.
6. `related` wikilinks point to another atom OR `rules/...` (dangling rule
refs allowed).

## Public API

- `pub fn schema_lint` — Run the full lint over `<root>/*/atoms/*.md`.
- Exact-match check for the draft-07 meta-schema URI.
- Strict `[[...]]` wikilink parse.

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: crate, kei_atom_discovery, serde_yaml_ng, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
