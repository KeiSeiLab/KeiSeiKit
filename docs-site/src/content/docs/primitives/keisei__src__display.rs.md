---
title: display.rs
path: keisei/src/display.rs
dna_hash: sha256:0dce82dea878b98c
language: rust
size_loc: 27
generated: by-keidocs
---

# keisei/src/display.rs

Display sanitization — ANSI/control-char stripper for user-facing output.

Constructor Pattern: single responsibility — convert untrusted strings
(brain names, paths from manifests) into display-safe text by replacing
every ASCII control character (`< 0x20` or `== 0x7F`) with `?`. Space
(0x20) is preserved. Characters outside ASCII (emoji / unicode) pass
through unchanged — their UTF-8 bytes are all `> 0x7F` and never
collide with the control-byte range.

Closes L9 (v0.19.2 audit): a malicious manifest
`name = "evil\x1b[2J..."` would clear the terminal or inject escape
sequences when `status` prints it. Every branch that prints
manifest-sourced text MUST route through `sanitize_display` first.

## Public API

- `pub fn sanitize_display` — Replace every ASCII control character (`< 0x20` or `== 0x7F`) with `?`.

## Related

- parent: `keisei/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
