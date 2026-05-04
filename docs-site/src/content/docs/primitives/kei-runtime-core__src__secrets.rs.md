---
title: secrets.rs
path: kei-runtime-core/src/secrets.rs
dna_hash: sha256:87d6e6922ae578ef
language: rust
size_loc: 101
generated: by-keidocs
---

# kei-runtime-core/src/secrets.rs

`SecretString` — a zeroising, redacting string newtype.

Stores a sensitive value (password, API key, JWT secret) and ensures:
- `Debug` prints `"<redacted>"` — never the value.
- `Drop` zeroes the heap bytes before deallocation.

The value is exposed only via [`SecretString::expose`], forcing callers
to be explicit about accessing the secret.

## Public API

- A string whose `Debug` impl is redacted and whose `Drop` zeroes memory.
- `pub fn new` — Wrap a value. The caller is responsible for not logging the input.
- `pub fn expose` — Expose the raw value. Name is intentionally verbose.
- Serializes as the literal `"<redacted>"` — never the secret value.
- Always prints `<redacted>` — never the secret value.
- Not `Display` — secrets should not be formatted accidentally.

## Related

- parent: `kei-runtime-core/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
