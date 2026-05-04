---
title: http_driver.rs
path: kei-spawn/tests/http_driver.rs
dna_hash: sha256:df2510e3bd2541c6
language: rust
size_loc: 244
generated: by-keidocs
---

# kei-spawn/tests/http_driver.rs

http_driver — end-to-end tests for the `http-driver` feature.

Uses `httpmock` to stand up a local HTTP server and `KEI_ANTHROPIC_ENDPOINT`
to redirect the driver at it. `KEI_ANTHROPIC_KEY` is set per-test so the
tests never require real credentials.

Every test is self-contained: fresh MockServer + per-test env vars. The
env_lock mutex below ensures concurrent tests don't trample each other's
process-global env.

## Public API

- Cargo test harness runs tests in parallel by default — env vars are
- Oversize response body must be rejected with a Transport error
- Body just under the 10 MiB cap must succeed through the parse stage

## Related

- parent: `kei-spawn/tests`
- imports: httpmock, kei_spawn, std

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
