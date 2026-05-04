---
title: model.rs
path: kei-model/src/model.rs
dna_hash: sha256:e7c3b495ba02c0a4
language: rust
size_loc: 216
generated: by-keidocs
---

# kei-model/src/model.rs

Core types: `Model`, `Provider`, `Capability`, `Status`.

Constructor Pattern: each enum is its own type with a single responsibility
(rendering / parsing / matching). Pricing lives in a sibling module so this
file stays focused on identity + capability shape.

## Public API

- One row in the model catalog.
- Canonical id, e.g. "claude-opus-4-7".
- Next model_id in the fallback chain. Empty string = chain terminates.
- `pub fn has_all_caps` — True if this model has every capability in `caps`.
- `pub fn has_role` — True if `tag` matches any role tag (case-sensitive, exact match).
- `pub fn fallback_target` — `Some(id)` if a non-empty fallback target is set, else `None`.
- Google: Gemini text-LLM family + image generation (nano-banana CLI
- fal.ai — image / video / 3D generation aggregator. Hosts Flux,
- ElevenLabs — text-to-speech and voice cloning.

## Related

- parent: `kei-model/Cargo.toml`
- imports: crate, serde

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
