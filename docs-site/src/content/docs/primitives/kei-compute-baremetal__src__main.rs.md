---
title: main.rs
path: kei-compute-baremetal/src/main.rs
dna_hash: sha256:de41b3438c657a66
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-compute-baremetal/src/main.rs

kei-compute-baremetal CLI — `dna` / `register` / `status` / `unregister`.
No HTTP, no cloud — pure SSH dispatch over the system `ssh` binary.

## Public API

- Print this primitive's own DNA.
- Register an existing SSH-reachable box. Runs cloud-init shell remotely.
- SSH-ping the registered box.
- Deregister (no hardware action).

## Related

- parent: `kei-compute-baremetal/Cargo.toml`
- imports: clap, kei_compute_baremetal, kei_runtime_core

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
