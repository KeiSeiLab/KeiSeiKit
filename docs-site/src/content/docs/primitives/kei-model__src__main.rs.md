---
title: main.rs
path: kei-model/src/main.rs
dna_hash: sha256:b1bb369fcd5af890
language: rust
size_loc: 194
generated: by-keidocs
---

# kei-model/src/main.rs

kei-model CLI entry. Dispatches to one handler per subcommand. Each
handler stays ≤30 LOC by delegating to library functions.

Exit codes:
0 — success
1 — file/IO error
2 — not-found / no-match / unknown id
3 — cycle in fallback chain

## Related

- parent: `kei-model/Cargo.toml`
- imports: clap, kei_model, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
