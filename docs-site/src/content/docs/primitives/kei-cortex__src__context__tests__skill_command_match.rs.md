---
title: skill_command_match.rs
path: kei-cortex/src/context/tests/skill_command_match.rs
dna_hash: sha256:24b1b2ccd09b1d91
language: rust
size_loc: 57
generated: by-keidocs
---

# kei-cortex/src/context/tests/skill_command_match.rs

`match_skill_command` finds `<project>/.claude/skills/<name>/SKILL.md`
when the user message starts with `/<name>`.

Resolution: project-local wins over $HOME-local; missing file -> None;
malformed names rejected.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

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
