# \_primitives — first-class building blocks

`_primitives/` holds standalone utilities that agents, hooks, and skills
(including `/compose-solution`) depend on. Unlike `_blocks/` (behavioral
markdown) or `_manifests/` (agent TOML), primitives are executable shell
programs installed at `$HOME/.claude/agents/_primitives/` by `install.sh`.

## Current primitives

| Primitive | Purpose | Invocation |
|---|---|---|
| `tomd.sh` | Universal non-native-format → markdown converter (PDF, DOCX, XLSX, PPTX, CSV, images, code). | `~/.claude/agents/_primitives/tomd.sh <file>` |

`tomd.sh` is a first-class primitive. Universal non-native-format →
markdown converter with configurable cache directory
(`KEISEI_TOMD_CACHE`) and KeiSeiKit-style error tags (`[tomd]`).

## Hook integration

`hooks/tomd-preread.sh` is a PreToolUse(Read) hook that auto-redirects
Claude to the converted markdown when a Read targets `.docx / .doc / .xlsx /
.pptx / .csv`. Cached under `$KEISEI_TOMD_CACHE` (default
`/tmp/keisei-tomd-cache`).

## `/compose-solution` discovery

Phase 3 prior-art sweep greps `_primitives/` alongside `_blocks/`,
`_manifests/`, `skills/`, `_bridges/`, `hooks/`. If a user task involves
file-format parsing, the meta-composer surfaces `tomd` automatically —
reuse over rewrite (RULE "No Patching").
