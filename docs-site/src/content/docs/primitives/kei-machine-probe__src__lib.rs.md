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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
