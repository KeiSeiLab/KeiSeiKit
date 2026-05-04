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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
