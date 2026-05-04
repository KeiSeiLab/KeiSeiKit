---
title: precedent.rs
path: kei-spawn/src/precedent.rs
dna_hash: sha256:fad7edcc51f0bdc3
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-spawn/src/precedent.rs

precedent — env-gated advisory check against `kei-dna-index`.

When `KEI_SPAWN_PRECEDENT_CHECK=1`, spawn.rs calls `run_advisory` after
it has composed the body but before `kei-ledger fork`. We shell out
to `kei-dna-index precedent --body <sha>`; if the primitive returns
a non-empty JSON array of prior-agent matches, we eprintln a WARN
and return the hit count. We never block the spawn — this is a
human-facing signal, not a gate.

Constructor Pattern: one module = one responsibility (precedent
advisory only). No git, no ledger write, no filesystem mutation.

## Public API

- `pub const ENABLE_ENV` — Env flag that enables this advisory. Absent → `run_advisory` is a
- `pub const BIN_ENV` — Env override for the `kei-dna-index` binary path. Default = PATH lookup.
- `pub fn run_advisory` — Run the advisory. Returns the number of prior-agent hits reported
- Shell to kei-dna-index and capture stdout on success. Any failure
- Resolve the kei-dna-index binary, env override first.
- Minimal JSON-array-length parser. kei-dna-index `precedent` emits a

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: anyhow, std

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
