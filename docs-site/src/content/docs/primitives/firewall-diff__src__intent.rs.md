---
title: intent.rs
path: firewall-diff/src/intent.rs
dna_hash: sha256:cc3c69e643a6c8bc
language: rust
size_loc: 111
generated: by-keidocs
---

# firewall-diff/src/intent.rs

Intent YAML schema + loader. See `_blocks/security-firewall-ufw.md` for
the reference format. Anything missing is treated as "don't care".

## Public API

- `pub fn key` — Canonical key used to match against a live rule: port/proto/from/action.

## Related

- parent: `firewall-diff/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
