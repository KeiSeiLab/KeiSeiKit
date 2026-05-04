---
title: kei-pet.rs
path: kei-pet/src/bin/kei-pet.rs
dna_hash: sha256:96abaef932925a23
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-pet/src/bin/kei-pet.rs

kei-pet — CLI wrapper over the `kei_pet` library.

Subcommands:
validate <path>     Parse + run R1–R19 on a pet.toml, print PASS/FAIL
show <path>         Print the rendered system-prompt overlay
identity <action>   new | show — generate or display Ed25519 keypair
tune <path> <kv>    Surgical axis edit (kv: `voice.tone_primary=warm`)

## Public API

- Parse and validate a pet.toml file.
- Render and print the system-prompt overlay for a pet.toml.
- Ed25519 identity operations.
- Surgical edit of one axis. `path` is the pet.toml, `kv` is key=value.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: anyhow, clap, kei_pet, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
