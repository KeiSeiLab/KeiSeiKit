---
title: bash.rs
path: kei-cortex/src/tool/bash.rs
dna_hash: sha256:6b9e785027a1c7c1
language: rust
size_loc: 286
generated: by-keidocs
---

# kei-cortex/src/tool/bash.rs

`bash` tool — sandboxed shell execution.

Composition: tokenize the command via `shell-words` → check argv0
against allow-list / deny-list → scan raw string for known-bad
substrings → reject multi-statement chains → spawn `/bin/sh -c`
under tokio with a 60-second wall-clock cap.

Why tokenize first: substring deny-list bypasses (`s${IFS}udo`,
`\sudo`, `s'u'do`, `$(echo s)udo`, `cat /etc/sh''adow`) all collapse
to the same argv string AFTER shell parses them. Checking that
resolved argv against a fixed allow-list closes the bypass class.

Layered defense per `bash_denylist.rs`:
1. tokenize — argv0 banned → deny
2. allow-list — argv0 not allowed → deny
3. multi-statement (`;`/`&&`/`||`) → deny
4. raw-string substring scan → deny on any hit

See `tests/bash_sandbox_denies.rs` for the full bypass corpus.

## Public API

- Reject commands per the layered defense in `bash_denylist`.
- Layer 4 — raw-string scan. Catches shell features the tokenizer
- Layers 1-3 — split argv on shell statement separators and check
- True for shell tokens that separate statements within a single
- Check that `chunk[0]` (argv0) is on the allow-list AND not on the
- Basename of `cmd` for allow-list matching. `/bin/cat` → `cat`.
- Drain stdout + stderr concurrently to avoid pipe-buffer deadlock,

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: serde, serde_json, std, tokio

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
