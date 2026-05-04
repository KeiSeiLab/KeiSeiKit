---
title: firmware_corpus.rs
path: frustration-matrix/src/firmware_corpus.rs
dna_hash: sha256:6738a04af9ac4302
language: rust
size_loc: 132
generated: by-keidocs
---

# frustration-matrix/src/firmware_corpus.rs

Corpus loader — walks a directory, concatenates training text.

Dispatch by extension:
* `.md`         — full body minus `### Assistant` blocks
* `.txt`        — raw body
* `.jsonl`      — user turns only (via existing `jsonl::parse_user_lines`)

Files separated by `\n` in the resulting buffer so n-grams don't bleed
across file boundaries (single-char gap is enough — the alphabet builder
collects `\n` like any other char, and its unigram drops below min_count
only if the corpus has no newlines at all).

Constructor Pattern: this cube only wires `fs` + existing parsers.

## Public API

- Kind of training file, matched on extension.
- `pub fn load_corpus_text` — Load and concatenate all training text under `root`. Returns one big
- Walk `root`, return every (path, kind) pair in deterministic order
- `pub fn strip_assistant_blocks` — Drop every line starting with `### Assistant` (or `## Assistant` /

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
