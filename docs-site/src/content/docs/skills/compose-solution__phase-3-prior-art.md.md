---
title: phase-3-prior-art
path: compose-solution/phase-3-prior-art.md
dna_hash: sha256:e929f563fa99a613
language: markdown
size_loc: 64
generated: by-keidocs
---

# compose-solution/phase-3-prior-art.md

## Public API

- `Phase 3 — Prior-art grep sweep (parallel)` — For EACH component from Phase 2, run three independent searches in parallel
- `3a — KeiSeiKit reuse` — ```bash
- `Replace <keywords> with the component's 3-5 distinctive keywords as an` — 
- `ERE alternation like (foo|bar|baz).` — 
- `The 7 paths cover: behavioral blocks, agent manifests, shell primitives,` — 
- `Rust primitive crates (source + Cargo.toml), all skills, cross-tool` — 
- `bridges, and enforced hooks. grep -r recurses, so _primitives/ catches` — 
- `both *.sh and _rust/<crate>/src/*.rs — but _primitives/_rust/ is listed` — 
- `explicitly for discoverability when someone reads this file.` — grep -rinlE '<keywords>' \
- `3b — Personal bundle reuse (conditional, skip on missing)` — If the environment variable `KEISEI_BUNDLE_PATH` is set and the directory
- `3c — External docs (delegate)` — For any component that involves an external API, framework, or third-party
- `3d — Classify + evidence-grade` — For each component produce ONE row:
- `Verify-criterion` — - Every component has a classification.

## Related

- parent: `compose-solution`

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
