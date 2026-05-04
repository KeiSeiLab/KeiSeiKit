---
title: lib.rs
path: kei-llm-llamacpp/src/lib.rs
dna_hash: sha256:bca3037908a59d76
language: rust
size_loc: 32
generated: by-keidocs
---

# kei-llm-llamacpp/src/lib.rs

kei-llm-llamacpp — adapter to llama.cpp via shell-out.

Wave 58 of the local-LLM stack. NO FFI, NO daemon. Each subcommand
is one Constructor-Pattern module:
discovery — find llama-cli / llama-server + version
models    — scan dirs for .gguf, detect quant
generate  — non-streaming inference
stream    — streaming token chunks
server    — spawn llama-server (loopback only)
runner    — Runner trait + RealRunner + ServerHandle
error     — Error enum + exit-code mapping
cli       — clap entry structs

## Public API

- `pub const KEI_WRAPPER_VERSION` — Wrapper version — surfaced by the `version` subcommand.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`

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
