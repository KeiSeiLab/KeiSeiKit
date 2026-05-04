# Breaking changes

> User-visible wire-format / API breaks. One section per change.
> Newest first.

---

## 2026-05-04 — DNA wire-format width bump 32→64 bits (Wave 7C)

**Affected crates:** `kei-shared`, `kei-runtime-core`, `kei-agent-runtime`.
**Affected commit:** `319017c fix(security/7C): bump DNA hex truncation 32→64 bits across substrate (HIGH)`.

### What changed

The substrate DNA wire format

```
<role>::<caps>::<scope_sha>::<body_sha>-<nonce>
```

now requires each of `scope_sha`, `body_sha`, `nonce` to be **16 ASCII hex
characters** (64-bit width). Pre-Wave-7C builds used **8 hex characters**
(32-bit). Old DNAs no longer parse.

### Why

At 32-bit per segment the birthday-bound collision threshold for agent
fingerprints sharing the same `role+caps+scope+body` quadruple is
~65 000 creations. The substrate is growing past that horizon; collisions
silently identify two distinct agents as "the same agent" in the registry.
The 64-bit width raises the threshold to ~4 billion.

### API renames

| Pre-Wave-7C | Post-Wave-7C |
|---|---|
| `kei_shared::dna::is_hex8` | `kei_shared::dna::is_hex16` |
| `kei_shared::dna::check_hex8` (private) | `kei_shared::dna::check_hex16` (private) |
| `kei_runtime_core::dna::sha256_hex8` (private) | `kei_runtime_core::dna::sha256_hex16` (private) |
| `kei_runtime_core::dna::random_hex8_lower` (private) | `kei_runtime_core::dna::random_hex16_lower` (private) |
| `DnaError::HexWidth` "hex8" message | "hex16" message |

`is_hex8` is **retained** as `#[deprecated]` shim returning `false` for
ALL inputs. This is intentional fail-closed: legacy callers that compiled
against the old API hard-fail on every call rather than silently accept
new 16-char DNAs. Migrate to `is_hex16`.

### Migration path

Substrate is pre-v1 with no production data persistence. There is **no
migration shim**. If you have:

- **Existing on-disk DNAs**: regenerate. `parse_dna` rejects width
  mismatch, so old DNAs hard-fail rather than silently mis-parse.
- **Hardcoded 8-hex literals in tests**: extend to 16-hex
  (`"12345678"` → `"1234567812345678"`).
- **Code that calls `is_hex8`**: rename to `is_hex16`. The deprecated
  shim will keep your build going during the transition but every call
  returns `false`.

### Out of scope

The same crypto-hash truncation pattern exists in other crates that
were NOT covered by this change:

- `kei-comments::derive_id` (was 64-bit, bumped to 128-bit in a follow-up
  Wave 10 commit; SQLite primary key, no wire-format break since the
  schema column is `TEXT`).
- `keidocs::compute_dna` (already 64-bit, accurate doc, scale-safe).
- `kei-artifact::hash` (full 256-bit, no truncation).

Future width bumps on those will get their own entries here.

[REAL: cargo test -p kei-shared -p kei-runtime-core -p kei-agent-runtime
       -p keidocs 2026-05-04, 0 FAILED across 16 test results]
