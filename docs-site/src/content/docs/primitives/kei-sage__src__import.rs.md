---
title: import.rs
path: kei-sage/src/import.rs
dna_hash: sha256:03c083d4711eb335
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-sage/src/import.rs

Obsidian-style vault import: walk a directory, ingest .md files.

Minimal subset of LBM internal/sage/import_obsidian.go — we do NOT parse
frontmatter here (the upstream parser used multiple helper files). Port
of frontmatter/wikilinks parsing is a later milestone; this cube honours
the public interface.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
