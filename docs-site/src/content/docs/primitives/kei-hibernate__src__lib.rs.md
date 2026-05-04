---
title: lib.rs
path: kei-hibernate/src/lib.rs
dna_hash: sha256:712edf7875d98c9a
language: rust
size_loc: 23
generated: by-keidocs
---

# kei-hibernate/src/lib.rs

kei-hibernate — whole-brain export/import for KeiSeiKit state.

Wave 14 primitive. Serialises an entire KeiSei installation
(sqlite stores + capabilities / roles / blocks / agents /
skills / hooks) into a single tar.zst bundle with a SHA-256
manifest, then restores it on another machine.

Public surface kept deliberately small: `export`, `import`,
`inspect`. Each dispatches to a Constructor-Pattern cube.

## Related

- parent: `kei-hibernate/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
