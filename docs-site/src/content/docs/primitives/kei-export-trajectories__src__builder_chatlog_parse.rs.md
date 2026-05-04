---
title: builder_chatlog_parse.rs
path: kei-export-trajectories/src/builder_chatlog_parse.rs
dna_hash: sha256:d8041d458a1a4e0f
language: rust
size_loc: 159
generated: by-keidocs
---

# kei-export-trajectories/src/builder_chatlog_parse.rs

Multi-turn chatlog parser. Splits raw chatlog on `<tool_call>` /
`<tool_response>` XML and emits typed `ShareGptMessage` turns.
Legacy fallback (no markers): single `From::Gpt` turn wrapped in
`<think>\n</think>\n` to keep the pre-Hermes golden fixture stable.

## Public API

- `pub fn parse_chatlog_turns` — Parse a chatlog into ShareGPT turns.
- One classified slice of the chatlog.
- Advance one segment from `cursor` and return the new cursor.
- Find the earliest opening marker in `rest`. Returns `(relative_offset, kind)`.
- Consume one `<tool_call>...</tool_call>` block starting at absolute
- Consume one `<tool_response>...</tool_response>` block starting at

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: crate

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
