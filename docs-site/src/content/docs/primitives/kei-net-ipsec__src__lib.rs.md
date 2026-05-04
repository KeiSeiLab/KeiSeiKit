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
