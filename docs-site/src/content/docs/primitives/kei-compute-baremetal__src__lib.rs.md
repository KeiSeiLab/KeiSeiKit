---
title: lib.rs
path: kei-compute-baremetal/src/lib.rs
dna_hash: sha256:89a2da36d5b9b98e
language: rust
size_loc: 18
generated: by-keidocs
---

# kei-compute-baremetal/src/lib.rs

kei-compute-baremetal — ComputeProvider impl for user-owned hardware.

Registers an existing SSH-reachable box (VPS, dedicated server, lab box)
as a managed VM. `create()` runs the cloud-init shell over SSH; `destroy()`
deregisters but never powers off user hardware. `resize()` / `start()` /
`stop()` return `NotImplemented` — manual user action only.

Auth: SSH key path is passed at construction (RULE 0.8 — never hardcoded).

## Related

- parent: `kei-compute-baremetal/Cargo.toml`

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
