# Training data

Training corpora and pre-trained firmwares are NOT shipped with the
public crate. The pipeline is build-your-own:

1. Collect labeled user lines per category in
   `data/corpus/<category>/msg-*.txt` and a baseline corpus in
   `data/corpus/neutral/msg-*.txt`.
2. Run `cargo run --release --bin train -- --corpus data/corpus/ --out data/firmwares/`.
3. Run `cargo test --release` to validate the trained firmwares
   against `tests/firmware.rs` thresholds.

The five built-in categories (`conservative-framing`,
`paradigm-slippage`, `data-contamination`, `repeat-signal`,
`frustration-tone`) are defined in `src/categories.rs` as regex
triggers; the n-gram firmware step is optional and only adds the
likelihood-ratio score on top of the regex weights.

`data/.gitignore` already excludes `corpus/`, `firmwares/*.fw`, and
generated artifacts so a populated working tree stays out of git.
