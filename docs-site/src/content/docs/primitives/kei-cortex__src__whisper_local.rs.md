---
title: whisper_local.rs
path: kei-cortex/src/whisper_local.rs
dna_hash: sha256:b1b8e1b7ef9e0a20
language: rust
size_loc: 197
generated: by-keidocs
---

# kei-cortex/src/whisper_local.rs

Local Whisper transcription via faster-whisper Python subprocess.

RULE 0.2 exception #6: faster-whisper (CTranslate2) has no native Rust
equivalent that accepts an arbitrary audio blob and returns text. The
daemon spawns `<python3> scripts/whisper_worker.py <tmpfile>` and reads
the transcript from stdout. Audio bytes are written to a tempfile
because faster-whisper reads from a path (ffmpeg demuxes).

Configurable via env:
`KEI_WHISPER_MODEL`  — default `base.en`, can be `medium.en` / `large-v3`.
`KEI_WHISPER_DEVICE` — default `auto`, can be `cpu` / `cuda` / `mps`.
`KEI_WHISPER_WORKER` — override path to the Python worker script
(defaults to the repo-relative
`kei-cortex/scripts/whisper_worker.py`).
`KEI_WHISPER_PYTHON` — override `python3` binary (absolute path). If
unset we resolve `python3` via `which::which`
once at first call and cache the result.

## Public API

- Upper bound on any single transcribe call. 120 s suffices for a minute
- Cache for the resolved python3 path. `None` means "we tried and failed"
- Errors the caller needs to distinguish (mapped to HTTP by `stt.rs`).
- Transcribe a blob by writing it to a tempfile and spawning the worker.
- Pick a filename extension from the MIME so ffmpeg (invoked inside
- Write audio bytes to a tempfile the worker can read.
- Find the worker script path: env override wins, otherwise relative to
- Resolve the `python3` interpreter path. Honors `KEI_WHISPER_PYTHON` first
- Spawn the Python worker on the given audio file, capture stdout.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tempfile, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
