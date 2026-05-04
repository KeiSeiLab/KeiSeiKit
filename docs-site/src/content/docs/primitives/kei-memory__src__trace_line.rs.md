---
title: trace_line.rs
path: kei-memory/src/trace_line.rs
dna_hash: sha256:09efa5e5edf58076
language: rust
size_loc: 132
generated: by-keidocs
---

# kei-memory/src/trace_line.rs

TraceLine — superset of real-trace + legacy-flat trace fields.

Constructor Pattern: this cube only declares the deserialised line
plus tiny helpers (text extraction, ts resolution). Decoding is
`serde_json` driven; persistence + classification live elsewhere.

Real Claude Code trace shape (sample 51a176c0-*.jsonl, 2026-04-30):
{"type": "assistant" | "user" | ..., "timestamp": "<rfc3339>",
"sessionId": "...", "cwd": "...", "gitBranch": "...",
"uuid": "...", "parentUuid": "...",
"message": {"role": "...", "content": [...]}}

Legacy KeiSeiKit flat shape (still supported for back-compat tests):
{"ts": 1700000000, "kind": "tool_use", "tool": "Bash",
"file_path": "...", "is_error": false, "message": "..."}

## Public API

- `pub fn message_text` — Best-effort plain text from `message` field for guard + persist.
- `pub fn resolved_ts` — Resolve event timestamp, preferring legacy `ts` (epoch i64) over

## Related

- parent: `kei-memory/Cargo.toml`
- imports: chrono, crate, serde, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
