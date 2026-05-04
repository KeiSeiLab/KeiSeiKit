---
title: stt.rs
path: kei-cortex/src/handlers/stt.rs
dna_hash: sha256:3847450d0644c166
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-cortex/src/handlers/stt.rs

`POST /api/v1/cortex/stt` — multipart audio → Whisper → JSON transcript.

Constructor Pattern: the handler body is a 3-step pipeline (parse form,
call upstream, wrap JSON), each step a sibling function <30 LOC. We
enforce a 25 MiB per-field cap (OpenAI's documented Whisper limit) and
whitelist the MIME prefixes we send to Whisper's ffmpeg layer.

## Public API

- OpenAI Whisper's own documented cap on the request audio field.
- Wire-level response body for a successful transcription.
- Parsed, validated multipart form payload.
- Handler entry point — wired in `routes.rs`.
- Walk the multipart payload, collecting the `audio` field.
- Read the `audio` field, enforcing MIME prefix and 25 MiB cap.
- Whitelist the MIME prefixes Whisper's ffmpeg layer can demux reliably.
- Call local faster-whisper worker, mapping its errors onto HTTP statuses.

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
