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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
