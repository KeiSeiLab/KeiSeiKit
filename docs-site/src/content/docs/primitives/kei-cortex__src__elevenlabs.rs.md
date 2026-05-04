---
title: elevenlabs.rs
path: kei-cortex/src/elevenlabs.rs
dna_hash: sha256:93b6fd4d4c57d0ba
language: rust
size_loc: 149
generated: by-keidocs
---

# kei-cortex/src/elevenlabs.rs

ElevenLabs TTS client — `POST /v1/text-to-speech/{voice_id}`.

Public surface: a single async function `synthesize(voice_id, text)` that
returns the raw mp3 bytes from ElevenLabs. Uses `eleven_turbo_v2_5` with
stability=0.5 / similarity_boost=0.75 — these values are the documented
middle-of-the-road defaults per the ElevenLabs API reference.

`ELEVENLABS_API_KEY` is read from the environment on every call so the
user may rotate it without restarting the daemon (RULE 0.8 — env by
reference, never hardcoded). We never log the full text; only its length
and the response status / byte count.

## Public API

- Errors surfaced to the caller; handlers map them onto HTTP codes.
- Cap on successful audio response bodies (64 MiB).
- Cap on error response bodies (16 KiB).
- Synthesize speech for `text` using the given `voice_id`. Returns mp3 bytes.
- POST the JSON body and collect the audio bytes. Split from `synthesize`
- Build the JSON request body. Voice settings are the documented API
- Turn an ElevenLabs response into raw mp3 bytes; map non-2xx to `Upstream`.
- Cap a string at `max` bytes on a char boundary. Used for error previews

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
