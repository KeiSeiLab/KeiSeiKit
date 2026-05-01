---
type: atom
kind: path
name: user-rules
template: ~/.claude/rules
expand_at: render
---

# Path atom — user-rules

Resolves to the user's `~/.claude/rules/` directory (umbrella rule files like `api-cost-guard.md`, `paradigm-native-measurement.md`, `manifold-tangent-sanity.md`, etc.).

Used by agent manifests (`_manifests/*.toml`) to reference rule files without leaking the absolute path into public artefacts under `_generated/`.

**Usage in manifests:**
```toml
[references]
extra = [
  "path:user-rules/api-cost-guard.md",
  "path:user-rules/paradigm-native-measurement.md",
]
```

**Resolution:** the assembler detects the `path:user-rules/` prefix, looks up this atom in the registry, and emits an opaque DNA reference into the rendered `_generated/<agent>.md`. Same content-addressing semantics as `path-user-memory` — published artefact has DNA hashes, not paths.

**Expand timing:** `render` — substitution at `_assembler` time.

**Constructor Pattern:** one cube, one path. Body + frontmatter is the atom.
