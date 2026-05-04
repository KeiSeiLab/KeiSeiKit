---
title: manager.rs
path: kei-svc-systemd/src/manager.rs
dna_hash: sha256:bec8cb3ff42d3697
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-svc-systemd/src/manager.rs

## Public API

- Where unit files are written. Defaults to `/etc/systemd/system/`
- `systemctl` binary path; default just `"systemctl"`.
- Whether to actually invoke systemctl or just write files (for tests).

## Related

- parent: `kei-svc-systemd/Cargo.toml`
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
