---
title: middleware.rs
path: kei-forge/src/middleware.rs
dna_hash: sha256:6b306966dc21af95
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-forge/src/middleware.rs

HTTP middleware — defence against cross-origin / DNS-rebinding attacks.

Two layers:
- [`require_local_host`] — rejects requests whose `Host:` header is not
exactly `localhost:8747` or `127.0.0.1:8747`. Blocks DNS-rebinding
(attacker points `a.evil.com` → 127.0.0.1 while browser still trusts
the evil.com origin for Same-Origin-Policy purposes).
- [`require_json_content_type`] — rejects `POST /forge` unless body is
`application/json`. Blocks CSRF via `<form>` submissions: urlencoded
POSTs are SOP-safe (no preflight), but JSON bodies trigger CORS
preflight so SOP engages.

Both are advisory: they compose via `axum::middleware::from_fn` and
never touch application state.

## Public API

- Reject requests whose `Host:` is not an exact allow-list match.
- Reject POSTs whose `Content-Type` is not `application/json`.

## Related

- parent: `kei-forge/Cargo.toml`
- imports: axum, tower

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
