---
title: main.rs
path: keisei/src/main.rs
dna_hash: sha256:cd04d65dd9f68e9d
language: rust
size_loc: 105
generated: by-keidocs
---

# keisei/src/main.rs

keisei — exobrain attach/status CLI (v0.22 multi-brain + Auto scope).

Constructor Pattern: main.rs = clap parse + dispatch only. All
subcommand logic lives in sibling modules
(`attach.rs`, `status.rs`, `mount.rs`, `detach.rs`, `list.rs`).

## Public API

- Host-wide config (`~/.claude/...`, `~/.cursor/...`).
- Project-local config (`./.claude/...`, `./.cursor/...`).
- Let the adapter pick based on CWD (v0.22 default). `./.claude/`
- Attach a brain to the single currently detected AI client.
- Path to the brain directory (must contain manifest.toml).
- Which client config to write — host-wide (`user`), project-local
- Attach a brain to EVERY detected AI client in one shot.
- Path to the brain directory (must contain manifest.toml).
- Remove the brain from every client recorded in the marker.
- Show the currently attached brain + health checks.
- List every registered adapter + whether it's detected here.

## Related

- parent: `keisei/Cargo.toml`
- imports: clap, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
