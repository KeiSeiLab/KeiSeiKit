---
title: lib.rs
path: kei-net-wireguard/src/lib.rs
dna_hash: sha256:2e1780bf106c3883
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-net-wireguard/src/lib.rs

kei-net-wireguard — Wave 9 NetworkMode impl over `wg-quick`/`wg`.

Constructor Pattern: 4 cubes
* `error`   — Error/Result + conversions into `kei_runtime_core::Error`
* `runner`  — `Runner` shell-out seam + `RunOutput` + `SystemRunner`
* `parse`   — `parse_wg_dump` (peer rows from `wg show <iface> dump`)
* `network` — `WireguardMode` (the `NetworkMode` impl)

Brings up a WireGuard interface via `wg-quick up <iface>` (config at
`/etc/wireguard/<iface>.conf` or `$WG_CONFIG_PATH`), tears down via
`wg-quick down <iface>`, and reports peer status by parsing the
tab-separated output of `wg show <iface> dump`. The interface name
defaults to `wg0` and may be overridden via `$WG_IFACE`.

Mode is private — `is_public()` is `false` (WireGuard is a private
mesh, not a public ingress). Mirror of the kei-net-tailscale sibling.

## Related

- parent: `kei-net-wireguard/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
