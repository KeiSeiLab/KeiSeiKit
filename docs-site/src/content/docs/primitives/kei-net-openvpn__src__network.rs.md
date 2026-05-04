---
title: network.rs
path: kei-net-openvpn/src/network.rs
dna_hash: sha256:40335d1f9e20819a
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-net-openvpn/src/network.rs

`OpenvpnMode` — `NetworkMode` impl over a host-resident OpenVPN
server managed by `systemd`.

Constructor surface:
* [`OpenvpnMode::with_runner`] — explicit runner, service name,
and management-socket path. Used by smoke tests with a mock
runner.
* [`OpenvpnMode::from_env`]    — reads `OPENVPN_SERVICE_NAME`
(default `server`) and `OPENVPN_CONFIG_PATH` (default
`/etc/openvpn/server/<name>.conf`); the management-socket path
is derived as `/var/run/openvpn/<name>.sock` unless an explicit
`OPENVPN_MGMT_SOCKET` is set. Uses [`SystemRunner`].

NetworkMode wire:
* `configure(_)` → `systemctl start openvpn-server@<name>`
* `teardown()`   → `systemctl stop  openvpn-server@<name>`
* `peers()`      → if `mgmt_socket` is `Some(path)`, connect via
`tokio::net::UnixStream`, send `status 2\r\n`,
read until `\nEND\n` (or EOF), then
`parse_status_output`. If `None`, return
`Ok(vec![])`.
* `is_public()`  → `true` (OpenVPN exposes a routable UDP/TCP
endpoint by default).

## Public API

- `pub fn with_runner` — Build with explicit runner + service name + paths. `parent` is
- `pub fn from_env` — Build from env. Required: none (all have defaults).

## Related

- parent: `kei-net-openvpn/Cargo.toml`
- imports: crate, kei_runtime_core, std, tokio

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
