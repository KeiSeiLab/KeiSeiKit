---
title: injection_check_textual.rs
path: kei-pet/src/injection_check_textual.rs
dna_hash: sha256:18b02fa7410eae7a
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-pet/src/injection_check_textual.rs

Textual rules for `injection_check`.

Constructor Pattern: prompt-override, ChatML, system-prefix, secret
markers, exfil substrings — all lower-case substring tests on the
input. No regex; no allocations beyond a single `to_lowercase`.

## Public API

- Detect prompt-override / role-prefix / ChatML payloads.
- Detect PEM private-key markers.
- Detect exfiltration shapes (bearer-token + URL, api_key + URL,
- True if any line, after trimming leading whitespace, starts with `system:`.
- True if `hay` contains all needles in order.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
