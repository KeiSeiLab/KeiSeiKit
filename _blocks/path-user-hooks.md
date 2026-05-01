---
type: atom
kind: path
name: user-hooks
template: ~/.claude/hooks
expand_at: render
---

# Path atom — user-hooks

Resolves to the user's `~/.claude/hooks/` directory (PreToolUse / PostToolUse / UserPromptSubmit / Stop hook scripts like `agent-stub-scan.sh`, `agent-outcome-backfill.sh`, `numeric-claims-guard.sh`, etc.).

Used by agent manifests (`_manifests/*.toml`) to reference hook scripts without leaking the absolute path (with the maintainer's home `/Users/<user>/...`) into public artefacts under `_generated/`.

**Usage in manifests:**
```toml
[references]
extra = [
  "path:user-hooks/agent-outcome-backfill.sh",
  "path:user-hooks/agent-stub-scan.sh",
]
```

**Resolution:** the assembler detects the `path:user-hooks/` prefix, looks up this atom in the registry, and emits an opaque DNA reference into the rendered `_generated/<agent>.md`. Same content-addressing semantics as `path-user-memory` and `path-user-rules` — published artefact has DNA hashes, not paths. A reader with a local kit + registry resolves the DNA back to the file; a reader without the kit sees only the opaque hash.

**Expand timing:** `render` — substitution happens at `_assembler` time, before the `_generated/` markdown is written.

**Constructor Pattern:** one cube, one path. No code, no logic. Body bytes + frontmatter ARE the atom. Hash → DNA via standard registry pipeline.
