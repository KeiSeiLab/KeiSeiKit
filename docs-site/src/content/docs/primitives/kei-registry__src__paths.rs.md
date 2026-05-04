---
title: paths.rs
path: kei-registry/src/paths.rs
dna_hash: sha256:3a57fbd2b2758c38
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-registry/src/paths.rs

Path resolution for CLI flags.

Constructor Pattern: this cube owns CLI default-path policy. Two
callers (handlers + scan_orchestrator) share these helpers so the
"where does the registry SQLite live by default" decision exists in
one place.

## Public API

- `pub fn resolve_db` — Resolve `--db <path>` or default to `~/.claude/registry.sqlite`.
- `pub fn resolve_kit_root` — Resolve `--kit-root` or default to current directory.
- `pub fn resolve_rules_root` — Resolve `--rules-root` or default to `~/.claude/rules`.
- `pub fn resolve_hooks_root` — Resolve `--hooks-root` or default to `~/.claude/hooks`.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
