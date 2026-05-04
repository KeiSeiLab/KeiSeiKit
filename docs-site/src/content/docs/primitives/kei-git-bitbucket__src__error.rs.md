---
title: error.rs
path: kei-git-bitbucket/src/error.rs
dna_hash: sha256:8bb29f234dcdb698
language: rust
size_loc: 64
generated: by-keidocs
---

# kei-git-bitbucket/src/error.rs

Local error type for the Bitbucket backend.

Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
impls can use `?` against the runtime-core `Result`.

## Public API

- `pub type Result` — Crate-local result alias.
- Crate-local error variants.
- Transport / TLS / timeout failure from `reqwest`.
- Non-success HTTP status with the (best-effort) body text.
- DNA construction or parse failure.
- Local IO (env var read, git CLI invocation, etc.).
- JSON serialize / deserialize failure.
- Resource lookup miss (e.g. 404 on get repo / branch).
- Missing or malformed configuration (env var, remote URL, etc.).
- Authentication failure (401/403).

## Related

- parent: `kei-git-bitbucket/Cargo.toml`
- imports: kei_runtime_core, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
