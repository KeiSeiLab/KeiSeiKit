---
title: os.rs
path: kei-machine-probe/src/os.rs
dna_hash: sha256:1357c653522f1795
language: rust
size_loc: 62
generated: by-keidocs
---

# kei-machine-probe/src/os.rs

OS detection.

Primary path: `sw_vers -productVersion` + `sw_vers -buildVersion` on
macOS. If `sw_vers` fails (Linux / other), fall back to
`uname -sr` to recover at least family and version. Anything we
can't classify reports `OsFamily::Other` rather than panicking.

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: crate

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
