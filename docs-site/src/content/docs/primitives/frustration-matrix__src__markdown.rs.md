---
title: markdown.rs
path: frustration-matrix/src/markdown.rs
dna_hash: sha256:7517ef7f0836f54a
language: rust
size_loc: 141
generated: by-keidocs
---

# frustration-matrix/src/markdown.rs

Markdown chatlog parser — extract USER blocks only.

Four block conventions coexist in our chatlog archive:
1. `### User ...` or `## User ...`     (header-delimited, most common)
2. `**User:**`                          (inline-bold label)
3. `> user: ...` quote-style            (quote block label)
4. raw assistant/user lines in `<User>` / `<Assistant>` XML-ish tags

Block scope: starts at a USER marker, ends at the next ASSISTANT marker
OR the next same-or-higher header. Everything in scope = USER text.

We emit ONE `UserLine` per non-empty, non-marker line in a user block —
this way `line_no` in output rows points at an actual line with content,
which is what a reviewer wants when they open the file in an editor.

## Public API

- One user-written line extracted from a chatlog.
- Role detected for a single line of markdown.
- `pub fn parse` — Parse a whole markdown buffer; file path is stamped into each line.
- Classify a line's role marker. We only look at the first ~64 chars —
- Match `role:` bare lines like `user:` or `assistant:` (JSONL-rendered).
- Strip quote / list prefixes; return Some only if non-empty text remains.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: std

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
