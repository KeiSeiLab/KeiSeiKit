---
title: lib.rs
path: kei-net-ipsec/src/lib.rs
dna_hash: sha256:cede3e460c1b76b5
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-net-ipsec/src/lib.rs

kei-net-ipsec — IPsec impl of [`kei_runtime_core::NetworkMode`] via
`swanctl` shell-out (strongSwan).

Layout (Constructor Pattern: 1 file = 1 cube, ≤200 LOC each):
- [`error`]: local `Error`/`Result` mapping into the runtime-core error.
- [`runner`]: [`Runner`] trait + [`SystemRunner`] / [`MockRunner`] —
single subprocess seam (mirror of `kei-llm-mlx::runner`).
- [`parse`]: SA-stanza parser for `swanctl --list-sas` text output.
- [`network`]: [`IpsecMode`] — DNA-bearing `NetworkMode` impl.

Mode flags:
- `is_public() = true` (IPsec exposes a routable public path; sibling
tailscale / wireguard adapters return `false`).

Env:
- `SWANCTL_CONFIG_DIR` — override `/etc/swanctl/` config root.
- `IPSEC_CHILD_NAME` — child SA name to bring up / tear down (default
`home`).

## Related

- parent: `kei-net-ipsec/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
