---
title: http_status.rs
path: kei-arch-map/src/evidence/http_status.rs
dna_hash: sha256:6e6d82175e1a8798
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-arch-map/src/evidence/http_status.rs

HTTP status evidence with hardened SSRF guard.

Security fix A (SSRF via DNS): the original implementation only blocked
IP literals in the URL. Hostnames like `evil.example` that resolve to
169.254.169.254 (AWS metadata) bypassed the check entirely. The
resolver now expands every hostname to its `SocketAddr` set BEFORE the
request is dispatched and rejects on the first unsafe address.

## Public API

- Reject loopback / private / link-local / unspecified / multicast literal IPs.
- Validate URL scheme + host literal IPs. Hostnames pass through here
- Resolve `url`'s host via the OS resolver and verify EVERY returned

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: std, url

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
