---
title: manifest.rs
path: keidna-sign/src/manifest.rs
dna_hash: sha256:029e735712a750bd
language: rust
size_loc: 179
generated: by-keidocs
---

# keidna-sign/src/manifest.rs

DNA manifest schema + compute / read / write / verify.

Aggregate `dna_hash` is sha256 over a canonical concatenation of:
name | version | sorted(file_path:sha256) | sorted(deps)
Order-independent in deps; deterministic across machines.

## Related

- parent: `keidna-sign/Cargo.toml`
- imports: anyhow, serde, sha2, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
