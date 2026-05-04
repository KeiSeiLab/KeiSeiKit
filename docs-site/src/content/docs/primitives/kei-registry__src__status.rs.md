---
title: status.rs
path: kei-registry/src/status.rs
dna_hash: sha256:19d33301364b003f
language: rust
size_loc: 398
generated: by-keidocs
---

# kei-registry/src/status.rs

Cross-cutting "what is alive right now" view.

Constructor Pattern: pure read-side cube. Joins three sources for a
single dashboard:
1. `blocks` table from kei-registry — atoms, skills, rules, hooks,
primitives, plus path-atoms (atoms whose source file is
`_blocks/path-*.md`).
2. `agents` table from `~/.claude/agents/ledger.sqlite` if present —
agent forks per RULE 0.12, with status (running / done / failed /
merged / rejected).
3. `git for-each-ref refs/heads` shell-out — local branches with
`ahead`, `behind` and `dirty` flags relative to their upstream.

No I/O beyond DB reads + one git invocation. No writes. The handler
formats the gathered struct into either an ASCII table (default) or
JSON (`--format json`).

## Public API

- Aggregate snapshot returned by `compute_status`.
- Deterministic DNA-style identifier for the branch. Format
- `pub fn compute_status` — Compute the full status snapshot. `git_repo` is the path to scan for
- Compute a deterministic DNA-style identifier for a git branch. Mirrors
- Take the first three segments of a `<role>::<caps>::<scope_sha8>::...`
- Parse `upstream:track,nobracket` output. Examples:
- `pub fn render_ascii` — Render `Status` as a multi-section ASCII report.
- `pub fn default_ledger_path` — Default ledger path: `$KEI_LEDGER_DB` or `~/.claude/agents/ledger.sqlite`.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, serde, sha2, std

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
