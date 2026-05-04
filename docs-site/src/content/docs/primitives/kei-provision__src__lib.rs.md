---
title: lib.rs
path: kei-provision/src/lib.rs
dna_hash: sha256:837110e49956d8df
language: rust
size_loc: 20
generated: by-keidocs
---

# kei-provision/src/lib.rs

kei-provision — unified VPS provisioner (Hetzner + Vultr, extensible).

Supersedes `_primitives/provision-hetzner.sh` + `_primitives/provision-vultr.sh`.

Layers:
`backend`           — `Backend` trait + `CreateOpts` + `ServerInfo`.
`backends::hetzner` — adapts `hcloud server …` JSON output.
`backends::vultr`   — adapts `vultr-cli instance …` JSON output.
`exec`              — shared `std::process::Command` + env/cli checks.

Tests inject a temp PATH containing a fake `hcloud` / `vultr-cli` that
emits canned JSON, so no cloud calls happen in CI.

## Related

- parent: `kei-provision/Cargo.toml`

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
