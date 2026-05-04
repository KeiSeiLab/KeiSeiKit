---
title: ip_filter.rs
path: kei-cortex/src/tool/ip_filter.rs
dna_hash: sha256:a936c8a148c2e66d
language: rust
size_loc: 162
generated: by-keidocs
---

# kei-cortex/src/tool/ip_filter.rs

SSRF protection — IP-range deny-list.

Composition: pure predicates over `IpAddr`. The webfetch tool
resolves the requested host, then refuses if ANY resolved IP is
private / loopback / link-local / CGNAT / IPv6 ULA.

Why the host alone is insufficient: DNS rebinding attacks (a name
resolves to 169.254.169.254 the second time after the validator
checked it once) and CNAME chains require us to filter at IP level
immediately before the connect. Combine this with a single-shot
resolve + connect via `IpAddr` directly to eliminate the gap.

AWS IMDS sits at `169.254.169.254/32` (link-local). Tailscale uses
CGNAT `100.64.0.0/10`; intentional reject — escape via opt-in env
`KEI_WEBFETCH_ALLOW_PRIVATE=1`.

## Public API

- `pub fn is_blocked_ip` — True iff `ip` falls in any blocked range.
- IPv4 ranges blocked by SSRF policy.
- IPv6 ranges blocked by SSRF policy.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
