#!/usr/bin/env bash
# sleep-report-tg.sh — Phase B → Telegram delivery hook.
#
# Runs as the FINAL step of phase-b-rem.sh after the report is committed +
# pushed to the memory-repo. Spawns a cloud Claude agent to read the report
# + tracking digests + cross-session analyze, distil 3-5 actionable
# findings, then sends to the whitelisted chat_id via @KeiSeiBot.
#
# Defensive: never blocks Phase B exit code. Failures land in
# ~/.claude/memory/sleep-report-tg-errors.log and exit 0.
#
# Required env (RULE 0.8 — secrets in ~/.claude/secrets/.env):
#   TELEGRAM_BOT_TOKEN          — @KeiSeiBot
#   TELEGRAM_ALLOWED_CHAT_ID    — 86059912 (Parfionovich, single-user whitelist)
#   ANTHROPIC_API_KEY           — for the reasoning step
#
# Bypass: SLEEP_REPORT_TG_BYPASS=1 ...
#
# Usage (called from phase-b-rem.sh after `git push`):
#   ~/.claude/hooks/sleep-report-tg.sh "$REPORT" "$TODAY"
#     $1 = path to reports/sleep-YYYY-MM-DD.md (required)
#     $2 = TODAY date string (required, YYYY-MM-DD)

set -u

ERR_LOG="${HOME}/.claude/memory/sleep-report-tg-errors.log"

log_err() {
    mkdir -p "$(dirname "$ERR_LOG")" 2>/dev/null || return 0
    printf '[%s] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*" >> "$ERR_LOG" 2>/dev/null || true
}

[ "${SLEEP_REPORT_TG_BYPASS:-0}" = "1" ] && exit 0

REPORT="${1:-}"
TODAY="${2:-$(date +%Y-%m-%d)}"
[ -f "$REPORT" ] || { log_err "report not found: $REPORT"; exit 0; }

# Source secrets if env vars not already set.
SECRETS_FILE="${HOME}/.claude/secrets/.env"
if [ -f "$SECRETS_FILE" ] && [ -z "${TELEGRAM_BOT_TOKEN:-}" ]; then
    set -a
    # shellcheck disable=SC1090
    . "$SECRETS_FILE" 2>/dev/null || true
    set +a
fi

[ -n "${TELEGRAM_BOT_TOKEN:-}" ] || { log_err "TELEGRAM_BOT_TOKEN unset"; exit 0; }
CHAT_ID="${TELEGRAM_ALLOWED_CHAT_ID:-86059912}"

command -v curl >/dev/null 2>&1 || { log_err "curl missing"; exit 0; }
command -v jq >/dev/null 2>&1 || { log_err "jq missing"; exit 0; }

# ---- Cloud-agent reasoning step -------------------------------------------
# Send the report to Claude API for distillation. Keep it cheap with Sonnet
# 4.6 — the task is summary, not generation. Caps + max_tokens prevent
# runaway cost on a malformed report.
TG_AGENT_LOG="${HOME}/.claude/memory/sleep-report-tg-agent.log"

if [ -z "${ANTHROPIC_API_KEY:-}" ]; then
    log_err "ANTHROPIC_API_KEY unset — sending raw report excerpt"
    SUMMARY=$(head -c 3500 "$REPORT")
else
    REPORT_BODY=$(cat "$REPORT")
    PROMPT_BODY=$(jq -n --arg r "$REPORT_BODY" '
{
  "model": "claude-sonnet-4-6",
  "max_tokens": 1500,
  "system": "You are the KeiSei sleep-report distiller. The user wakes up and reads ONE Telegram message summarizing what happened in their dev environment overnight. Be ruthlessly concise.\n\nMandatory output structure (max 3500 chars total, plain text suitable for Telegram, no markdown beyond *bold* and `code`):\n\n1. ONE-LINE TL;DR — what is the single most important thing the user should know.\n2. NUMBERS — agent outcomes (functional/partial/scaffolding/fail counts), cost burned, top tool-call categories. Cite raw values from the digests, no interpretation.\n3. ACTIONABLE FINDINGS — 3-5 bullet points. Each must be: a concrete observation FROM THE DATA + suggested action. Examples: \"Opus burned $X with 0 functional outcomes — investigate or accept as legacy noise\", \"Skill X has Y% success rate over N runs — candidate for archive\". DO NOT invent findings without data backing.\n4. RULE-CANDIDATES — if any cross-session pattern in the analyze section appeared >=3 times, name it + suggest a /escalate-recurrence command. Skip section if nothing qualifies.\n\nNO emoji except a single 💤 prefix. NO chat-style preamble (\"Доброе утро!\"). NO closing pleasantries. Direct technical brief.",
  "messages": [{"role":"user","content":[{"type":"text","text":$r}]}]
}')

    RESP=$(curl -sS -X POST "https://api.anthropic.com/v1/messages" \
        -H "Content-Type: application/json" \
        -H "x-api-key: $ANTHROPIC_API_KEY" \
        -H "anthropic-version: 2023-06-01" \
        --max-time 120 \
        -d "$PROMPT_BODY" 2>/dev/null)

    printf '[%s]\n%s\n---\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$RESP" >> "$TG_AGENT_LOG"

    SUMMARY=$(printf '%s' "$RESP" | jq -r '.content[0].text // empty' 2>/dev/null)
    if [ -z "$SUMMARY" ]; then
        ERR=$(printf '%s' "$RESP" | jq -r '.error.message // "unknown"' 2>/dev/null)
        log_err "claude api returned no text (error: $ERR) — falling back to raw excerpt"
        SUMMARY=$(head -c 3500 "$REPORT")
    fi
fi

# ---- Telegram send --------------------------------------------------------
# Telegram message limit is 4096 chars. We cap at 3900 to leave room for
# the prefix + headers. If the cloud agent went over budget, hard-truncate
# rather than splitting (one cohesive message > two fragmented).
HEADER="💤 *Sleep report* — ${TODAY}"$'\n\n'
BODY="${HEADER}${SUMMARY}"
BODY=$(printf '%s' "$BODY" | head -c 3900)

# Use --data-urlencode for safe transport of newlines / specials.
HTTP_RESP=$(curl -sS -X POST "https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/sendMessage" \
    --max-time 30 \
    --data-urlencode "chat_id=${CHAT_ID}" \
    --data-urlencode "parse_mode=Markdown" \
    --data-urlencode "text=${BODY}" 2>/dev/null)

OK=$(printf '%s' "$HTTP_RESP" | jq -r '.ok' 2>/dev/null)
if [ "$OK" = "true" ]; then
    MSG_ID=$(printf '%s' "$HTTP_RESP" | jq -r '.result.message_id' 2>/dev/null)
    log_err "INFO sent to chat=${CHAT_ID} msg_id=${MSG_ID} report=${TODAY}"
else
    DESC=$(printf '%s' "$HTTP_RESP" | jq -r '.description' 2>/dev/null)
    log_err "send failed: $DESC"

    # Markdown parse errors are a known failure mode (orphan * / [) — retry
    # without parse_mode so the user at least sees the report verbatim.
    if printf '%s' "$DESC" | grep -qi "parse"; then
        curl -sS -X POST "https://api.telegram.org/bot${TELEGRAM_BOT_TOKEN}/sendMessage" \
            --max-time 30 \
            --data-urlencode "chat_id=${CHAT_ID}" \
            --data-urlencode "text=${BODY}" >/dev/null 2>&1 || true
    fi
fi

exit 0
