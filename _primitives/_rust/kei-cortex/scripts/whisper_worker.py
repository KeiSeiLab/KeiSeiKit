#!/usr/bin/env python3
# RULE 0.2 exception #6: external binding only exists in Python (faster-whisper
# wraps CTranslate2; no equivalent Rust crate accepts a generic audio path +
# returns transcript as of 2026-04-24). Invoked as a subprocess by the
# Rust daemon's STT handler.
#
# Contract:
#   argv[1] = path to audio file (webm/wav/mp3/m4a/ogg — ffmpeg handles it)
#   env    KEI_WHISPER_MODEL       (optional, default "base.en"; use
#                                   "medium.en" or "large-v3" for higher quality)
#           KEI_WHISPER_DEVICE      (optional, default "auto"; "cpu"/"cuda"/"mps")
#           KEI_WHISPER_LOCAL_ONLY  (optional, default "0"; "1" disables any
#                                   Hugging Face download — model MUST already
#                                   be in the local cache)
#   stdout = transcript string (plain text, utf-8, single line)
#   stderr = progress / model download messages
#   exit  0 ok; 1 on any error (error message on stderr)
"""Local Whisper transcription worker."""
from __future__ import annotations

import os
import sys


def main(argv: list[str]) -> int:
    """Entry point. Returns process exit code."""
    if len(argv) < 2:
        print("usage: whisper_worker.py <audio_path>", file=sys.stderr)
        return 1
    audio_path = argv[1]
    if not os.path.isfile(audio_path):
        print(f"audio file not found: {audio_path}", file=sys.stderr)
        return 1
    try:
        from faster_whisper import WhisperModel
    except ImportError:
        print(
            "faster-whisper not installed: "
            "pip install -r scripts/requirements.txt",
            file=sys.stderr,
        )
        return 1

    model_name = os.environ.get("KEI_WHISPER_MODEL", "base.en")
    device = os.environ.get("KEI_WHISPER_DEVICE", "auto")
    compute_type = "int8" if device == "cpu" else "default"
    local_only = os.environ.get("KEI_WHISPER_LOCAL_ONLY", "0") == "1"

    try:
        model = WhisperModel(
            model_name,
            device=device,
            compute_type=compute_type,
            local_files_only=local_only,
        )
    except Exception as e:  # noqa: BLE001
        print(f"model load failed ({model_name}): {e}", file=sys.stderr)
        return 1

    try:
        segments, _info = model.transcribe(audio_path, vad_filter=True)
        text = " ".join(s.text.strip() for s in segments).strip()
    except Exception as e:  # noqa: BLE001
        print(f"transcribe failed: {e}", file=sys.stderr)
        return 1

    print(text)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
