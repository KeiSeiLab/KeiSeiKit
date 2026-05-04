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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
