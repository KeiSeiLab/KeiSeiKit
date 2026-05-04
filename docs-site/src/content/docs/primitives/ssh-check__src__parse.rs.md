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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
