---
title: docs.rs
path: kei-projects-index/src/docs.rs
dna_hash: sha256:38edb701eaa63c15
language: rust
size_loc: 37
generated: by-keidocs
---

# kei-projects-index/src/docs.rs

Documentation-presence detector.

Constructor Pattern: one cube = "does this project have N standard
doc files?". No git2 dep — the dashboard can answer "which repos
lack a CLAUDE.md?" without pulling libgit2 in.

## Public API

- Four-way doc-presence snapshot at the project root.
- Case-insensitive `<root>/<name>` lookup. Probes the supplied form
- `pub fn detect_docs` — Detect CLAUDE.md / DECISIONS.md / RUNBOOK.md / README.md at the

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
