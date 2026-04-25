# Phase 6 — Done (setup URL, clipboard, browser open)

Compose the setup URL from `UI_HOST`, `CORTEX_PORT`, and the token on
disk. Try to copy it to the clipboard. Offer one final click: open it
in the default browser, copy-only, or show the token (already in URL).

Per RULE 0.8 the token in the URL itself is not considered "printed to
chat in isolation" — the URL is the whole mechanism the cortex-ui
consumes. But the chat summary never shows the token as a standalone
string; the user must read it out of the URL themselves.

## 6a — Compose the setup URL

```bash
# Read token at this moment (NOT cached from Phase 3 — in case of rerun)
TOK="$(cat "$TOKEN_PATH")"

# URL-encode daemon URL (just %3A and %2F — RFC 3986 reserved)
DAEMON="http://127.0.0.1:${CORTEX_PORT}"
DAEMON_ENC="http%3A%2F%2F127.0.0.1%3A${CORTEX_PORT}"

SETUP_URL="http://${UI_HOST}/?daemon=${DAEMON_ENC}&token=${TOK}"
unset TOK
```

Note: we intentionally compute `DAEMON_ENC` inline rather than calling
`jq -rR @uri` or `python3 -c 'import urllib.parse'` so the skill has no
extra shell dependency. The only chars to escape are `:` → `%3A` and
`/` → `%2F`, handled by the literal string.

## 6b — Copy to clipboard (best-effort)

Try in order:

```bash
CLIP_OK=0
if   command -v pbcopy >/dev/null 2>&1 && [ "$OS" = "macos" ]; then
  printf '%s' "$SETUP_URL" | pbcopy && CLIP_OK=1
elif command -v wl-copy >/dev/null 2>&1; then
  printf '%s' "$SETUP_URL" | wl-copy && CLIP_OK=1
elif command -v xclip   >/dev/null 2>&1; then
  printf '%s' "$SETUP_URL" | xclip -selection clipboard && CLIP_OK=1
fi
```

`CLIP_OK=1` means the URL is on the clipboard; `CLIP_OK=0` means the
user will need to copy from the chat output manually.

## 6c — Final AskUserQuestion

ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Setup URL ready. What next?",
      "header": "Open UI",
      "multiSelect": false,
      "options": [
        {"label": "Open in default browser",    "description": "Invoke `open <url>` (macOS) / `xdg-open <url>` (Linux) to launch cortex-ui now"},
        {"label": "Copy URL only (no open)",   "description": "URL is on your clipboard (or printed below) — paste it into your browser manually"},
        {"label": "Show the token separately",  "description": "Print the bearer token on its own line (it's already in the URL above) — for debugging"}
      ]
    }
  ]
}
```

Handle:

- `Open in default browser`:
  ```bash
  if [ "$OS" = "macos" ]; then
    open "$SETUP_URL"
  elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$SETUP_URL"
  else
    echo "No browser launcher found on this host. URL is on clipboard / printed above."
  fi
  ```
  Set `OPEN_ACTION = "browser launched"`.

- `Copy URL only (no open)`:
  If `CLIP_OK=1`, print `URL on clipboard — paste into your browser.`
  If `CLIP_OK=0`, print `URL below — copy manually:` followed by the URL
  in a fenced code block.
  Set `OPEN_ACTION = "copy only"`.

- `Show the token separately`:
  Print the token in a fenced code block, then the URL in a separate
  fenced code block. This is the ONLY place the token appears outside
  a URL; the user explicitly asked for it. Set
  `OPEN_ACTION = "token shown"`.

  ```
  Token (64 hex chars):
  <hex-value>

  Setup URL (token embedded):
  <SETUP_URL>
  ```

## 6d — Render setup URL in chat

Regardless of branch, print the URL in a fenced code block so the
record is in the chat transcript:

```
Setup URL:
http://<UI_HOST>/?daemon=http%3A%2F%2F127.0.0.1%3A<CORTEX_PORT>&token=<64-hex>
```

If `CLIP_OK=1`, append `(also on clipboard)` to the line above.

## 6e — Final summary block

Print the checklist mirroring the SKILL.md contract:

```
=== CORTEX-SETUP REPORT ===
✓ Token ............ ~/.keisei/cortex.token  (chmod 600, <hidden>)
✓ faster-whisper ... installed, model: <MODEL>
✓ cortex-ui dist ... <UI_DIST>
✓ Whisper cache .... <WHISPER_CACHE>
✓ Port ............. <CORTEX_PORT>
✓ UI host .......... <UI_HOST>
✓ Supervisor ....... <launchd | systemd | manual>
✓ Setup URL ........ <copied to clipboard | printed below>
✓ Keys (env refs) .. ANTHROPIC=<state>  ELEVENLABS=<state>  FAL=<state>

Next steps:
  • Write chat messages, upload portraits, test voice.
  • Tail log:  tail -f ~/Library/Logs/keisei-cortex.log           (macOS)
               journalctl --user -u keisei-cortex -f              (linux)
  • Rerun this wizard:  /cortex-setup  (idempotent — safe to re-run).
  • Rotate token:       /cortex-setup → Phase 3 → "Regenerate".
```

Substitute real values; omit `(copied to clipboard)` if `CLIP_OK=0`.

## 6f — Final "notes" block (only if degraded state)

If ANY of these are true, append a notes block before exit:

- Any API key state is `missing` (from Phase 1).
- `SUPERVISOR == "manual"`.
- `INSTALL_ACTION != "installed"` (e.g. `"skipped (user)"` or partial).

```
Notes:
  • <list missing keys>           → feature will 500 until added to ~/.claude/secrets/.env
  • Supervisor is manual          → start with:
                                      <BIN_PATH> --port <CORTEX_PORT> --token-file ~/.keisei/cortex.token
  • Install was skipped / partial → re-run /cortex-setup → Phase 4 → "Proceed"
```

Only include the bullets that actually apply.

## 6g — Exit

No further phases. The wizard is complete. The user's next action is
whatever they clicked in 6c (browser / copy / token shown).

## Verify-criterion

- `SETUP_URL` contains no `<placeholder>` — all three of `UI_HOST`,
  `CORTEX_PORT`, and token have been substituted.
- Token in URL matches `^[0-9a-fA-F]{64}$`.
- Exactly 1 `AskUserQuestion` (6c).
- Clipboard attempt logged (success or fallback).
- Final summary block contains 9 `✓` rows with real values.
- Degraded-state notes block is present iff at least one degradation is
  actually present; otherwise omitted.
