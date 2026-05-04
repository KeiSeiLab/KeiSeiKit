---
title: test_env.rs
path: kei-store/src/test_env.rs
dna_hash: sha256:f4dd19071f4b1a34
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-store/src/test_env.rs

Shared `ENV_LOCK` for kei-store tests that mutate process-wide env vars.

Constructor Pattern: single responsibility — one global `Mutex<()>` that
every test serialising on `KEI_STORE_*` and related env variables takes
before `set_var` / `remove_var`. Prevents the cargo-test default parallel
runner from racing multiple tests on the same env state.

Exposed under `#[cfg(any(test, feature = "s3"))]` so:
- in-crate unit tests (`github.rs`, `s3_cloud/*`) can use it
- the out-of-crate smoke test (`tests/s3_smoke.rs`) can import it via
the `s3` feature gate (same gate the smoke test already sits behind)

NOT exposed in normal release builds — this is a test-only hygiene shim.

## Public API

- `pub fn env_lock` — Take the lock, recovering from a poisoned guard (another test panicked

## Related

- parent: `kei-store/Cargo.toml`
- imports: std

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
