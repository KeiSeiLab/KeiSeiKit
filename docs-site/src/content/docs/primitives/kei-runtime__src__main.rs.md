---
title: main.rs
path: kei-runtime/src/main.rs
dna_hash: sha256:8b4035288e57631b
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-runtime/src/main.rs

kei-runtime — CLI dispatcher.

Subcommands: list-atoms | invoke | schema-lint | pipe (stub).
Default --root: `~/.claude/agents/_primitives/_rust`.

## Public API

- Exit code returned when `invoke` lands on a not-yet-wired atom.
- List atoms discovered under --root.
- Invoke one atom (MVP stub — see docs).
- Lint every `atoms/*.md` under --root for schema correctness.
- Execute a pipeline (not yet implemented).
- Map typed invoke errors to exit codes per locked §Runtime schema.

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: clap, kei_runtime, std

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
