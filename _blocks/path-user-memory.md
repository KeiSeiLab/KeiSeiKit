---
type: atom
kind: path
name: user-memory
template: ~/.claude/memory
expand_at: render
---

# Path atom — user-memory

Resolves to the user's `~/.claude/memory/` directory.

Used by agent manifests (`_manifests/*.toml`) to reference companion memory files without leaking the absolute path (with maintainer's home `/Users/<user>/...`) into public artefacts under `_generated/`.

**Usage in manifests:**
```toml
[references]
extra = [
  "path:user-memory/wrong-paths-specialized-ml.md",
  "path:user-memory/fal-ai-models.md",
]
```

**Resolution:** the assembler detects the `path:user-memory/` prefix, looks up this atom in the registry, and emits an opaque DNA reference into the rendered `_generated/<agent>.md`. The published markdown therefore contains a content-addressed atom-DNA link, not an absolute path. A reader with a local kit + registry resolves the DNA back to the file; a reader without the kit sees only the opaque hash.

**Expand timing:** `render` — substitution happens at `_assembler` time, before the `_generated/` markdown is written.

**Constructor Pattern:** one cube, one path. No code, no logic. Body bytes + frontmatter ARE the atom. Hash → DNA via standard registry pipeline.
