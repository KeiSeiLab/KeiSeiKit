---
title: smoke.rs
path: kei-forge/tests/smoke.rs
dna_hash: sha256:2e23f3cb6f127d72
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-forge/tests/smoke.rs

Integration smoke test for kei-forge.

Exercises GET / and POST /forge via `tower::ServiceExt::oneshot` on
the Router — no real socket. With pure-Rust templating, the generator
is hermetic when pointed at a non-existent crate name: it returns a
structured `CrateNotFound` without touching the filesystem, so these
tests can run in any working directory without creating or mutating
real atoms on disk.

Unit tests for the pure-Rust pipeline (happy path, file-exists refuse,
crate-not-found, template-missing) live inside `src/generate.rs` and
its three Constructor-Pattern submodules (placeholders, paths,
rollback) — they use `tempfile::TempDir` for full hermetic runs.

Run with: `cargo test -p kei-forge`

## Public API

- FIX A (DNS rebinding): a POST whose `Host:` header names an attacker
- FIX B (CSRF): `application/x-www-form-urlencoded` is SOP-safe, so a
- FIX C (description injection): a newline in `description` could
- FIX (defence-in-depth): GET / must carry the four hardening

## Related

- parent: `kei-forge/tests`
- imports: axum, kei_forge, serde_json, tower

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
