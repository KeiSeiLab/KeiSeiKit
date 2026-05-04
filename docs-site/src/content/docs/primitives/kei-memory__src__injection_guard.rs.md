---
title: injection_guard.rs
path: kei-memory/src/injection_guard.rs
dna_hash: sha256:06e62842e5abbc2f
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-memory/src/injection_guard.rs

Injection / exfiltration guard for memory entries.

Constructor Pattern: scan logic only; pattern definitions live in
`injection_patterns.rs`.

## Wire-points (3 paths protected, P2.1.b lock 2026-04-28)

1. `ingest::insert_event` — REAL memory writes from agent JSONL
transcripts. Each event message is scanned before it is persisted
into the `events` table. Block-tier hits short-circuit insertion.
2. `kei-pet::memory::record_interaction` — user-facing pet
conversation memory. Uses a substring/char-only sibling guard
(`kei_pet::injection_check`) to avoid a regex dep bump on the pet
crate. Block-tier coverage mirrors this module's prompt-override
+ invisible-unicode + PEM-marker rules.
3. `cmd_backlog --add` — RULE 0.14 audit-CRUD. Backlog items are
rendered into self-audit reports verbatim; malicious content
survives that path the same way it would survive insert_event.

All three paths use the same `Severity::Block` semantics: a hit
results in early-return / persistence-skip, with the finding logged.

## Rationale

Memory entries are injected verbatim into the system prompt. Any
prompt-override fragment, role-prefix, ChatML tag, invisible bidi
codepoint, hardcoded credential, or large base64 attestation blob
survives that injection and becomes effective text the model reads.
The scan treats these as untrusted input and rejects them.

## Bypass

`KEI_MEMORY_SKIP_GUARD=1` skips the scan after logging an explicit
warning to stderr. Intended for one-off recovery — never the default.

## Public API

- One pattern hit. Severity drives whether the call site rejects.
- `pub fn scan` — Scan `content` for prompt-injection / secret-leak patterns.
- Like `scan` but returns every finding (Block + Warn). Useful for

## Related

- parent: `kei-memory/Cargo.toml`
- imports: crate

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
