---
title: network.rs
path: kei-net-ipsec/src/network.rs
dna_hash: sha256:edcabfe6cf76821a
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-net-ipsec/src/network.rs

[`IpsecMode`] — DNA-bearing [`NetworkMode`] impl that brings a
strongSwan child SA up / down via `swanctl`.

Mechanism (per RFC: each step is a single `swanctl` invocation routed
through [`Runner`] for testability):

* `configure`  → `swanctl --load-all` (refresh `/etc/swanctl/`),
then `swanctl --initiate --child <child_name>`.
* `teardown`   → `swanctl --terminate --child <child_name>`.
* `peers`      → `swanctl --list-sas`, parsed by [`crate::parse`].

`is_public() = true`. Sibling tailscale / wireguard NetworkMode adapters
return `false`.

## Public API

- `pub const DEFAULT_CONFIG_DIR` — Default config root for `swanctl` (overridable via env
- `pub const DEFAULT_CHILD_NAME` — Default child SA name (overridable via env `IPSEC_CHILD_NAME`).
- `pub struct IpsecMode` — strongSwan / swanctl `NetworkMode`. Construction injects a
- `pub fn new` — Construct with explicit runner + child name.
- `pub fn from_env` — Construct from environment: `IPSEC_CHILD_NAME` (default `home`)
- `pub fn child_name` — Inspect the child SA name this mode operates on.
- `pub fn config_dir` — Inspect the swanctl config directory (informational).

## Related

- parent: `kei-net-ipsec/Cargo.toml`
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
