---
title: tts.rs
path: kei-cortex/src/handlers/tts.rs
dna_hash: sha256:78becc4167bd115e
language: rust
size_loc: 125
generated: by-keidocs
---

# kei-cortex/src/handlers/tts.rs

`POST /api/v1/cortex/pet/:user_id/tts` — JSON text → ElevenLabs → mp3.

Constructor Pattern: the handler body is a 4-step pipeline (validate,
resolve voice_id, call upstream, wrap response), each step a sibling
function <30 LOC. Voice id comes from `voice.voice_id` in the pet
manifest; absent → ElevenLabs "Rachel" default (see `voice_id` module).

## Public API

- ElevenLabs charges by characters — we enforce a 3000-char ceiling.
- Wire-level request body.
- Handler entry point — wired in `routes.rs`.
- Enforce non-empty text and the 3000-char ceiling.
- Call ElevenLabs, mapping its errors onto 504 / 502 / 500 as appropriate.
- Wrap mp3 bytes in an `audio/mpeg` HTTP response.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde

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
