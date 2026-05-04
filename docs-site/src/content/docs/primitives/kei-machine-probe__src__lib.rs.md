---
title: lib.rs
path: kei-machine-probe/src/lib.rs
dna_hash: sha256:d638ed31796e991c
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-machine-probe/src/lib.rs

kei-machine-probe — public library surface (Wave 56).

Foundation for the local-LLM stack: every Wave 57-60 primitive
(ollama / llamacpp / mlx / router) calls `probe()` to know what the
current machine can run before choosing a backend.

Constructor Pattern: each detector lives in its own module and
accepts a `&dyn Runner` so unit tests can substitute a fixture-backed
mock. NO direct `std::process::Command::new` outside `runner.rs`.

## Public API

- `pub fn probe` — One-shot probe — runs every detector and returns a fully populated

## Related

- parent: `kei-machine-probe/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
