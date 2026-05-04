---
title: drive_http_parse.rs
path: kei-spawn/src/drive_http_parse.rs
dna_hash: sha256:a3b7891e987a4857
language: rust
size_loc: 185
generated: by-keidocs
---

# kei-spawn/src/drive_http_parse.rs

drive_http_parse — request / response DTOs for Anthropic `/v1/messages`.

Kept in its own module so the `drive_http` HTTP glue stays under the
Constructor Pattern ≤200 LOC budget and the DTO surface is unit-testable
without a live reqwest client.

## Public API

- `pub const MODEL_ID` — Model id used for every `kei-spawn drive` request.
- `pub const MAX_TOKENS` — max_tokens limit per Anthropic spec (plenty for report envelopes).
- `pub const ANTHROPIC_VERSION` — Anthropic API version header value.
- `pub const DEFAULT_ENDPOINT` — Default endpoint; overridable via `KEI_ANTHROPIC_ENDPOINT` for tests.
- Outbound POST body.
- Inbound response shape.
- `pub fn to_agent_result` — Fold the parsed response into the public `AgentResult` envelope.
- `pub fn build_preamble` — Build the `[kei-spawn routing] …` preamble required by the task spec.
- `pub fn compose_user_content` — Build the full user message (preamble + prompt).
- `pub fn parse_response` — Parse a JSON response body. Errors map to `Transport` with the
- `pub fn excerpt` — Truncate `s` to at most `n` bytes at a char boundary.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: crate, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
