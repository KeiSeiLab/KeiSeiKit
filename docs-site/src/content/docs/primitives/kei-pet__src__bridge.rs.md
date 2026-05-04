---
title: bridge.rs
path: kei-pet/src/bridge.rs
dna_hash: sha256:56df6e2270859161
language: rust
size_loc: 47
generated: by-keidocs
---

# kei-pet/src/bridge.rs

Bridge between a validated `PetManifest` and an agent-spawn prompt.

Used by the `/spawn-agent` skill's pet-overlay phase: compose the final
system prompt as `base_prompt` ++ (optional persona overlay) ++ `task_body`.
No I/O here — pure string composition. Deterministic.

Scope boundary (see crate root): this module renders prompts for any
agent runtime. It imports nothing from sibling research-grade projects.

## Public API

- Everything the bridge needs to compose one spawn prompt.
- `pub fn compose_prompt_with_pet` — Compose the full prompt: base + persona overlay (if any) + task body.

## Related

- parent: `kei-pet/Cargo.toml`
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
