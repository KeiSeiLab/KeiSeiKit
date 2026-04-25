# Phase 0 — Preflight

Detect the host OS, confirm the daemon binary and setup primitive exist,
and print a 4-line summary. This phase has ZERO user interaction — it
either succeeds silently and continues, or it hard-fails with a single
actionable error message pointing at `./install.sh --profile=cortex`.

## 0a — OS detection

Run:

```bash
uname -s
```

Map to `OS`:

- `Darwin`  → `OS=macos`
- `Linux`   → `OS=linux`
- anything else → HARD FAIL with:

```
Unsupported OS: <uname output>. kei-cortex currently targets macOS and
Linux only. Windows users: run under WSL2 (Ubuntu). Re-run /cortex-setup
from the WSL shell.
```

## 0b — Daemon binary probe

Compute `BIN_PATH`:

```bash
BIN_PATH="$HOME/.claude/agents/_primitives/_rust/kei-cortex/target/release/kei-cortex"
```

Check existence + executable bit:

```bash
if [ ! -x "$BIN_PATH" ]; then
  echo "MISSING: $BIN_PATH" >&2
  exit 1
fi
```

If missing, HARD FAIL with this exact message (RULE 0.4 — do not invent
an alternative path):

```
kei-cortex binary not found at:
  <BIN_PATH>

The cortex profile has not been installed. Run:

  ./install.sh --profile=cortex

from your KeiSeiKit repo root, then re-run /cortex-setup. This wizard
cannot create the binary itself — it is built by the installer.
```

STOP the wizard. Do NOT proceed to Phase 1.

## 0c — Setup primitive probe

Compute `SETUP_PATH`:

```bash
SETUP_PATH="$HOME/.claude/agents/_primitives/kei-cortex-setup.sh"
```

Check existence + executable bit:

```bash
if [ ! -x "$SETUP_PATH" ]; then
  echo "MISSING: $SETUP_PATH" >&2
  exit 1
fi
```

If missing, HARD FAIL with this exact message:

```
kei-cortex-setup.sh primitive not found at:
  <SETUP_PATH>

The KeiSeiKit install appears partial. Re-run:

  ./install.sh --profile=cortex

from your KeiSeiKit repo root. If install.sh reports success but the
file is still missing, file an issue with `./install.sh --doctor` output.
```

STOP the wizard. Do NOT proceed.

## 0d — Target install directory

Compute the install target root (where the wizard will write token + UI
dist + supervisor artefacts):

```bash
TARGET_DIR="$HOME/.keisei"
mkdir -p "$TARGET_DIR"
```

The directory is created here (idempotent `mkdir -p`) so every later
phase can assume it exists. No permission changes yet — Phase 3 sets
chmod 600 on the token file specifically.

## 0e — Preflight summary

Print exactly 4 lines (no more, no less) so the user can visually
confirm the wizard is wired to the right paths:

```
OS:              <OS>
Daemon binary:   <BIN_PATH>
Setup script:    <SETUP_PATH>
Target dir:      <TARGET_DIR>  (will hold token, ui-dist, supervisor refs)
```

## 0f — Early advisory checks (non-blocking)

Run these probes for informational surface only — do NOT block the
wizard on any of them. Phase 4 will re-check and handle failures.

```bash
command -v python3 >/dev/null 2>&1 && echo "python3: present ($(python3 --version 2>&1))" \
                                   || echo "python3: MISSING (needed for Phase 4 pip-install)"
command -v pip3    >/dev/null 2>&1 && echo "pip3: present"    || echo "pip3: MISSING (needed for Phase 4)"
command -v ffmpeg  >/dev/null 2>&1 && echo "ffmpeg: present"  || echo "ffmpeg: MISSING (needed by faster-whisper at runtime)"
command -v node    >/dev/null 2>&1 && echo "node: present ($(node -v 2>&1))" \
                                    || echo "node: MISSING (needed for Phase 4 ui-build)"
command -v pnpm    >/dev/null 2>&1 && echo "pnpm: present"    || echo "pnpm: MISSING (needed for Phase 4 ui-build)"
```

Print this block under the summary with a `Host tools:` header. The
user can resolve any MISSING items before Phase 4, but the wizard does
not gate on them here — the primitive's subcommands give better error
messages when they actually run.

## 0g — Proceed

No `AskUserQuestion`. Automatically continue to Phase 1.

Print a one-line transition:

```
Preflight OK. Proceeding to key check…
```

## Verify-criterion

- `OS ∈ {macos, linux}`.
- `BIN_PATH` exists AND is executable.
- `SETUP_PATH` exists AND is executable.
- `~/.keisei/` exists (created idempotently in 0d).
- Exactly 4-line summary + host tools block emitted.
- ZERO `AskUserQuestion` in this phase.
- On ANY failure in 0a-0c, the wizard halts with a single actionable
  message pointing at `./install.sh --profile=cortex`. No retry loop.
