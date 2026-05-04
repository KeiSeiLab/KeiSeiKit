---
title: persistence.rs
path: kei-frustration-loop/src/persistence.rs
dna_hash: sha256:354dbc05695dd4bb
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-frustration-loop/src/persistence.rs

Per-user firmware persistence — paths + atomic file swap.

Layout under `<home>/.claude/frustration/`:
* `<user>.firmware.gz`   — per-user trained byte n-gram (gz JSON, same
format as `firmware::Firmware::save`)
* `<user>.last-scan.ts`  — Unix timestamp (seconds) of last nightly scan
* `<user>.feedback.jsonl` — one JSON record per user correction
* `queue.jsonl`          — shared queue of new hits awaiting review

Constructor Pattern: this cube only resolves paths and shovels bytes.
Format decisions live in `feedback.rs` / `firmware.rs`.

## Public API

- `pub const FRUSTRATION_DIR` — Directory name under `~/.claude/` where the loop persists per-user state.
- `pub fn user_firmware_path` — Build the per-user firmware path: `<home>/.claude/frustration/<user>.firmware.gz`.
- `pub fn last_scan_ts_path` — Path to the per-user last-scan timestamp marker (Unix seconds, plain text).
- `pub fn feedback_path` — Path to the per-user feedback log (jsonl, append-only).
- `pub fn queue_path` — Shared queue of new hits awaiting user review (across users).
- `pub fn frustration_dir` — Resolve `<home>/.claude/frustration` (no side effects).
- `pub fn ensure_dir` — Create `<home>/.claude/frustration` with 0700 permissions if missing.
- `pub fn atomic_write` — Atomic write: pipe `bytes` into `<dest>.tmp`, fsync, rename → `dest`.
- `pub fn atomic_swap` — Rename `tmp` over `dest`. Both must live on the same filesystem.
- `pub fn load_or_default` — Read a file or return `fallback` if it does not exist. Other IO
- `pub fn read_last_scan_ts` — Read a Unix-seconds timestamp from `<user>.last-scan.ts`. Missing file
- `pub fn write_last_scan_ts` — Persist `ts` into `<user>.last-scan.ts` atomically.
- Build the `<dest>.tmp` sibling path used by `atomic_write`.
- Apply 0700 permissions on UNIX targets. No-op on Windows (the kit is

## Related

- parent: `kei-frustration-loop/Cargo.toml`
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
