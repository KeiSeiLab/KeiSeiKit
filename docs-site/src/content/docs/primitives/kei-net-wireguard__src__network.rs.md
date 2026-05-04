---
title: network.rs
path: kei-net-wireguard/src/network.rs
dna_hash: sha256:5b2bcf5c79b45676
language: rust
size_loc: 165
generated: by-keidocs
---

# kei-net-wireguard/src/network.rs

[`WireguardMode`] — the `NetworkMode` impl.

Brings up a WireGuard interface via `wg-quick up <iface>` (config at
`/etc/wireguard/<iface>.conf` or `$WG_CONFIG_PATH`), tears it down via
`wg-quick down <iface>`, and reports peer status by parsing
`wg show <iface> dump`.

Shell-out goes through the [`Runner`] seam so smoke tests substitute a
recording mock without touching the host. The async surface (NetworkMode
is `async_trait`) is bridged via `tokio::task::spawn_blocking` because
the underlying `Runner` is sync (mirror of the kei-llm-mlx pattern).

## Public API

- `pub struct WireguardMode` — Private-mesh WireGuard adapter.
- `pub fn new` — Explicit constructor — used by smoke tests with a [`MockRunner`] and
- `pub fn from_env` — Build from environment: `$WG_IFACE` (default `wg0`).
- Run `wg-quick <subcommand> <iface>` and return Ok on exit code 0.
- Run `wg show <iface> dump` and return parsed peer rows.
- Bring the interface up. `cfg` is accepted for API parity with the

## Related

- parent: `kei-net-wireguard/Cargo.toml`
- imports: async_trait, crate, kei_runtime_core, std

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
