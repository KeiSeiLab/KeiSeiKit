---
title: new_project.rs
path: kei-decompose/src/parsers/new_project.rs
dna_hash: sha256:71b4ef9935c9ac7e
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-decompose/src/parsers/new_project.rs

/new-project phases adapter.

Detects:
- One or more `## Phase N` headings (`## Phase 1: scaffold`, etc.)
- Often combined with `## Verification` / `## Output` per phase.

Extracts:
- One Action per `## Phase N` heading. The Action title is the phase
summary; the body is the phase content up to the next phase or EOF.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, regex, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
