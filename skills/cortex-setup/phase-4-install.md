# Phase 4 — Install (pip + whisper-pull + ui-build)

Run the three heavy install subcommands in order, with per-stage progress
echo. Each subcommand is independently idempotent per the primitive
contract, so a partial re-run is safe. The user sees three progress
stages and one decision click (proceed / skip).

## 4a — Size / time estimate

Print an up-front estimate so the user knows what they're committing to:

```
Phase 4 will run three install steps (all idempotent — safe to re-run):

  1. pip install faster-whisper==1.2.1  (~150 MB download, ~30 s)
  2. whisper-pull <MODEL>               (base.en ~140 MB / small.en ~470 MB
                                         / medium.en ~1.5 GB, ~1-3 min)
  3. ui-build  (pnpm install + pnpm build, ~2 min, ~250 MB node_modules)

Total disk: ~500 MB-2 GB depending on whisper model.
Total time: ~3-7 min on a fast connection.
```

Substitute `<MODEL>` with `$MODEL` from Phase 2.

## 4b — Proceed click

ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Run the 3 install steps now?",
      "header": "Install",
      "multiSelect": false,
      "options": [
        {"label": "Proceed",          "description": "Run pip / whisper-pull / ui-build in order. Re-runs are idempotent."},
        {"label": "Skip (already installed)", "description": "I've already run these — jump to supervisor setup"},
        {"label": "Abort",            "description": "Stop the wizard; re-run /cortex-setup later"}
      ]
    }
  ]
}
```

Handle:

- `Proceed` → continue to 4c.
- `Skip (already installed)` → set `INSTALL_ACTION = "skipped (user)"`,
  still probe for `UI_DIST` and `WHISPER_CACHE` via the primitive's
  `status` subcommand (4g), then continue to Phase 5.
- `Abort` → print `Aborted — re-run /cortex-setup later.` and exit.

## 4c — Step 1: pip-install

```bash
REQ="$HOME/.claude/agents/_primitives/_rust/kei-cortex/scripts/requirements.txt"
echo "[1/3] pip install -r $REQ …"
"$SETUP_PATH" pip-install "$REQ"
PIP_EXIT=$?
```

On non-zero exit, capture stderr and emit the retry click (4h). Common
failure reasons to surface in the preview:

- `python3` not on PATH — user missed a Phase 0 advisory.
- `pip` not on PATH — suggest `python3 -m ensurepip --user`.
- network error — retry works on next attempt once offline resolves.
- permission error — suggest `--user` flag or a venv.

## 4d — Step 2: whisper-pull

```bash
echo "[2/3] whisper-pull $MODEL  (this downloads weights from Hugging Face on first run) …"
WHISPER_OUT="$("$SETUP_PATH" whisper-pull "$MODEL")"
WHISPER_EXIT=$?
```

On success, `WHISPER_OUT` has the exact form
`WHISPER_READY:<model>:<cached-path>`. Parse with a sanity check:

```bash
case "$WHISPER_OUT" in
  WHISPER_READY:"$MODEL":*)
    WHISPER_CACHE="${WHISPER_OUT#WHISPER_READY:$MODEL:}"
    ;;
  *)
    echo "whisper-pull returned unexpected output: $WHISPER_OUT" >&2
    WHISPER_EXIT=1
    ;;
esac
```

On non-zero exit → retry click (4h). Common failure reasons:

- HF hub rate limit — wait 60 s and retry.
- disk full — free space and retry.
- SSL / proxy issue — suggest `HF_ENDPOINT` override or VPN off.

## 4e — Step 3: ui-build

```bash
UI_SRC="$HOME/.claude/agents/_ts_packages/packages/cortex-ui"
echo "[3/3] ui-build  (pnpm install + vite build, ~2 min) …"
UI_OUT="$("$SETUP_PATH" ui-build "$UI_SRC")"
UI_EXIT=$?
```

On success, `UI_OUT` has the exact form `UI_DIST:<abs-path>`. Parse:

```bash
case "$UI_OUT" in
  UI_DIST:/*)
    UI_DIST="${UI_OUT#UI_DIST:}"
    ;;
  *)
    echo "ui-build returned unexpected output: $UI_OUT" >&2
    UI_EXIT=1
    ;;
esac
```

On non-zero exit → retry click (4h). Common failure reasons:

- `node` / `pnpm` not on PATH — suggest `brew install pnpm` or
  `npm install -g pnpm`.
- node version too old — cortex-ui targets node>=18.
- TypeScript build error — user may have edited cortex-ui locally;
  suggest reverting to the shipped source.

## 4f — All three succeeded

Print a compact 3-line success summary:

```
[1/3] pip-install:   ok  (faster-whisper==1.2.1)
[2/3] whisper-pull:  ok  (<MODEL>, cache: <WHISPER_CACHE>)
[3/3] ui-build:      ok  (dist: <UI_DIST>)
```

Set `INSTALL_ACTION = "installed"`. Proceed to Phase 5.

## 4g — "Skip (already installed)" status probe

If the user clicked `Skip (already installed)` in 4b, fetch the status
from the primitive to populate `UI_DIST` and `WHISPER_CACHE` for
downstream phases:

```bash
STATUS_JSON="$("$SETUP_PATH" status)"
```

Parse `ui_dist` and infer `whisper_cache` from the model name. If the
JSON is missing either field → set them to empty strings and warn:

```
Skip chosen, but the primitive's `status` subcommand could not locate
<field>. Phase 5 will still proceed; the supervisor unit will fail at
runtime if this path is wrong. Constructive paths:
  (A) re-run /cortex-setup, pick Proceed this time
  (B) manually run `<SETUP_PATH> ui-build <UI_SRC>` then retry
```

## 4h — Retry click (on any step failure)

Emit ONE `AskUserQuestion` with the tail of stderr (last 20 lines) as
context:

```json
{
  "questions": [
    {
      "question": "Step <N>/<3> failed. What next?",
      "header": "Install step failed",
      "multiSelect": false,
      "options": [
        {"label": "Retry",  "description": "Re-run the failed subcommand — fixes transient network errors"},
        {"label": "Skip",   "description": "Continue without this step — later phases may fail at runtime"},
        {"label": "Abort",  "description": "Stop the wizard — re-run /cortex-setup later"}
      ]
    }
  ]
}
```

Max 3 retries per step. After 3 failures, force-skip that step and
continue; `INSTALL_ACTION` becomes `"partial (step N skipped)"`.

## Verify-criterion

- If `Proceed` picked: all three subcommands ran (or were retried/skipped
  with explicit user consent).
- `UI_DIST` points to an existing directory (after step 3 OR status probe).
- `WHISPER_CACHE` is a non-empty string (after step 2 OR status probe).
- Exactly 1 `AskUserQuestion` in the happy path (4b). Up to +3 per
  failing step (4h) = 10 max.
- `INSTALL_ACTION ∈ {installed, skipped (user), partial (step N skipped)}`.
