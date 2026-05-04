---
title: main.rs
path: kei-decompose/src/main.rs
dna_hash: sha256:a6b51168bafadfd3
language: rust
size_loc: 181
generated: by-keidocs
---

# kei-decompose/src/main.rs

kei-decompose CLI entry — clap dispatch only.

All real work lives in module entrypoints. This file's only job is to
convert clap matches → module call → exit code.

Exit codes:
0  success
1  file/IO error
2  no parser detected / parse error
3  kei-spawn invocation failed

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: clap, kei_decompose, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
