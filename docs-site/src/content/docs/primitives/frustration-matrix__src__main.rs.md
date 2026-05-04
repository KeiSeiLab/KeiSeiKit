---
title: main.rs
path: frustration-matrix/src/main.rs
dna_hash: sha256:f0ecd67290c6934a
language: rust
size_loc: 279
generated: by-keidocs
---

# frustration-matrix/src/main.rs

frustration-matrix — longitudinal user-pushback scanner + firmware trainer
+ likelihood-ratio classifier.

Constructor Pattern: main.rs only dispatches. Work is in cubes:
categories / markdown / jsonl / since / row / scan / report / firmware /
firmware_corpus / firmware_ngram / classifier. CLI shape stable; extend
categories in categories.rs only, firmware behaviour in firmware*.rs,
classifier behaviour in classifier.rs.

## Public API

- Walk chatlogs, apply category regexes, write CSV or JSONL output.
- Skip raw `.jsonl` session transcripts; scan only curated `.md`.
- Drop user messages shorter than N chars before regex match.
- If set, load firmware bundle from this directory and classify
- Classify a single message via the loaded firmware bundle. Prints
- Message to classify. Positional — quote it in shell.
- Drop messages shorter than N chars (see classifier::MIN_LEN).
- Normalized log-ratio threshold (see classifier::THRESHOLD).
- Read scan output, aggregate, print top-N table.
- Train a byte-level n-gram firmware from a corpus directory.
- Context depth. internal calibration knee is 4 on 10-25 MB.
- Fraction of the corpus held out for perplexity. Pass `0.1`
- Compare regex-based (v1) vs firmware-based (v2) classification on a
- Path to `labeled-training-set.jsonl`. Only rows with
- Directory with firmware bundle (`neutral.fw` + per-category `.fw`).
- Output CSV path. One row per `(model, category)`.
- Wire CLI args through the thin `eval::evaluate` orchestrator.
- Classify a single message via the firmware bundle at `dir`. Delegates
- Split `text` at char-boundary `holdout` fraction. Returns (train, test).
- Expand a leading `~/` using $HOME. Absolute/relative paths pass through.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
