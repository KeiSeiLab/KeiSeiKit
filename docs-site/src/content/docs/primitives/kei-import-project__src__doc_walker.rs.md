---
title: doc_walker.rs
path: kei-import-project/src/doc_walker.rs
dna_hash: sha256:9c433c719bf6919d
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-import-project/src/doc_walker.rs

Walk a repo root and collect documentation markdown file paths.

Collects: README.md / README / readme.md at top level + every
docs/**/*.md (skipping _*.md, .git/, target/, node_modules/).

## Public API

- `pub fn collect_doc_paths` — Return ordered list of candidate markdown paths to extract skills from.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: std, tempfile, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
