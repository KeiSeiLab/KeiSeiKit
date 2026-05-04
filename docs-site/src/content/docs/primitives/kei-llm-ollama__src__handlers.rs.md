---
title: handlers.rs
path: kei-llm-ollama/src/handlers.rs
dna_hash: sha256:4dd3ecc8e6204d9d
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-llm-ollama/src/handlers.rs

CLI command handlers — one async fn per subcommand.

Each handler returns `Result<(), ApiError>`; `main` maps the error to an
exit code via `ApiError::exit_code`.

## Related

- parent: `kei-llm-ollama/Cargo.toml`
- imports: crate, futures, std

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
