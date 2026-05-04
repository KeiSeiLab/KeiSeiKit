---
title: persona.rs
path: kei-cortex/src/persona.rs
dna_hash: sha256:81c58635bc27f507
language: rust
size_loc: 79
generated: by-keidocs
---

# kei-cortex/src/persona.rs

Load a pet manifest from disk and build the system prompt for a chat turn.

Delegates to `kei_pet::parse` (TOML + validation) and
`kei_pet::system_prompt` (overlay render). Adds the chat-specific
"respond naturally, 1-3 sentences" tail so the response-length hint is
not baked into the persona but is owned by this transport.

Manifest path: `<pet_root>/<user_id>.toml` — matches the flat layout
used elsewhere in this daemon (see `handlers/pet.rs`,
`config::AppConfig::pet_root`).

## Public API

- `pub const CHAT_TAIL` — Tail appended to the persona overlay. Chat-transport-owned, not persona-owned.
- `pub fn load_manifest` — Load `<pet_root>/<user_id>.toml` → `PetManifest`. 404 if absent.
- `pub fn build_system_prompt` — Render the system prompt a chat turn sends upstream.
- `pub fn load_and_render` — Convenience: load manifest, render system prompt, return both.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, kei_pet, std

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
