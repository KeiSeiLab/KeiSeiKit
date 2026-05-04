---
title: fal_pipeline.rs
path: kei-cortex/src/fal_pipeline.rs
dna_hash: sha256:da4050cce27aa0de
language: rust
size_loc: 173
generated: by-keidocs
---

# kei-cortex/src/fal_pipeline.rs

fal.ai HTTP pipeline steps — upload, enqueue, poll, download.

Each step is a single HTTPS round-trip.  All use the shared `HTTP_CLIENT`
from `http_helpers` (no per-request `reqwest::Client::new()`).
Response bodies are read through `read_capped` so a runaway upstream
cannot allocate unbounded memory.

## Public API

- Step 1 — ask fal storage for a signed PUT URL, then PUT the image to it.
- Step 2 — POST the generation request, return the validated status URL.
- Step 3 — poll `status_url` until `COMPLETED`, or give up at `deadline`.
- Step 4 — download the PNG bytes from a validated fal CDN URL.
- Dig out `images[0].url` from the completed status payload, then validate.
- Decode a fal response into JSON, capping error/success bodies.
- Cap a string at `max` bytes on a char boundary.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
