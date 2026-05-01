# kei-cortex — Cortex daemon

## What it is

Local HTTP daemon on 127.0.0.1:9797 that backs the cortex-ui Svelte app.
Endpoints: /chat (Anthropic SSE), /tts (ElevenLabs proxy), /stt (local
faster-whisper via Python subprocess), /portrait/stylize (fal.ai Flux),
/pet/:id (TOML manifest CRUD), /ledger, /memory.

## Install

    ./install.sh --profile=cortex

The `cortex` profile provisions six primitives: `kei-cortex` (this daemon),
`cortex-ui` (Svelte frontend), `kei-pet`, `kei-shared`, `kei-ledger`,
`kei-memory`.

After install, a one-shot setup wizard `/cortex-setup` (Wave 24 PR2) will:

- Generate `~/.keisei/cortex.token` (32 random hex bytes, chmod 600).
- Run `pip install -r scripts/requirements.txt` (faster-whisper==1.2.1).
- Pre-download the `base.en` whisper model with SHA-256 verification.
- Build the cortex-ui Svelte bundle (`pnpm install && pnpm build`).
- Register a launchd plist (macOS) or systemd user unit (Linux).

PR1 (this PR) only registers both primitives with the installer so the
profile resolves and the manifest parses. PR2 wires the wizard; PR3+ adds
multi-tenant auth via `kei-auth`.

## Env vars

Required in `~/.claude/secrets/.env` (RULE 0.8 — never hardcode keys):

- `ANTHROPIC_API_KEY`      — /chat endpoint
- `ELEVENLABS_API_KEY`     — /tts endpoint
- `FAL_KEY`                — /portrait/stylize endpoint

Optional:

- `KEI_WHISPER_MODEL`      — default `base.en`
- `KEI_WHISPER_DEVICE`     — default `auto`
- `KEI_WHISPER_PYTHON`     — absolute path to python3
- `KEI_WHISPER_WORKER`     — absolute path to whisper_worker.py
- `KEI_WHISPER_LOCAL_ONLY` — `1` to forbid HF downloads

## Host requirements

Soft-checked by the installer when `kei-cortex` / `cortex-ui` are in scope:

- `python3` >= 3.9 and `pip3` (whisper worker subprocess + deps)
- `ffmpeg` on PATH (faster-whisper audio demux)
- `node` >= 18 and `pnpm` (cortex-ui build, cortex-ui only)

The Python exception for whisper_worker.py is tracked under RULE 0.2
exception #6 (external-binding-only; faster-whisper is a Python-only lib).
The Rust daemon invokes it as a subprocess — no Python in the daemon core.

## Multi-tenant (Wave 25)

Current: single bearer per daemon, one user per process. Wave 25 wires
`kei-auth` for N users per daemon with session-based auth.

## See also

- `_ts_packages/packages/cortex-ui/` — browser frontend
- `_primitives/_rust/kei-pet/` — pet manifest schema consumed by /chat
- `_primitives/_rust/kei-auth/` — multi-tenant target (not yet wired)
