---
title: secrets_handler.rs
path: kei-registry/src/secrets_handler.rs
dna_hash: sha256:911e10f97a0fd6d0
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-registry/src/secrets_handler.rs

Handler for the `secrets` subcommand.

Constructor Pattern: this cube owns the secrets command dispatch only.
Env-file resolution + report output. No scanner logic — that lives
in `secrets.rs`.

## Public API

- `pub fn handle_secrets` — Top-level handler wired from `handlers::dispatch`.
- Resolve default env files when user provides none.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

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
