---
title: kimi.rs
path: kei-skill-importer/src/parsers/kimi.rs
dna_hash: sha256:c4f9a955ae0f7a32
language: rust
size_loc: 182
generated: by-keidocs
---

# kei-skill-importer/src/parsers/kimi.rs

Kimi CLI agent-spec parser.

Format research (verified 2026-04-25 via WebFetch
`https://raw.githubusercontent.com/MoonshotAI/kimi-cli/main/AGENTS.md`):

- Two file types coexist:
1. `AGENTS.md` — root-level architecture map (markdown body).
2. `src/kimi_cli/agents/<name>.yaml` — agent spec (pure YAML)
with keys: `name`, `description`, `extend`, `tools`,
`subagents`, `system_prompt` (often inline multiline string).
- For YAML specs, the entire file is the frontmatter; body = empty
OR pulled from `system_prompt` (multiline literal scalar).
- For markdown AGENTS.md, behaviour matches OpenClaw (H2 sections).

This parser handles BOTH shapes: detect by extension. `.yaml`/`.yml`
→ spec mode; otherwise → markdown mode (delegates to internal H2 split).

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, serde, serde_yaml_ng, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
