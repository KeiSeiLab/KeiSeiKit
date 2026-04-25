# Phase 1 — API keys check (RULE 0.8)

Verify the three API keys that the daemon needs live in the umbrella env
file. The daemon endpoints will return 500 until each is present:

- `ANTHROPIC_API_KEY`  → `/chat`
- `ELEVENLABS_API_KEY` → `/tts`
- `FAL_KEY`            → `/portrait/stylize`

This phase NEVER prints a key value. Only `<present>` / `<missing>` /
`<pasted>` markers appear in chat. Per RULE 0.8 the umbrella file is
the single source of truth; pasted keys are written through to it here.

## 1a — Umbrella env file path

```bash
ENV_FILE="$HOME/.claude/secrets/.env"
```

If the file does not exist, create it with chmod 600:

```bash
if [ ! -f "$ENV_FILE" ]; then
  mkdir -p "$HOME/.claude/secrets"
  touch "$ENV_FILE"
  chmod 600 "$ENV_FILE"
fi
```

## 1b — Probe each key

Use a safe grep that matches `KEY=nonempty` but never echoes the value:

```bash
probe_key() {
  local name="$1"
  if grep -qE "^${name}=.+\$" "$ENV_FILE" 2>/dev/null; then
    echo "present"
  else
    echo "missing"
  fi
}

KEY_STATE_ANTHROPIC=$(probe_key ANTHROPIC_API_KEY)
KEY_STATE_ELEVENLABS=$(probe_key ELEVENLABS_API_KEY)
KEY_STATE_FAL=$(probe_key FAL_KEY)
```

Render a 3-line summary to chat (values NEVER shown — only the state):

```
ANTHROPIC_API_KEY:   <present | missing>
ELEVENLABS_API_KEY:  <present | missing>
FAL_KEY:             <present | missing>
```

## 1c — If all three present, skip to Phase 2

If `{KEY_STATE_ANTHROPIC, KEY_STATE_ELEVENLABS, KEY_STATE_FAL}` are all
`present`, print:

```
All 3 API keys already in ~/.claude/secrets/.env. Proceeding to config…
```

Zero `AskUserQuestion` emitted. Continue to Phase 2.

## 1d — For each missing key, ONE AskUserQuestion

Loop over the 3 keys; for each one whose state is `missing`, emit ONE
`AskUserQuestion` (not batched — one per missing key so the user sees
each decision independently):

```json
{
  "questions": [
    {
      "question": "ANTHROPIC_API_KEY is missing from ~/.claude/secrets/.env. How should we handle it?",
      "header": "ANTHROPIC_API_KEY",
      "multiSelect": false,
      "options": [
        {"label": "Skip (feature will 500 until added)",   "description": "Proceed; /chat endpoint returns 500 until you add the key manually"},
        {"label": "Paste now",                             "description": "Enter the key in the follow-up free-text prompt; written to env with chmod 600"}
      ]
    }
  ]
}
```

Substitute the key name for each iteration (`ELEVENLABS_API_KEY`,
`FAL_KEY`). Endpoint note per key:

- `ANTHROPIC_API_KEY`   → `/chat` (Anthropic SSE)
- `ELEVENLABS_API_KEY`  → `/tts`
- `FAL_KEY`             → `/portrait/stylize`

## 1e — Handle "Skip"

Set `KEY_STATE_<NAME> = missing` (unchanged). Continue to next missing
key OR to Phase 2 if this was the last one. Log:

```
<KEY_NAME>: skipped — add to ~/.claude/secrets/.env manually later.
```

## 1f — Handle "Paste now"

Emit a follow-up `AskUserQuestion` with `freeText: true`:

```json
{
  "questions": [
    {
      "question": "Paste the <KEY_NAME> value. It will be written to ~/.claude/secrets/.env (chmod 600). It will NOT be echoed back to chat.",
      "header": "<KEY_NAME>",
      "freeText": true
    }
  ]
}
```

Capture the reply as `PASTED_VALUE`. Validate:

- non-empty after trim
- contains no newline character (single-line secret)
- length between 10 and 512 chars (sanity bounds — reject obvious typos
  like accidentally pasting a command)

On validation failure, print exactly:

```
Invalid value for <KEY_NAME> (empty / multi-line / wrong length).
Retry or skip?
```

…and re-emit the paste-or-skip `AskUserQuestion`. Max 3 retries per
key; after 3 invalid pastes, force-skip and log
`<KEY_NAME>: force-skipped after 3 invalid pastes`.

On valid paste, append (or replace) the line in the env file WITHOUT
echoing the value:

```bash
# strip existing assignment if any (idempotent)
if grep -qE "^${KEY_NAME}=" "$ENV_FILE" 2>/dev/null; then
  tmp=$(mktemp)
  grep -vE "^${KEY_NAME}=" "$ENV_FILE" > "$tmp"
  mv "$tmp" "$ENV_FILE"
fi
printf '%s=%s\n' "${KEY_NAME}" "${PASTED_VALUE}" >> "$ENV_FILE"
chmod 600 "$ENV_FILE"
unset PASTED_VALUE
```

Set `KEY_STATE_<NAME> = pasted`. Log:

```
<KEY_NAME>: written to ~/.claude/secrets/.env (chmod 600). Value hidden.
```

## 1g — Final summary

After all three keys have been probed (and optionally pasted), print:

```
Key state:
  ANTHROPIC_API_KEY:   <present | missing | pasted>
  ELEVENLABS_API_KEY:  <present | missing | pasted>
  FAL_KEY:             <present | missing | pasted>
```

Store the 3-field `KEY_STATE` object for the final report in Phase 6.

## Verify-criterion

- `~/.claude/secrets/.env` exists with mode 600.
- For each key: state is one of `present` / `missing` / `pasted`.
- ZERO key values appear anywhere in chat output (RULE 0.8).
- `AskUserQuestion` count: 0 (all present) up to 6 (3 paste-flow cycles
  of confirm + freeText).
- No retry loop exceeds 3 tries per key.
