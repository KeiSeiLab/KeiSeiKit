---
title: openvpn_smoke.rs
path: kei-net-openvpn/tests/openvpn_smoke.rs
dna_hash: sha256:e08c2d066aa39ebb
language: rust
size_loc: 161
generated: by-keidocs
---

# kei-net-openvpn/tests/openvpn_smoke.rs

Smoke tests for `OpenvpnMode`. We use a recording `MockRunner`
instead of `SystemRunner`, so these tests are hermetic — no
`systemctl`, no UNIX socket, no live OpenVPN.

## Public API

- Per-call result. If empty, default to status=0.

## Related

- parent: `kei-net-openvpn/tests`
- imports: kei_net_openvpn, kei_runtime_core, std

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
