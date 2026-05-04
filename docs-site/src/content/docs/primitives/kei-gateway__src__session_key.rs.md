---
title: session_key.rs
path: kei-gateway/src/session_key.rs
dna_hash: sha256:50a7724591843d7f
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-gateway/src/session_key.rs

Deterministic session-key construction.

Port of Hermes `gateway/session.py:build_session_key` (572-637) with the
KeiSei addition of an opt-in blake3 hash for keys exceeding a length floor —
storage layers can index either form.

## Public API

- Tunables forwarded from `GatewayConfig`.
- In group chats, prefix with the participant ID so each user gets an
- In threads, ALSO isolate per user (Hermes default = false: threads are
- Optional logical agent name. Defaults to `"main"`.
- `pub fn build_session_key` — Build a deterministic session key from a [`SessionSource`].
- DM key — chat_id+thread_id+platform-specific normalisation.
- Group / channel key — supports per-user isolation and thread shaping.
- Threads default to shared sessions; per-user only when explicitly enabled.
- WhatsApp JID/LID canonicalisation — mirrors Hermes
- `pub fn hash_session_key` — blake3 hash a key (hex-encoded). Useful for fixed-length DB indices.

## Related

- parent: `kei-gateway/Cargo.toml`
- imports: crate

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
