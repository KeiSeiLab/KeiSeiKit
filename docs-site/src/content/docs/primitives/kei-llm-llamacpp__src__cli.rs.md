---
title: cli.rs
path: kei-llm-llamacpp/src/cli.rs
dna_hash: sha256:f846a008bab41085
language: rust
size_loc: 70
generated: by-keidocs
---

# kei-llm-llamacpp/src/cli.rs

CLI — clap structs for the 5 subcommands.

Subcommands:
probe                  — discover binaries, emit JSON
models [--dir <path>]  — list .gguf files in directory tree
generate ...           — shell to llama-cli, emit Response (or NDJSON Chunks)
server   ...           — spawn llama-server, emit ServerInfo
version                — emit { llama_cli_version, llama_server_version, kei_wrapper_version }

## Public API

- Locate llama-cli / llama-server on PATH and emit JSON BinPaths.
- List .gguf files. Default: ~/.cache/llama.cpp + macOS app-support.
- Override search directory (recursive scan).
- Shell to llama-cli. Without --stream emits one Response JSON.
- Path to .gguf model file.
- Prompt text.
- Cap on generated tokens.
- Sampling temperature (omit to use llama-cli default).
- Stream tokens line-by-line as NDJSON.
- Spawn llama-server and emit JSON ServerInfo.
- Emit JSON {llama_cli_version, llama_server_version, kei_wrapper_version}.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: clap, std

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
