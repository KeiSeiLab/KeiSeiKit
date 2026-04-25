# Phase 2 — Config (port, UI host, whisper model)

Gather the three runtime config choices as a SINGLE `AskUserQuestion`
batch with three questions — one click each, all defaulting to safe
values. Custom port / UI host adds a follow-up free-text prompt.

## 2a — Tailscale probe (advisory for UI host option)

Detect whether tailscale is present and has an assigned IP. If so, the
Phase 2b "tailscale IP" option becomes an actual value; if not, it is
replaced with a disabled-looking stub:

```bash
TS_IP=""
if command -v tailscale >/dev/null 2>&1; then
  TS_IP="$(tailscale ip -4 2>/dev/null | head -n 1)"
fi
```

Store `TS_IP` (may be empty). Do not print it yet — it is only used
inside the AskUserQuestion option labels below.

## 2b — Single AskUserQuestion batch (3 questions)

Emit ONE `AskUserQuestion` with THREE questions, click-only:

```json
{
  "questions": [
    {
      "question": "Which port should the kei-cortex daemon listen on?",
      "header": "Port",
      "multiSelect": false,
      "options": [
        {"label": "9797 (default)",   "description": "Pinned in INSTALL.md and cortex-ui defaults; pick this unless 9797 is already taken on this host"},
        {"label": "Custom",           "description": "Enter an exact port number in the next prompt (1024-65535)"}
      ]
    },
    {
      "question": "Which host should cortex-ui bind to?",
      "header": "UI host",
      "multiSelect": false,
      "options": [
        {"label": "127.0.0.1:18080 (default)", "description": "Loopback only — the UI is reachable only from this machine"},
        {"label": "<TS_IP>:18080 (tailscale)", "description": "Bind to the tailscale interface — reachable from all your tailnet devices"},
        {"label": "Custom",                    "description": "Enter host:port in the next prompt (e.g. 0.0.0.0:18080 for LAN)"}
      ]
    },
    {
      "question": "Which Whisper model for speech-to-text?",
      "header": "Whisper model",
      "multiSelect": false,
      "options": [
        {"label": "base.en (~140 MB)",   "description": "Fast, English-only, recommended default — fits on any modern laptop"},
        {"label": "small.en (~470 MB)",  "description": "Better accuracy, English-only, small latency increase"},
        {"label": "medium.en (~1.5 GB)", "description": "Best English quality, meaningful latency/RAM hit — pick if you dictate long text"}
      ]
    }
  ]
}
```

If `TS_IP` is empty string, REPLACE the second option of the second
question with a disabled stub showing why it is unavailable:

```json
{"label": "Tailscale unavailable", "description": "`tailscale` CLI not found or no IPv4 — install tailscale or pick 127.0.0.1 / Custom"}
```

## 2c — Resolve `CORTEX_PORT`

Based on the Port answer:

- `9797 (default)` → set `CORTEX_PORT = 9797`.
- `Custom` → follow-up `AskUserQuestion` with `freeText`:

```json
{
  "questions": [
    {
      "question": "Enter the port number (integer 1024-65535). Example: 19797",
      "header": "Custom port",
      "freeText": true
    }
  ]
}
```

Validate: integer, 1024 ≤ x ≤ 65535. On failure print:

```
Invalid port '<reply>'. Expected integer 1024-65535 (ports below 1024
require root on linux; >65535 does not exist).
```

…and re-emit the freeText prompt. Max 3 retries; after 3 invalid inputs,
fall back to `9797` and log `CORTEX_PORT defaulted to 9797 after 3
invalid inputs`.

## 2d — Resolve `UI_HOST`

Based on the UI host answer:

- `127.0.0.1:18080 (default)` → set `UI_HOST = "127.0.0.1:18080"`.
- `<TS_IP>:18080 (tailscale)`   → set `UI_HOST = "<TS_IP>:18080"`.
- `Tailscale unavailable`       → re-emit the Phase 2b batch (user must
  pick default or custom; the stub is not selectable). In practice the
  AskUserQuestion UI will refuse the stub, so this branch is a safety
  net for CLI drivers.
- `Custom` → follow-up `AskUserQuestion` with `freeText`:

```json
{
  "questions": [
    {
      "question": "Enter the UI bind address as host:port. Example: 0.0.0.0:18080 for LAN reachability.",
      "header": "Custom UI host",
      "freeText": true
    }
  ]
}
```

Validate regex `^[A-Za-z0-9\.\-]+:[0-9]+$`. On failure print:

```
Invalid host '<reply>'. Expected host:port (examples: 127.0.0.1:18080,
0.0.0.0:18080, mymac.local:18080).
```

…and re-emit. Max 3 retries; fall back to `127.0.0.1:18080` after 3
failures and log `UI_HOST defaulted to 127.0.0.1:18080 after 3 invalid
inputs`.

## 2e — Resolve `MODEL`

Based on the Whisper model answer, set verbatim:

- `base.en (~140 MB)`   → `MODEL = "base.en"`
- `small.en (~470 MB)`  → `MODEL = "small.en"`
- `medium.en (~1.5 GB)` → `MODEL = "medium.en"`

No follow-up prompt — no custom model allowed here (Phase 4's
`whisper-pull` subcommand only advertises these three variants in the
primitive contract).

## 2f — Echo config summary

Print:

```
Port:            <CORTEX_PORT>
UI host:         <UI_HOST>
Whisper model:   <MODEL>

(Config collected; no files written yet — Phase 3 writes the token,
Phase 4 installs deps, Phase 5 registers the supervisor.)
```

## Verify-criterion

- `CORTEX_PORT ∈ [1024, 65535]` (integer).
- `UI_HOST` matches `^[A-Za-z0-9\.\-]+:[0-9]+$`.
- `MODEL ∈ {base.en, small.en, medium.en}`.
- Exactly 1 AskUserQuestion batch (3 questions) in the happy path.
- Up to 2 additional free-text follow-ups if the user picks Custom for
  port or UI host.
- No files are written in this phase.
