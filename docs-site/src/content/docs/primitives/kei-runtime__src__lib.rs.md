---
title: lib.rs
path: kei-runtime/src/lib.rs
dna_hash: sha256:424239f05e53377d
language: rust
size_loc: 16
generated: by-keidocs
---

# kei-runtime/src/lib.rs

kei-runtime — atom invocation runtime + schema linter.

Four modules:
- `discover` — walks `<root>/*/atoms/*.md`, parses YAML frontmatter
- `validate` — JSON Schema draft-07 validation of input/output
- `invoke`   — MVP stub: discovers + validates, exec wire-up TBD
- `lint`     — `schema-lint` correctness pass over atom frontmatter

Per `docs/SUBSTRATE-SCHEMA.md` §Runtime invocation contract (LOCKED).

## Related

- parent: `kei-runtime/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
