---
title: client.rs
path: kei-git-bitbucket/src/client.rs
dna_hash: sha256:dc26854e4413556a
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-git-bitbucket/src/client.rs

Thin async REST 2.0 client for Bitbucket Cloud.

No upstream Rust SDK is used — we hit the public surface directly
(`https://api.bitbucket.org/2.0`) with HTTP Basic auth read from
`BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD`. Base URL is overridable
for `wiremock` tests via `BITBUCKET_URL`.

## Public API

- `pub const DEFAULT_BASE_URL` — Default REST root.
- `pub const DEFAULT_TIMEOUT_SECS` — Per-request timeout.
- Subset of the Bitbucket repository object we depend on.
- Subset of the branch ref object we depend on.
- Body for POST /repositories/{ws}/{slug}.
- REST client. Cheap to clone (`Arc` inside `reqwest::Client`).
- `pub fn new` — Build with explicit credentials + base URL (use [`DEFAULT_BASE_URL`] in prod).
- `pub fn from_env` — Read `BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD` (and optional
- `pub fn with_url` — Override base URL (for wiremock tests).
- `pub fn base_url` — Accessor for the configured base URL.
- GET /repositories/{workspace}/{repo_slug} — `Ok(true)` on 200,
- POST /repositories/{workspace}/{repo_slug} with `{scm:"git", is_private:true}`.
- GET /repositories/{ws}/{slug}/refs/branches/{branch} — branch SHA.

## Related

- parent: `kei-git-bitbucket/Cargo.toml`
- imports: base64, crate, reqwest, serde, std

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
