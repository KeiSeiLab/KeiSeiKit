---
title: rule.rs
path: kei-registry/src/scanners/rule.rs
dna_hash: sha256:42e3fe4b269aabc7
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-registry/src/scanners/rule.rs

Rule scanner — walks `<rules-root>/*.md`.

Constructor Pattern: this cube knows only the flat `~/.claude/rules/`
directory layout. Body = raw markdown; name = filename stem; caps = `md`.

## Public API

- `pub struct RuleScanner` — `<rules-root>/<name>.md` adapter. The rules root is configurable via

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
