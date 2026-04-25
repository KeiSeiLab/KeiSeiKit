# Phase 3 — Token generation (RULE 0.8)

Generate a 32-hex-byte (64-character) bearer token and write it to
`~/.keisei/cortex.token` with mode 600. Zero user interaction — the
primitive's `token-gen` subcommand does the work; the wizard only
redirects its stdout to disk and verifies length.

Per RULE 0.8 the token VALUE never appears in chat output. Only the
path, byte length, and a `<hidden>` marker are surfaced.

## 3a — Idempotency check

Re-running the wizard MUST NOT clobber an existing token unless the
user explicitly asks. Check first:

```bash
TOKEN_PATH="$HOME/.keisei/cortex.token"
if [ -f "$TOKEN_PATH" ]; then
  EXISTING_LEN=$(wc -c < "$TOKEN_PATH" 2>/dev/null | tr -d ' ')
else
  EXISTING_LEN=0
fi
```

- If `EXISTING_LEN == 64` (correct length) → emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "A valid cortex token already exists at ~/.keisei/cortex.token. Keep it or regenerate?",
      "header": "Existing token",
      "multiSelect": false,
      "options": [
        {"label": "Keep existing",  "description": "Skip token-gen — preserves any saved URLs / in-flight sessions that embed this token"},
        {"label": "Regenerate",     "description": "Rotate the token now — any previously-issued setup URL will stop working"}
      ]
    }
  ]
}
```

If the user picks `Keep existing` → set `TOKEN_ACTION = reused` and skip
to 3d. If `Regenerate` → proceed to 3b.

- If `EXISTING_LEN > 0` but `!= 64` → print:

```
Found cortex.token of unexpected length <EXISTING_LEN> bytes (expected 64).
File will be overwritten with a freshly generated token.
```

Proceed to 3b without asking — the existing file is malformed.

- If `EXISTING_LEN == 0` → proceed to 3b (first-time setup).

## 3b — Invoke `token-gen`

Run the primitive subcommand and capture stdout:

```bash
TOK="$($SETUP_PATH token-gen)"
```

Validate the captured token WITHOUT echoing it:

```bash
TOK_LEN=${#TOK}
if [ "$TOK_LEN" -ne 64 ]; then
  echo "ERROR: token-gen returned $TOK_LEN bytes (expected 64)." >&2
  unset TOK
  exit 1
fi

# Must be pure hex
case "$TOK" in
  *[!0-9a-fA-F]*)
    echo "ERROR: token-gen returned non-hex characters." >&2
    unset TOK
    exit 1
    ;;
esac
```

On failure (wrong length, non-hex, non-zero exit from `token-gen`),
HARD FAIL the wizard with the stderr from the primitive. Do NOT retry
silently — a broken `token-gen` means the install itself is broken.

## 3c — Write to disk with chmod 600

```bash
mkdir -p "$HOME/.keisei"
printf '%s' "$TOK" > "$TOKEN_PATH"
chmod 600 "$TOKEN_PATH"
unset TOK
```

Note: `printf '%s'` (not `echo`) so no trailing newline is written —
the daemon reads the file as a single bearer value, a stray `\n` would
mismatch on comparison.

Set `TOKEN_ACTION = generated`.

## 3d — Verify

Re-read the file length without exposing the value:

```bash
FINAL_LEN=$(wc -c < "$TOKEN_PATH" 2>/dev/null | tr -d ' ')
```

Must equal 64. If it doesn't, HARD FAIL the wizard with:

```
Token write verification failed: ~/.keisei/cortex.token is <FINAL_LEN>
bytes, expected 64. Check disk space and filesystem flags (noexec/ro
mounts). Constructive paths:
  (A) free disk space and retry Phase 3 by re-running /cortex-setup
  (B) check the directory is writable: `ls -la ~/.keisei/`
  (C) remove an existing file with the wrong perms: `rm ~/.keisei/cortex.token`
      then re-run /cortex-setup
```

Verify chmod 600:

```bash
PERMS=$(stat -f '%Lp' "$TOKEN_PATH" 2>/dev/null || stat -c '%a' "$TOKEN_PATH" 2>/dev/null)
if [ "$PERMS" != "600" ]; then
  chmod 600 "$TOKEN_PATH"
fi
```

(`stat -f` = macOS BSD syntax; `stat -c` = GNU/Linux — try both.)

## 3e — Emit summary line

Exactly ONE line to chat, value hidden:

```
Token written: ~/.keisei/cortex.token (64 hex bytes, chmod 600, <hidden>)
```

If `TOKEN_ACTION == reused`, emit instead:

```
Token kept:    ~/.keisei/cortex.token (existing 64 hex bytes, chmod 600, <hidden>)
```

## 3f — Set pipeline variable

Store `TOKEN_PATH = "$HOME/.keisei/cortex.token"` for use by Phase 5
(supervisor-install needs the path, NOT the value) and Phase 6 (which
reads the file at that moment to compose the setup URL).

## Verify-criterion

- `~/.keisei/cortex.token` exists.
- File size is exactly 64 bytes.
- File mode is `600`.
- File contents match `^[0-9a-fA-F]{64}$` (confirmed at generation time,
  not re-grepped here to avoid echoing the value).
- ZERO `AskUserQuestion` unless a 64-byte token already existed (then
  exactly ONE).
- No token value appears in chat output (RULE 0.8).
- `TOKEN_ACTION ∈ {generated, reused}`.
