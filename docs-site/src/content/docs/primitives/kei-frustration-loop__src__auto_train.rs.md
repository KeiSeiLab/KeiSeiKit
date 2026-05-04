---
title: auto_train.rs
path: kei-frustration-loop/src/auto_train.rs
dna_hash: sha256:76470c0a86dfde4b
language: rust
size_loc: 151
generated: by-keidocs
---

# kei-frustration-loop/src/auto_train.rs

Auto-retrain trigger logic.

When the per-user feedback log crosses `threshold` rows, the orchestrator
invokes `auto_train`: rebuild the corpus from confirmed-correct hits
(treating `Wrong` and `NewCategory(_)` as exclusions / extension only),
retrain the firmware, and atomic-swap it into place.

Default threshold = 20 (see `DEFAULT_THRESHOLD`); overridable via the
`KEI_FRUSTRATION_THRESHOLD` env var or the `--threshold` CLI flag.

Constructor Pattern: this cube only orchestrates `feedback`,
`frustration_matrix::firmware`, `frustration_matrix::firmware_corpus`,
`persistence`. No format decisions.

## Public API

- `pub const DEFAULT_THRESHOLD` — Default feedback-row count at which `should_retrain` flips to `true`.
- `pub const THRESHOLD_ENV` — Env var name that overrides `DEFAULT_THRESHOLD`.
- Result returned to the CLI for the `auto-train` subcommand.
- True iff a retrain happened (false = under threshold or no corpus).
- Total UTF-8 chars used to train the new firmware.
- Path of the per-user firmware after the swap.
- Feedback row count seen at trigger time.
- Threshold the count was compared against.
- `pub fn resolve_threshold` — Resolve effective threshold: explicit `cli_threshold` (if `Some`) wins,
- `pub fn should_retrain` — True iff the feedback log at `path` has at least `threshold` rows.
- `pub fn auto_train` — Rebuild the per-user firmware from the supplied corpus + feedback log.
- Concatenate base-corpus text with every kept feedback message into one
- Pull every kept feedback message body into one newline-joined string.
- Decide whether one feedback row contributes to the new corpus.
- Save firmware via tmp-sibling + atomic_write so a crash mid-write

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, crate, frustration_matrix, serde, std

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
