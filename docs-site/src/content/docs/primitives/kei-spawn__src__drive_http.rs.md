---
title: drive_http.rs
path: kei-spawn/src/drive_http.rs
dna_hash: sha256:2184a4c731ae746f
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-spawn/src/drive_http.rs

drive_http — reqwest::blocking-backed Anthropic driver.

Gated behind the `http-driver` Cargo feature. Reads `KEI_ANTHROPIC_KEY`
at every `invoke` call (so key rotation takes effect without rebuilds).

Endpoint defaults to <https://api.anthropic.com/v1/messages> and can be
overridden via `KEI_ANTHROPIC_ENDPOINT` (test hook for httpmock).

Constructor Pattern: one struct + one impl + small helpers, every fn
≤30 LOC, file ≤200 LOC.
Unit-level tests for helpers. End-to-end tests (with httpmock)
live in `tests/http_driver.rs`.

## Public API

- Cap response body at 10 MiB to mitigate memory-DoS from an oversized
- `pub struct HttpDriver` — Real Anthropic-backed driver. Zero-state: key + endpoint read per call.
- Read response body with a hard cap of `MAX_BODY_BYTES`.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
