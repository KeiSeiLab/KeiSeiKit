---
title: portrait.rs
path: kei-cortex/src/handlers/portrait.rs
dna_hash: sha256:74a73a0ae71414ef
language: rust
size_loc: 253
generated: by-keidocs
---

# kei-cortex/src/handlers/portrait.rs

`POST /api/v1/cortex/pet/:user_id/portrait/stylize` — take an uploaded
portrait, run it through fal.ai Flux 2 Pro, clone a bundled Cubism rig,
and swap `texture_00.png` with the stylized image.

Constructor Pattern: the handler body is a 5-step pipeline and each step
is a sibling function <30 LOC. No business logic in the handler body
itself beyond orchestration + timing.

Concurrency: installs are serialised per `user_id` via `AppState::user_lock`
so two concurrent requests for the same user cannot race on the clone /
rename. Different users run in parallel.

## Public API

- Upper bound on short text fields (`style`, `base_model`) — prevents an
- Response body returned to the Setup UI after a successful stylize run.
- Parsed, validated multipart form payload.
- Handler entry point — wired in `routes.rs`.
- Walk the multipart payload, collecting `file` / `style` / `base_model`.
- Read a file field, enforcing MIME prefix and 10 MiB cap.
- Read a short text field, enforcing `MAX_FIELD_BYTES` BEFORE we return
- Call the Flux client, mapping its errors onto 504 / 502 as appropriate.
- Clone the base rig on a blocking thread and return the three URL strings.
- Blocking helper: atomically install the rig AND drop a `preview.png`
- Build the three public URLs. `model_json` is the first `*.model3.json`
- Return the first `*.model3.json` basename inside `dir`, if any.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
