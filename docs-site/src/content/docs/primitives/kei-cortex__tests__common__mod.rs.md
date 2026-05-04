---
title: mod.rs
path: kei-cortex/tests/common/mod.rs
dna_hash: sha256:eb8ded12b5966680
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-cortex/tests/common/mod.rs

Shared test harness: spins up the router on an ephemeral port and hands
back the base URL + bearer token + config to the test body.

Every integration-test file includes this module with `mod common;`, so
items unused by one file still count as live via the others. The
`#![allow(dead_code)]` silences per-file false positives.

## Public API

- `pub const MINIMAL_PET_TOML` — Minimal valid pet.toml used by multiple tests.
- `pub struct TestServer` — Handle returned to each test; dropping stops the server.
- Spin up the router on 127.0.0.1 with a random port.
- `pub fn write_minimal_pet` — Write a minimal pet.toml for `user_id` under `<pet_root>/<user_id>.toml`.
- `pub fn async_client` — Build an async reqwest client.
- `pub struct MockAnthropicServer` — Handle to the process-wide mock Anthropic upstream. The server runs
- `pub fn uri` — Base URL of the mock (`http://127.0.0.1:<port>/v1/messages`).
- Build the canned-reply axum router used by the mock. Same body for
- Spin up the mock listener on a dedicated thread+runtime, return the
- `pub fn mock_anthropic_responding_with` — Per-call mock variant. Spawns a fresh dedicated-thread mock so every
- `pub fn shared_mock_anthropic` — Process-wide shared mock Anthropic server. Initialised on first call

## Related

- parent: `kei-cortex/tests/common`
- imports: axum, kei_cortex, std, tempfile, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
