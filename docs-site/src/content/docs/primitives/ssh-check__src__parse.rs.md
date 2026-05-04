---
title: parse.rs
path: ssh-check/src/parse.rs
dna_hash: sha256:40694a98acaa01ab
language: rust
size_loc: 127
generated: by-keidocs
---

# ssh-check/src/parse.rs

sshd_config parser — read main file + drop-ins, merge with last-wins
precedence per OpenSSH rules (main file first, then drop-ins in
filename-sort order; first occurrence of a directive wins in sshd,
BUT we surface ALL occurrences to report duplicates).

## Public API

- A single directive occurrence (name, value, source path, line number).
- Merged view: directive name (lowercased) → first-occurrence value +
- Parse one config line. Returns (lowercased_directive, raw_value) or None

## Related

- parent: `ssh-check/Cargo.toml`
- imports: std

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
