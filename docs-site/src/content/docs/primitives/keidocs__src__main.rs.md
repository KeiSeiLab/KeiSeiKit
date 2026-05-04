---
title: main.rs
path: keidocs/src/main.rs
dna_hash: sha256:f548c532ed42ee5e
language: rust
size_loc: 167
generated: by-keidocs
---

# keidocs/src/main.rs

keidocs CLI — extract / validate per-file markdown docs.

## Public API

- Walk source tree under --root and emit one .md per source file in --out.
- Verify each .md in --out has dna_hash + parent backlink.

## Related

- parent: `keidocs/Cargo.toml`
- imports: anyhow, clap, keidocs, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
