---
title: dna.rs
path: kei-agent-runtime/src/dna.rs
dna_hash: sha256:b76a6f803a667ea0
language: rust
size_loc: 190
generated: by-keidocs
---

# kei-agent-runtime/src/dna.rs

Layer G — DNA identity for agent invocations.

DNA format:  `<role>::<caps-bitmap>::<scope-hash>::<body-hash>-<nonce>`
where
- `role`        — role slug, e.g. `edit-local`
- `caps-bitmap` — hyphen-separated 2-char atom codes (ordered, from
the resolved capability list)
- `scope-hash`  — 8-char truncated SHA-256 of canonicalised scope fields
(32-bit; widened from 16-bit to push birthday collision
threshold from ~256 to ~65k agents per role+caps group)
- `body-hash`   — 8-char truncated SHA-256 of `task.body.text` (32-bit)
- `nonce`       — 8-char hex from `rand::random::<u32>()` (full 32-bit
entropy; was 16-bit pre-2026-04 H4/M4/S3 widening)

Constructor Pattern: one cube = DNA identity primitive only. No I/O.

Round-trip: `compose` → `render` → `parse` → equal.
Parse accepts both shipped DNA strings and hand-written ones; it enforces
the 5-segment shape but tolerates arbitrary (non-empty) segment content
so future schema extensions don't break old ledger rows. For rolling
upgrade, 4-hex legacy hash/nonce values still parse silently — the
fallback is a successful parse path, not an error.

Wire-format SSoT lives in `kei_shared::dna` — `render()` delegates to
`kei_shared::compose_dna` so the format string exists in one place.
Strict parser primitives from `kei_shared` (`parse_dna`, `ParsedDna`,
`is_hex8`) are re-exported for callers that want width validation;
the in-crate lenient `Dna::parse` stays for rolling-upgrade support.

## Public API

- Re-export of the strict wire-format parser from `kei_shared::dna`.
- `pub const CAP_CODES` — Capability-name → 2-char atom code lookup.
- Agent DNA — composition fingerprint.
- Error during lenient rolling-upgrade DNA parsing.
- `pub fn compose` — Build DNA from a task + already-resolved role.
- `pub fn render` — Render to the canonical wire format. Delegates the format-string
- `pub fn parse` — Parse a DNA string. Lenient on segment content, strict on shape.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, sha2, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
