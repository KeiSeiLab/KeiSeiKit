---
title: openai_compat.rs
path: kei-cortex/tests/openai_compat.rs
dna_hash: sha256:4071287c7cec81ef
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-cortex/tests/openai_compat.rs

Integration tests for the OpenAI-compatible /v1/* surface.

Exercises the wire format end-to-end against an in-process axum
`Router` via `tower::ServiceExt::oneshot`. Bypasses the HTTP socket
so tests are fast and deterministic. Streaming endpoints are
exercised by reading the response body to bytes and asserting the
presence of the expected SSE frames.

Upstream Anthropic traffic is diverted to a process-wide axum mock
served by `common::shared_mock_anthropic`. The mock runs on a
dedicated OS-thread runtime so it outlives every `#[tokio::test]`
runtime in the binary; tests set `ANTHROPIC_ENDPOINT` to its URI
(idempotent across tests because the singleton URI never changes).
`anthropic::endpoint()` reads the env at call time, so the redirect
is picked up without any source-side wiring.

## Public API

- Set `KEI_API_KEY` for tests so the auth middleware uses the bearer

## Related

- parent: `kei-cortex/tests`
- imports: axum, kei_cortex, std, tower

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
