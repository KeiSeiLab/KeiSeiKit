---
title: fal.rs
path: kei-cortex/src/fal.rs
dna_hash: sha256:debfeeaaa0df2f3e
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-cortex/src/fal.rs

fal.ai Flux 2 Pro client — stylize a portrait into an anime frame.

Public surface: `stylize(source_png, style)`.

Pipeline: (1) upload source image to fal storage, (2) POST generation
request to the queue endpoint, (3) poll status until `COMPLETED`, (4)
download the first result image.  60-second wall-clock budget; past that
the function returns `Error::Timeout` so the handler can surface HTTP 504.

`FAL_KEY` is read from the environment on every call (env-rotation-friendly,
per RULE 0.8 — never hardcoded, never cached in daemon state).

HTTP pipeline steps live in `fal_pipeline`; SSRF mitigation in `fal_ssrf`.

## Public API

- Style preset — drives the prompt and nothing else.
- `pub fn from_wire` — Parse the wire-level string. Unknown values fall back to `Cute`.
- Errors surfaced to the caller; handlers map them onto HTTP codes.
- Stylize `source_png` into an anime portrait. Returns raw PNG bytes.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
