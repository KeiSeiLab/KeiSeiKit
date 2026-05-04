---
title: cli.rs
path: kei-decompose/src/cli.rs
dna_hash: sha256:152ff0bba1d3422e
language: rust
size_loc: 122
generated: by-keidocs
---

# kei-decompose/src/cli.rs

Clap definitions — 5 subcommands.

Subcommand surface kept stable; main.rs dispatches to module entrypoints.

## Public API

- Forced-format flag value.
- Auto-detect which parser claims this MD file.
- Path to the markdown file.
- Parse using detected (or forced) format → JSON Action[].
- Path to the markdown file.
- Force a specific format. Default: auto-detect.
- Parse + emit one task.toml per Action under <out>.
- Path to the markdown file.
- Output directory for the emitted task.toml files.
- Force a specific format. Default: auto-detect.
- Full chain: parse → emit → kei-spawn each → kei-ledger pre-fork.
- Path to the markdown file.
- Skip kei-spawn invocation; only emit and report intent.
- Cap the number of actions dispatched.
- Force a specific format. Default: auto-detect.
- Skip kei-ledger pre-fork registration.
- List registered parsers + their detection signatures.
- Walk rule files, split into sections, register each in kei-registry.
- Root directory containing rule `.md` files.
- Path to the registry SQLite database.
- Directory where fragment `.md` files are stored on disk.
- Print what would be registered/written without doing either.
- Re-extract ALL existing rule-type rows in the registry to the

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: clap, std

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
