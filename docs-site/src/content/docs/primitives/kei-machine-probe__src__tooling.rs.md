---
title: tooling.rs
path: kei-machine-probe/src/tooling.rs
dna_hash: sha256:7d5fcf71c3db4bc0
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-machine-probe/src/tooling.rs

Local-LLM tooling detection.

For each binary of interest:
1. `which <bin>` — present? (non-zero exit ⇒ absent)
2. `<bin> --version` — extract version string (best-effort).

Bins probed: `ollama`, `brew`, `llama-server` (llama.cpp's HTTP
daemon — the binary kei-llm-llamacpp will spawn). All optional —
every detector returns `None` on failure rather than erroring.

## Public API

- Pull a version token from `<bin> --version` output. Tries, in order:

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate, regex

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
