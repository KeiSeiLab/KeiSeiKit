---
title: rule.rs
path: kei-decompose/src/parsers/rule.rs
dna_hash: sha256:b0f33ef6e818b355
language: rust
size_loc: 262
generated: by-keidocs
---

# kei-decompose/src/parsers/rule.rs

Rule file parser — splits `~/.claude/rules/*.md` into per-H2-section fragments.

Each fragment maps to one `RuleFragment` record. The body of the fragment
is registered in `kei-registry` as a `BlockType::Rule` block. H3 headings
stay inside their H2 parent; code blocks are preserved verbatim (no split
inside fences).

Constructor Pattern: this cube owns the parsing/splitting logic only.
Registry writes live in the `decompose_rules` CLI handler (main.rs).

## Public API

- One H2-bounded section of a rule file.
- Stem of the source file, e.g. `"karpathy-behavioral"`.
- Kebab-cased section heading, e.g. `"think-before-coding"`.
- Verbatim heading text (without leading `## `).
- Section body (heading line not included).
- 1-based line number of the H2 heading (or 1 for `_root`).
- 1-based line number of the last body line (inclusive).
- `pub fn parse_rule_file` — Parse `path` into per-section `RuleFragment`s.
- Convert a heading string to a kebab-case slug.
- Push a completed section into `out` if body is non-empty.
- Return a fallback `_root` fragment when no H2 headings were found.
- State machine for one line during section scanning.
- Classify one line given current fence state and section state.
- Core split logic — operates on the raw text, not file I/O.
- Returns the heading text if this line is an H2 (`## …`), else None.

## Related

- parent: `kei-decompose/Cargo.toml`
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
