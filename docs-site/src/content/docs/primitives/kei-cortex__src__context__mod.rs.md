---
title: mod.rs
path: kei-cortex/src/context/mod.rs
dna_hash: sha256:f042a3c9e178e734
language: rust
size_loc: 36
generated: by-keidocs
---

# kei-cortex/src/context/mod.rs

`context` — auto-discover CLAUDE.md / AGENTS.md / SOUL.md context
files by walking up from the chat process's cwd, optionally match a
`/skill-name` command at the start of the user message, and inject all
of it ahead of the persona prompt before the upstream call.

Public surface:
- [`discover`] — walk up, return nearest-first.
- [`match_skill_command`] — pull leading `/<name>` from a user message.
- [`build_system_prompt`] — concat persona + discovered + skill, capped.

See `INTEGRATION.md` for the orchestrator-side patch in `chat.rs`.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
