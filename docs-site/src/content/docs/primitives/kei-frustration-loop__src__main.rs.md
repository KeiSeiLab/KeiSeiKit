---
title: main.rs
path: kei-frustration-loop/src/main.rs
dna_hash: sha256:92cee33733f73ef2
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-frustration-loop/src/main.rs

kei-frustration-loop — per-user frustration learning loop binary.

Five subcommands: bootstrap / nightly-scan / feedback / auto-train /
personalize. All work happens in cubes; this file dispatches only.

Constructor Pattern: main.rs only routes parsed args to `cli::dispatch`.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: clap, kei_frustration_loop, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
