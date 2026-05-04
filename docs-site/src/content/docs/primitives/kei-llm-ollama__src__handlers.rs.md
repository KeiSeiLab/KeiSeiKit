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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
