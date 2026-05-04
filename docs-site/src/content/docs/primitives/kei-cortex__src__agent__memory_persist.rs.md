---
title: memory_persist.rs
path: kei-cortex/src/agent/memory_persist.rs
dna_hash: sha256:6b20975d347e7e49
language: rust
size_loc: 122
generated: by-keidocs
---

# kei-cortex/src/agent/memory_persist.rs

Disk-backed memory write for review outcomes.

Constructor Pattern: this cube owns ONE responsibility — persisting
a successful review reply to the `pet_conversations` SQLite table
used by the rest of kei-cortex (`handlers/memory.rs`,
`kei_pet::memory`). Empty / short-circuit / error replies are
filtered out so the table only accrues genuine memory entries.

Threading model: `record_review_blocking` is synchronous — callers
wrap it in `tokio::task::spawn_blocking` (matching the
`handlers/memory.rs` pattern). The file does not own a tokio
runtime so it stays cheap to import in non-async contexts (tests).

## Public API

- `pub const REVIEW_ROLE` — Role string the review writes under. Distinct from `"user"` /
- Outcome of a single persist attempt.
- Reply was empty after trimming — nothing written.
- Reply matched the `Nothing to save.` short-circuit.
- Reply was a transport-error placeholder.
- Successful write; carries the new rowid.
- Underlying SQLite or schema error; write skipped.
- `pub fn classify_reply` — Decide whether the reply is worth persisting. Pure — no I/O.
- `pub fn record_review_blocking` — Synchronous write path. Caller wraps in `spawn_blocking`. Schema is
- Open the sqlite db, creating the parent directory if absent. The
- Carry-around bundle for async contexts (chat handler) — avoids
- `pub fn run` — Run the blocking write. Designed to be the body of a

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: kei_pet, rusqlite, std

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
