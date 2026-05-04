---
title: jsonl.rs
path: frustration-matrix/src/jsonl.rs
dna_hash: sha256:670db98b8f4f5f63
language: rust
size_loc: 137
generated: by-keidocs
---

# frustration-matrix/src/jsonl.rs

JSONL session transcript parser — extract USER messages only.

Raw Claude Code session files (`~/.claude/projects/*/sessions/*.jsonl`)
are newline-delimited JSON. One message per line. Shapes vary across
Claude Code versions — see `extract_user_text` for the five known
variants we normalise.

Constructor Pattern: one file, one public entry (`parse_user_lines`).
Helpers are small and private. No full-file `read_to_string` — we
stream via `BufReader::lines()` so a 1.4 GB corpus never materialises
in memory all at once.

System echoes (`<local-command-*>`, `<command-*>`, `<system-reminder>`,
`<task-notification>`, `<command-stderr>`) are injected by the CLI
runtime, not typed by the user — we drop them here, not in the
category regexes.

## Public API

- One user-written message extracted from a raw `.jsonl` session file.
- `pub fn parse_user_lines` — Stream-parse a `.jsonl` session file, yielding user messages only.
- Parse a single JSONL line; return `None` unless it is a real user
- A record is a user message if either the top-level `type == "user"` OR
- Extract visible text from the three known content shapes:
- Concatenate all `{type: "text", text: "..."}` blocks in an array.
- True for CLI-injected tags that masquerade as user turns. These are

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, serde_json, std

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
