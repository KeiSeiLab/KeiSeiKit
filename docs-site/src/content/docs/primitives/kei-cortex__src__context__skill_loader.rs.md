---
title: skill_loader.rs
path: kei-cortex/src/context/skill_loader.rs
dna_hash: sha256:7fc039b120143039
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-cortex/src/context/skill_loader.rs

Match a leading `/skill-name` token in the user's first turn and load
the corresponding `SKILL.md` body.

Resolution order (project-local wins):
1. `<project_root>/.claude/skills/<name>/SKILL.md`
2. `~/.claude/skills/<name>/SKILL.md`

Only the FIRST whitespace-delimited token is inspected. If that token
does not start with `/`, or the resolved file does not exist, the
function returns `None` — the caller treats this as "no skill matched"
and falls back to the persona/context-only system prompt.

## Public API

- Hard read cap mirroring `discover::MAX_FILE_BYTES`. Skills are usually
- `pub fn match_skill_command` — If `user_msg` starts with `/skill-name`, locate and read the
- Pull the `<name>` out of `/<name>...`. Returns `None` if the message
- Reject pathological skill names: empty, path-segment, dot-relative, or
- Attempt `<root>/.claude/skills/<name>/SKILL.md` and read it.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
