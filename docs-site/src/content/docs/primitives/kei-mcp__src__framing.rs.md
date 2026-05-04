---
title: framing.rs
path: kei-mcp/src/framing.rs
dna_hash: sha256:5eb706f5891679cf
language: rust
size_loc: 163
generated: by-keidocs
---

# kei-mcp/src/framing.rs

Line framing for stdio JSON-RPC — bounded by `MAX_MESSAGE_BYTES`.

MISS-8 hardening. A single JSON-RPC line is read with a hard 10 MB cap so
a malicious or runaway peer cannot OOM the server by sending one huge
line. The cap is enforced INCREMENTALLY — once `MAX_MESSAGE_BYTES`
payload bytes have been pulled into the buffer, we stop allocating and
only DRAIN bytes (without storing them) until the next newline. This
keeps the resident set bounded at ~10 MB per oversize event regardless
of how big the peer's line actually is.

The reader must implement `AsyncBufRead` so we can use the buffered
`fill_buf`/`consume` interface to peek-and-drain without copying into
a growing `Vec<u8>` after the cap is hit.

## Public API

- `pub const MAX_MESSAGE_BYTES` — Hard cap on a single JSON-RPC line (10 MB). Anything larger is rejected
- One outcome of a single bounded read.
- Upstream is closed; caller should exit the loop.
- Line was empty / whitespace-only; caller should retry.
- Valid line within the cap.
- Line exceeded `MAX_MESSAGE_BYTES`; the rest of the line has been
- Read one line from `reader` with a hard `MAX_MESSAGE_BYTES` cap.
- 11 MB line → cap hit → Oversize. Buffer never exceeds the cap.
- Exactly-at-cap line is accepted (boundary).

## Related

- parent: `kei-mcp/Cargo.toml`
- imports: anyhow, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
