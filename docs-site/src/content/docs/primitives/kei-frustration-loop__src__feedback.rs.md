---
title: feedback.rs
path: kei-frustration-loop/src/feedback.rs
dna_hash: sha256:3694646eb17a67fa
language: rust
size_loc: 144
generated: by-keidocs
---

# kei-frustration-loop/src/feedback.rs

User-feedback log — Feedback struct + JSONL append/count/read.

One line in `<user>.feedback.jsonl` records one correction the user made
while reviewing a queued nightly hit. The retrain trigger walks this log
to decide whether the per-user firmware should be rebaked.

Constructor Pattern: this cube only owns the on-disk shape of feedback.
Threshold logic lives in `auto_train.rs`; queue-emission lives in
`nightly.rs`; firmware retraining lives in `firmware.rs`.

## Public API

- One correction the user made about one queued nightly hit.
- The hit identifier the user reviewed (matches `Hit.id` in the queue).
- The original message text (denormalised so log is self-contained).
- Verdict: classifier was right / wrong / a new category emerged.
- Predicted (or new) category name. Empty for `Wrong` with no
- Unix-seconds timestamp of when the user filed the feedback.
- User identifier (the `--user` slug; defaults to `$USER`).
- Verdict the user attached to one queued hit.
- Classifier categorised this hit correctly.
- Classifier mis-categorised — discard this hit from corpus.
- User wants a brand-new category to be tracked. Inner string is the
- `pub fn parse` — Parse the CLI form: `correct`, `wrong`, `new:<slug>`.
- `pub fn append_feedback` — Append one feedback row to `path` as a single JSONL line.
- `pub fn count_pending` — Count rows in `path`. Missing file → 0. Malformed lines are skipped
- `pub fn read_all` — Read every well-formed feedback row from `path`. Missing file → empty.
- Stream JSONL lines from `path`, dropping blanks. Missing file → empty
- Helper for `Label::parse`: handle the `new:<slug>` form and reject

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
