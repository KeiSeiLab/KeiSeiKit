---
title: fragment_writer.rs
path: kei-import-project/src/fragment_writer.rs
dna_hash: sha256:3b3ec2590d751e04
language: rust
size_loc: 116
generated: by-keidocs
---

# kei-import-project/src/fragment_writer.rs

Write and manage canonical SKILL.md fragment files on disk.

One fragment file per extracted skill. Format: YAML frontmatter
(name + description) followed by the verbatim section body.
Idempotent: if content is unchanged the file is not rewritten.

## Public API

- Outcome of a single write attempt.
- `pub fn fragment_path` — Build the canonical fragment file path.
- `pub fn render_skill_md` — Render SKILL.md content (frontmatter + body).
- `pub fn write_fragment` — Write a fragment file. Returns Unchanged if the existing content matches.
- `pub fn sanitize` — Slugify a string: lowercase, replace non-alnum with `-`, collapse runs.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, std, tempfile

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
