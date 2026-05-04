---
title: tool_apply.rs
path: kei-cortex/src/handlers/tool_apply.rs
dna_hash: sha256:88818245ad3be8f7
language: rust
size_loc: 200
generated: by-keidocs
---

# kei-cortex/src/handlers/tool_apply.rs

`POST /api/v1/cortex/tool/apply` — apply edit/write proposed by the
agentic loop. UI captures `tool_use_start{name:"edit"}`, shows DiffPane,
and POSTs here on Apply. TRUSTED op — bearer-auth only; see INTEGRATION.md.
Wave 44b F-CRIT-4: atomic write moved to `tool_apply_atomic.rs` (`O_NOFOLLOW`
openat + post-rename canonical re-check).

## Public API

- Hard cap on file bytes — matches `tool::read::MAX_BYTES`.
- Request body. `old_text`/`new_text` are aliases for `old_string`/`new_string`
- Response body — UI consumes `applied` and `diff_summary.lines_changed`.
- Resolved sandbox view: the candidate path the caller asked for plus the
- Handler entry point — wired in `routes.rs` under the bearer group.
- Run sandbox helpers + confirm path canonicalises inside `project_root`.
- Walk `p` upward until an ancestor exists; return its canonical form.
- Apply an `edit` — read file, enforce uniqueness, atomic-write back.
- Apply a `write` — refuses overwrite without `force=true`.
- Enforce uniqueness/match rules and produce the post-replacement string.
- Count of lines that differ between `before` and `after` (length-tolerant).

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
