---
title: error.rs
path: kei-auth-apple/src/error.rs
dna_hash: sha256:4e1198046b1920d9
language: rust
size_loc: 73
generated: by-keidocs
---

# kei-auth-apple/src/error.rs

Local error type for the Apple Sign-In auth provider.

Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
impls can use `?` against the runtime-core `Result`.

## Public API

- `pub type Result` — Crate-local result alias.
- Crate-local error variants.
- Transport / TLS / timeout failure from `reqwest`.
- Non-success HTTP status with the (best-effort) body text, or
- id_token shape / base64 / utf8 / json failure during unverified decode.
- ES256 signature verification against Apple JWKS failed, or a required
- id_token decoded but a required claim (e.g. `sub`) was missing.
- DNA construction or parse failure.
- Local IO (env var read, etc.).
- JSON serialize / deserialize failure.

## Related

- parent: `kei-auth-apple/Cargo.toml`
- imports: kei_runtime_core, thiserror

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
