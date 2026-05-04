---
title: main.rs
path: kei-memory/src/main.rs
dna_hash: sha256:a888d4b928c8d170
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-memory/src/main.rs

kei-memory — offline session analyzer (binary entrypoint).

Constructor Pattern: main.rs only dispatches; work lives in cubes.
Storage: `~/.claude/memory/kei-memory.sqlite` (or $KEI_MEMORY_DB).
RULE 0.14 — session self-audit, silent-first until 10 sessions ingested.

## Public API

- Override DB path (default: $KEI_MEMORY_DB or ~/.claude/memory/kei-memory.sqlite)
- Read a JSONL transcript and insert session + events.
- Print a retrospective for a session or the last N sessions.
- List recurring event-class patterns.
- Top-k past sessions by TF-IDF cosine similarity to the query text.
- Dump a session's events as markdown to stdout.
- N sessions, N events, top tools.
- Manage the silent-first audit backlog items.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: clap, kei_memory, rusqlite, std

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
