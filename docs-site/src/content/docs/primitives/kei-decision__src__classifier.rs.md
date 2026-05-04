---
title: classifier.rs
path: kei-decision/src/classifier.rs
dna_hash: sha256:5dce696e8b77f489
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-decision/src/classifier.rs

Heuristic classifier — `RawAction.title + severity + effort` → [`ActionKind`].

Pure function; no IO. Keyword match on lowercased title decides; severity
and effort do NOT change the kind today (reserved for future tuning).

## Public API

- `pub fn slug` — Lower-snake form for use in slugs / file names.
- `pub fn classify` — Pure mapping `RawAction → ActionKind`. Order of checks matters: more

## Related

- parent: `kei-decision/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
