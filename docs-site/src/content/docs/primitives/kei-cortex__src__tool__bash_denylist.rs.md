---
title: bash_denylist.rs
path: kei-cortex/src/tool/bash_denylist.rs
dna_hash: sha256:92a544cb5ef7067a
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-cortex/src/tool/bash_denylist.rs

Bash deny-list — split into argv0 + substring layers.

Layer 1 — `BANNED_ARGV0`: full-string match against argv[0] AFTER
shell tokenization. Catches `sudo`, `/bin/sh`, `bash`, etc. even
when wrapped in quotes (`s'u'do`), backslash escapes (`\sudo`),
command substitution (`$(echo s)udo`) — because the tokenizer
resolves all of those to the same argv0 string.

Layer 2 — `ALLOWED_ARGV0`: explicit allow-list of read-only / safe
binaries. Anything not on this list is rejected. This is the
default-deny gate; it makes Layer 1 belt-and-suspenders.

Layer 3 — `BANNED_SUBSTRINGS`: defense-in-depth against patterns the
tokenizer cannot detect (`> /etc/`, `:(){`, `rm -rf /`, …) — these
match against the raw command string. Final guard for cases the
tokenizer misses (e.g. shell features that sneak past argv split).

Order of evaluation in `bash::run`:
1. tokenize → check argv0 against BANNED_ARGV0 (deny early)
2. check argv0 against ALLOWED_ARGV0 (deny if not on list)
3. scan raw string for BANNED_SUBSTRINGS (deny on any hit)
4. enforce that the command is single-statement (no `;`/`&&`/`||`)

## Public API

- `pub const BANNED_ARGV0` — Argv0 values that are ALWAYS denied even before allow-list check.
- `pub const ALLOWED_ARGV0` — Argv0 values explicitly permitted. The tokenizer-based check rejects
- `pub const BANNED_SUBSTRINGS` — Defense-in-depth substrings checked against the RAW command string.
- `pub fn has_pipe_to_shell` — Test whether the substring `curl …| sh` pattern appears in cmd.

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
