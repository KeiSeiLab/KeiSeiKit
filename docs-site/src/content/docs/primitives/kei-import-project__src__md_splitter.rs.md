---
title: md_splitter.rs
path: kei-import-project/src/md_splitter.rs
dna_hash: sha256:2a6b41c3cb6f57e3
language: rust
size_loc: 112
generated: by-keidocs
---

# kei-import-project/src/md_splitter.rs

Markdown H2-heading splitter and description extractor.

Provides pure text operations used by skill_extractor:
- split_by_h2: parse `## ` sections from markdown text
- first_sentences: extract up to 3 sentences for a skill description
- strip_markdown: remove markdown syntax for plain-text extraction

## Public API

- `pub fn split_by_h2` — Split markdown text into `(slug, heading, body)` tuples at `## ` boundaries.
- `pub fn first_sentences` — Extract first 1-3 sentences up to `max_chars` from body text.
- `pub fn strip_markdown` — Strip markdown syntax (headings, code fences, links) for description use.

## Related

- parent: `kei-import-project/Cargo.toml`
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
