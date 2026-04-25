# Phase 1b — Multi-provider API key collection (RULE 0.8)

INSERT-AFTER `phase-1-keys.md`. The base phase 1 covers the three legacy
keys (`ANTHROPIC_API_KEY`, `ELEVENLABS_API_KEY`, `FAL_KEY`). Phase 1b
extends provider coverage so the cortex daemon can route chat traffic
to alternate providers (OpenAI fallback, Moonshot Kimi cheaper tier,
local-only future mode) without a second wizard pass.

This phase NEVER prints a key value. Per RULE 0.8 the umbrella file
`~/.claude/secrets/.env` is the single source of truth; pasted keys are
written through with `chmod 600`. Verification pings each provider's
`/models` endpoint and stores the result; failures route to a single
`Retry / Skip / Continue without` click instead of bailing the wizard.

## 1b.1 — Pre-state from Phase 1

Phase 1 produced `KEY_STATE = {anthropic, elevenlabs, fal}`. Phase 1b
EXTENDS this to a provider-keyed map. Initialize:

```bash
ENV_FILE="$HOME/.claude/secrets/.env"
declare -A PROVIDER_KEY_NAME=(
  [anthropic]=ANTHROPIC_API_KEY
  [openai]=OPENAI_API_KEY
  [kimi]=MOONSHOT_API_KEY
  [local]=LOCAL_ONLY
)
declare -A PROVIDER_VERIFY_URL=(
  [anthropic]="https://api.anthropic.com/v1/models"
  [openai]="https://api.openai.com/v1/models"
  [kimi]="https://api.moonshot.cn/v1/models"
  [local]=""
)
declare -A VERIFIED=()
declare -A KEY_STATE_PROV=(
  [anthropic]="${KEY_STATE_ANTHROPIC:-missing}"
)
```

If `KEY_STATE_ANTHROPIC=present` from Phase 1 we DO NOT re-prompt for
that provider; Phase 1b only collects NEW providers (openai, kimi, local).

## 1b.2 — Provider selection (1 multi-select AskUserQuestion)

Emit ONE batch with multi-select:

```json
{
  "questions": [
    {
      "question": "Which providers should the cortex daemon route to? Select all that apply. Anthropic is the default chat provider; the rest are optional fallbacks.",
      "header": "Providers",
      "multiSelect": true,
      "options": [
        {"label": "Anthropic (Claude)",    "description": "Default chat — sk-ant-* key. /chat endpoint."},
        {"label": "OpenAI",                 "description": "Fallback — gpt-4o-mini route. sk-* key."},
        {"label": "Moonshot Kimi",         "description": "Cheaper alternate — moonshot-v1-* models."},
        {"label": "Local-only (no cloud)", "description": "Future offline mode — no API key required."}
      ]
    }
  ]
}
```

Capture selection as `SELECTED_PROVIDERS` (e.g. `["anthropic","openai"]`).
If empty, log `No providers selected — Phase 1b skipped.` and continue.

## 1b.3 — Idempotency probe per selected provider

For each `prov` in `SELECTED_PROVIDERS` whose `KEY_STATE_PROV[$prov]` is
unset, probe the env file the same way Phase 1 does:

```bash
probe_provider() {
  local name="$1"
  if grep -qE "^${name}=.+\$" "$ENV_FILE" 2>/dev/null; then
    echo "present"
  else
    echo "missing"
  fi
}

for prov in "${SELECTED_PROVIDERS[@]}"; do
  [ "$prov" = "local" ] && KEY_STATE_PROV[$prov]=present && continue
  KEY_STATE_PROV[$prov]=$(probe_provider "${PROVIDER_KEY_NAME[$prov]}")
done
```

For each `prov` with state `present`, emit ONE click batch:

```json
{
  "questions": [
    {
      "question": "<KEY_NAME> is already in ~/.claude/secrets/.env. Keep or replace?",
      "header": "<provider>",
      "multiSelect": false,
      "options": [
        {"label": "Keep existing",   "description": "No change. Verification will re-ping with the existing key."},
        {"label": "Replace",          "description": "Paste a new value in the next prompt. Old value is overwritten."}
      ]
    }
  ]
}
```

On `Replace`, set `KEY_STATE_PROV[$prov]=missing` so the paste flow runs.

## 1b.4 — Paste flow (one freeText AskUserQuestion per missing provider)

For each `prov` with `KEY_STATE_PROV[$prov]=missing` (and not `local`):

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

Validate the reply (`PASTED_VALUE`):
- non-empty after trim
- no newline (single-line secret)
- length 10-512 chars

Failure → re-emit the paste prompt up to 3 times, then force-skip with
`<KEY_NAME>: force-skipped after 3 invalid pastes` and proceed.

Write through (idempotent — strip-then-append):

```bash
KEY_NAME="${PROVIDER_KEY_NAME[$prov]}"
if grep -qE "^${KEY_NAME}=" "$ENV_FILE" 2>/dev/null; then
  tmp=$(mktemp)
  grep -vE "^${KEY_NAME}=" "$ENV_FILE" > "$tmp"
  mv "$tmp" "$ENV_FILE"
fi
printf '%s=%s\n' "${KEY_NAME}" "${PASTED_VALUE}" >> "$ENV_FILE"
chmod 600 "$ENV_FILE"
unset PASTED_VALUE
KEY_STATE_PROV[$prov]=pasted
```

Log `<KEY_NAME>: written to ~/.claude/secrets/.env (chmod 600). Value hidden.`

## 1b.5 — Verification ping

For each `prov` in `SELECTED_PROVIDERS` with state `present` or `pasted`
(skip `local`), ping the provider's `/models` endpoint with a 5s timeout:

```bash
verify_key() {
  local prov="$1"
  local name="${PROVIDER_KEY_NAME[$prov]}"
  local url="${PROVIDER_VERIFY_URL[$prov]}"
  # shellcheck disable=SC1090
  set -a && . "$ENV_FILE" && set +a
  local val="${!name}"
  [ -z "$val" ] && echo "fail" && return
  local hdr
  case "$prov" in
    anthropic) hdr=("-H" "x-api-key: $val" "-H" "anthropic-version: 2023-06-01") ;;
    openai)    hdr=("-H" "Authorization: Bearer $val") ;;
    kimi)      hdr=("-H" "Authorization: Bearer $val") ;;
  esac
  local code
  code=$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 "${hdr[@]}" "$url" 2>/dev/null || echo "000")
  unset val
  if [ "$code" = "200" ]; then echo "ok"; else echo "fail (http $code)"; fi
}

for prov in "${SELECTED_PROVIDERS[@]}"; do
  [ "$prov" = "local" ] && VERIFIED[$prov]="ok" && continue
  [ "${KEY_STATE_PROV[$prov]}" = "missing" ] && VERIFIED[$prov]="skipped" && continue
  VERIFIED[$prov]=$(verify_key "$prov")
done
```

ZERO key values appear in the verify command output (we use `--max-time`
+ header-only mode and unset the local variable immediately).

## 1b.6 — On verify failure: one click, three options

For each `prov` whose `VERIFIED[$prov]` starts with `fail`, emit:

```json
{
  "questions": [
    {
      "question": "Verification ping for <provider> failed (<reason>). How to proceed?",
      "header": "<provider>",
      "multiSelect": false,
      "options": [
        {"label": "Retry",                   "description": "Re-ping with the same key. Use if the failure was a transient network blip."},
        {"label": "Skip",                    "description": "Re-paste the key. Returns to the paste prompt for this provider."},
        {"label": "Continue without",        "description": "Mark provider unavailable. /chat will fall back to the next provider."}
      ]
    }
  ]
}
```

- `Retry` → call `verify_key $prov` again; one extra retry max.
- `Skip` → set `KEY_STATE_PROV[$prov]=missing`; re-enter paste flow §1b.4.
- `Continue without` → leave `VERIFIED[$prov]=fail`; provider excluded
  from `PROVIDERS_AVAILABLE`.

## 1b.7 — Export state for Phase 2

Build the export array — only providers with `VERIFIED == ok`:

```bash
PROVIDERS_AVAILABLE=()
for prov in "${SELECTED_PROVIDERS[@]}"; do
  [ "${VERIFIED[$prov]}" = "ok" ] && PROVIDERS_AVAILABLE+=("$prov")
done
export PROVIDERS_AVAILABLE
```

Phase 2 reads this to offer route-target choices. Empty array means only
the legacy ANTHROPIC slot from Phase 1 is wired.

## 1b.8 — Final summary

```
Provider state:
  anthropic:  <present | missing | pasted>  verify: <ok | fail | skipped>
  openai:     <present | missing | pasted>  verify: <ok | fail | skipped>
  kimi:       <present | missing | pasted>  verify: <ok | fail | skipped>
  local:      n/a                            verify: ok
PROVIDERS_AVAILABLE=[<list>]
```

## Verify-criterion

- `~/.claude/secrets/.env` mode 600.
- For each selected provider: `KEY_STATE_PROV` is one of `present`/`missing`/`pasted`.
- `VERIFIED` is one of `ok`/`fail (...)`/`skipped`.
- `PROVIDERS_AVAILABLE` exported, contains only `ok` providers.
- ZERO key values in chat or logs (RULE 0.8).
- AskUserQuestion count: 1 (multi-select) + N freeText (≤3 per missing key)
  + M failure clicks (≤1 per failed verify) + K keep/replace clicks
  (≤1 per already-present provider). Bounded by 1 + 3·4 + 4 + 4 = 21.
- Re-running the wizard: every `present` provider routes through §1b.3
  Keep-or-Replace; no automatic overwrite.
