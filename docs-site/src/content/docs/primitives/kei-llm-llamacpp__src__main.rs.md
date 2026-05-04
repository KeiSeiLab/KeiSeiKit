---
title: main.rs
path: kei-llm-llamacpp/src/main.rs
dna_hash: sha256:03880fc80cb77806
language: rust
size_loc: 17
generated: by-keidocs
---

# kei-llm-llamacpp/src/main.rs

kei-llm-llamacpp — CLI dispatcher (thin).

Each subcommand maps to one helper in `dispatch.rs`. Errors flow
through `Error::exit_code()` so the harness sees the canonical
exit-code surface.

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: clap, kei_llm_llamacpp, std

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
