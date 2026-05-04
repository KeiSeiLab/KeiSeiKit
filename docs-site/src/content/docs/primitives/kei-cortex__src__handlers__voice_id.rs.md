---
title: voice_id.rs
path: kei-cortex/src/handlers/voice_id.rs
dna_hash: sha256:92c9547b1fc22e78
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-cortex/src/handlers/voice_id.rs

Voice-id resolution — pluck `voice.voice_id` out of a pet.toml manifest.

The `kei_pet::Voice` schema does not model `voice_id` as a typed field
(the crate tracks tone / humor only). Rather than widen the schema, we
re-read the manifest TOML here and pick the optional `voice.voice_id`
string directly. Absent / wrong-typed → fall back to the ElevenLabs
"Rachel" default.

## Public API

- `pub const DEFAULT_VOICE_ID` — ElevenLabs "Rachel" — used when the pet manifest has no voice_id.
- Allowed shape for a pet's `voice_id` — limits blast radius if the TOML
- `pub fn resolve` — Load the pet manifest and pluck `voice.voice_id`. 404 if no such pet.
- `pub fn extract` — Dig `voice.voice_id` out of a raw TOML value. Returns `None` on absent /
- `pub fn sanity_check` — Reject voice_ids that look pathological (empty, too long, non-ascii).

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
