---
title: parse.rs
path: kei-net-ipsec/src/parse.rs
dna_hash: sha256:5006cb920849b2f8
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-net-ipsec/src/parse.rs

Parser for `swanctl --list-sas` text output.

## Grammar (informal)

Each Security Association occupies a stanza beginning with a header:

```text
<conn>: #<id>, ESTABLISHED, <proto>, <spi-stuff>...
local  '<id>' @ <local_ip>[port]
remote '<id>' @ <remote_ip>[port]
<encr_alg=...>
<child_sa_lines>
bytes_i (... , N bytes), packets_i (M packets)
bytes_o (... , N bytes), packets_o (M packets)
```

We accept any whitespace lead-in. We only emit a [`PeerStatus`] for
stanzas that contain the literal token `ESTABLISHED` — `CONNECTING`,
`INSTALLED`, `REKEYING`, etc. are skipped (per spec: "ignore partial
SAs").

`last_seen_ms`: strongSwan does not surface a clean per-SA last-handshake
timestamp in `--list-sas`, so we set it to **the current wall-clock**
when the SA reports `ESTABLISHED`, and `0` when the SA is partial /
ignored. Documented behaviour, not invented data.

Bytes parsing tolerates the human-friendly formatter: strongSwan prints
`bytes_i (1.04K, 1067 bytes)`. We pull the integer that immediately
precedes the literal `bytes` token; suffix forms like `1.04K` are
ignored in favour of the exact byte count.

## Public API

- `pub fn parse_sas_output` — Parse the full `swanctl --list-sas` stdout into one [`PeerStatus`] per
- A new stanza starts at any non-indented line containing `: #` (the
- Extract `<remote_ip>` from a line shaped like
- Extract the integer byte count following the `<key>` token. Tolerant

## Related

- parent: `kei-net-ipsec/Cargo.toml`
- imports: kei_runtime_core, std

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
