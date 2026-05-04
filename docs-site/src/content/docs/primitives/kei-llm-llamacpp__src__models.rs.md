---
title: models.rs
path: kei-llm-llamacpp/src/models.rs
dna_hash: sha256:3e9cf391232df952
language: rust
size_loc: 99
generated: by-keidocs
---

# kei-llm-llamacpp/src/models.rs

Models — recursively scan a directory for `.gguf` files.

Default search dirs:
- `~/.cache/llama.cpp/`
- `~/Library/Application Support/llama.cpp/models/` (macOS)

Quant detection is conservative: only well-known patterns map to a
quant string (Q4_0 / Q4_K_M / Q5_K_S / Q6_K / Q8_0 / F16 / F32).
Unknown filenames produce `quant: None`.

## Public API

- One discovered .gguf file.
- `pub fn list_models` — Scan `dir` recursively for .gguf files. Non-existent dir → empty Vec.
- `pub fn default_dirs` — Default search roots that should be probed by the `models` subcommand
- Recursively traverse `dir`, appending .gguf entries to `out`.
- Build a `ModelEntry` from a .gguf path. Returns None on metadata error.
- `pub fn detect_quant` — Conservative quant detection. Returns canonical uppercase form.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, regex, serde, std

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
