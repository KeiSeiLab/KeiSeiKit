---
title: wireguard_smoke.rs
path: kei-net-wireguard/tests/wireguard_smoke.rs
dna_hash: sha256:6795a6fcd6ada33a
language: rust
size_loc: 130
generated: by-keidocs
---

# kei-net-wireguard/tests/wireguard_smoke.rs

Smoke tests for `kei-net-wireguard`. Drive the `WireguardMode` impl
through a recording [`MockRunner`] — never invokes a real `wg-quick`.

## Public API

- Records every `(cmd, args)` invocation and replays a programmable

## Related

- parent: `kei-net-wireguard/tests`
- imports: anyhow, kei_net_wireguard, kei_runtime_core, std

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
