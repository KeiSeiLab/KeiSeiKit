---
title: hook.rs
path: kei-registry/src/scanners/hook.rs
dna_hash: sha256:5dae97538999d41b
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-registry/src/scanners/hook.rs

Hook scanner — walks `<hooks-root>/*.sh`.

Constructor Pattern: this cube knows only the flat `~/.claude/hooks/`
directory layout. Body = raw shell script bytes; name = filename stem;
caps = `shell`.

## Public API

- `pub struct HookScanner` — `<hooks-root>/<name>.sh` adapter. Configurable root because hooks live

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
