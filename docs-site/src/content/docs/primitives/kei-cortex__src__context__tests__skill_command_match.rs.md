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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
