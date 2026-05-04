---
title: auth.rs
path: kei-cortex/src/auth.rs
dna_hash: sha256:30ea20d714f39bac
language: rust
size_loc: 155
generated: by-keidocs
---

# kei-cortex/src/auth.rs

Token lifecycle: generate / save (chmod 600) / load / validate.

The bearer token is a 32-byte random value rendered as 64 lowercase hex
characters. It is stored at `~/.keisei/cortex.token` with file mode
0600 on unix. Reads trim trailing whitespace so a caller-added newline
does not corrupt comparisons.

## Public API

- `pub const TOKEN_BYTES` — Length of the raw token in bytes (32 → 64 hex chars).
- `pub const TOKEN_HEX_LEN` — Length of the hex-rendered token (always `2 * TOKEN_BYTES`).
- Errors surfaced by this module.
- `pub fn generate_token` — Generate a fresh 32-byte token rendered as 64 lowercase hex characters.
- Lowercase hex encoder; avoids pulling `hex` crate for one function.
- `pub fn save_token` — Write `token` to `path`, creating parent directories and enforcing
- `pub fn load_token` — Read the token from `path`, trimming trailing whitespace, and validate it.
- `pub fn validate_hex` — Validate the token is exactly `TOKEN_HEX_LEN` lowercase-or-uppercase hex.
- `pub fn tokens_match` — Constant-time-ish comparison (length + byte-level xor fold).
- Build a unique temp path next to `path`: `<path>.<nanos>.tmp`.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: rand, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
