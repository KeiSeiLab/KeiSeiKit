---
title: provider.rs
path: kei-compute-baremetal/src/provider.rs
dna_hash: sha256:9e60301f4bc24300
language: rust
size_loc: 300
generated: by-keidocs
---

# kei-compute-baremetal/src/provider.rs

[`BaremetalCompute`] — `ComputeProvider` for user-owned hardware.

Differences vs cloud providers:
* `create()` does not provision — it registers an SSH connection and
runs the cloud-init shell remotely.
* `destroy()` deregisters; user hardware is never powered off.
* `resize()`/`start()`/`stop()` return `NotImplemented`.
* `cost_per_hour_microcents()` is always 0 (user-owned).

## Public API

- `pub struct BaremetalCompute` — Bare-metal provider. Stateless w.r.t. registered boxes — the SSH
- Default SSH key path applied when a `VmSpec` does not override it
- First 8 hex chars of SHA-256(input). Stable host-fingerprint for DNA caps.

## Related

- parent: `kei-compute-baremetal/Cargo.toml`
- imports: crate, kei_runtime_core, std

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
