---
title: loader.rs
path: kei-skills/src/loader.rs
dna_hash: sha256:5dae58a533c9c9c2
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-skills/src/loader.rs

Walk a directory and load every valid `SKILL.md`.

Used by [`crate::registry::SkillRegistry::new`] at daemon start. Lossy
by default — invalid skills surface as `LoadOutcome::Invalid` so the
daemon can log them without crashing the boot path.

## Public API

- Per-file outcome of `load_all`.
- `pub fn load_all` — Walk `dir` recursively for `SKILL.md` files. Each is read, validated,
- `pub fn loaded_only` — Shorthand for callers that only want the valid skills (drops Invalid/Io).
- `pub fn tally` — Count outcomes by kind for diagnostics. Returns `(loaded, invalid, io)`.

## Related

- parent: `kei-skills/Cargo.toml`
- imports: crate, std, walkdir

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
