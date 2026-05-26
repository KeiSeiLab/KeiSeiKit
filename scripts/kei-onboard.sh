#!/usr/bin/env bash
# kei-onboard — post-install wizard.
#
# Runs after install.sh / bootstrap.sh to guide the user through:
#   Step 1: pick the primary LLM orchestrator (default for `kei` no-args)
#   Step 2: wire kei-mcp into the chosen CLI (cross-CLI policy + spawn_agent)
#   Step 3: optional MOONSHOT_API_KEY hint for kei limits
#   Step 4: quick health check
#
# Idempotent — safe to re-run anytime via `kei onboard`.
# Honors TTY gate: non-interactive runs print summary + exit, no prompts.

set -eu

KEI_PRIMARY_CFG="${KEI_PRIMARY_CFG:-$HOME/.claude/config/primary.toml}"
PICK_SH="$HOME/.claude/scripts/kei-pick.sh"
WIRE_SH="$HOME/.claude/scripts/kei-mcp-wire.sh"

# Colors only if stdout is a TTY (TTY-INTERACTIVITY-GATE: -t 1 for color is OK).
C0= CB= CC= CG= CD= CR=
if [ -t 1 ]; then
  C0=$'\033[0m'
  CB=$'\033[1;38;5;39m'    # blue
  CC=$'\033[1;38;5;220m'   # gold
  CG=$'\033[32m'           # green
  CR=$'\033[31m'           # red
  CD=$'\033[2m'            # dim
fi

# Non-interactive (no stdin TTY): print summary + exit.
# Per tty-interactivity-gate.md: -t 0 not -t 1.
if [ ! -t 0 ]; then
  cat <<EOF

${CB}KeiSeiKit · onboarding${C0} (non-interactive — wizard skipped)

Next manual steps:
  ${CC}kei onboard${C0}       run this wizard interactively
  ${CC}kei pick${C0}          pick primary LLM CLI
  ${CC}kei mcp-wire${C0}      wire kei-mcp into your CLIs
  ${CC}kei limits${C0}        check subscription quotas (honest report)
  ${CC}kei-doctor${C0}        substrate health diagnostic

EOF
  exit 0
fi

# Banner
cat <<EOF

${CB}╔═══════════════════════════════════════════════════════════════════╗
║  KeiSeiKit · post-install onboarding                                ║
╚═══════════════════════════════════════════════════════════════════╝${C0}

The install put 38 agents, 54 hooks, and 60+ Rust primitives in place.
Now let's wire up the LLM CLIs you'll actually use.

EOF

# ── Step 1: pick primary ───────────────────────────────────────────
echo "${CB}── Step 1/4 — Pick your primary LLM orchestrator ──${C0}"
echo
echo "When you run ${CC}kei${C0} (no args) it launches your primary CLI."
echo "Each agent's manifest can also declare a preferred provider (DNA)."
echo

declare -a BACKENDS=(claude grok agy copilot kimi)
declare -A LABELS=(
  [claude]="Claude Code       (Anthropic, full hook enforcement)"
  [grok]="Grok              (xAI, native --agent flag)"
  [agy]="Antigravity       (Google Gemini)"
  [copilot]="GitHub Copilot    (Microsoft, MCP-wrapped)"
  [kimi]="Kimi              (Moonshot, TUI-primary)"
)

i=1
for b in "${BACKENDS[@]}"; do
  if command -v "$b" >/dev/null 2>&1; then
    mark="${CG}✓${C0}"
  else
    mark="${CR}✗${C0} ${CD}(not installed)${C0}"
  fi
  printf "  ${CB}%d${C0}) %s %-20s %s\n" "$i" "$mark" "$b" "${LABELS[$b]}"
  i=$((i+1))
done
echo "  ${CB}s${C0}) skip — keep current primary (claude default)"
echo

current=""
[ -f "$KEI_PRIMARY_CFG" ] && current=$(awk -F'=' '/^provider/ {gsub(/[" ]/, "", $2); print $2; exit}' "$KEI_PRIMARY_CFG")
printf "Current primary: ${CC}%s${C0}\n" "${current:-claude (default)}"
printf "Pick [1-${#BACKENDS[@]}/s, default=s]: "
read -r choice
choice="${choice:-s}"

primary_set=""
case "$choice" in
  s|S|"")
    echo "  ${CD}— keeping ${current:-claude}${C0}"
    primary_set="${current:-claude}"
    ;;
  [1-9])
    idx=$((choice-1))
    if [ $idx -ge ${#BACKENDS[@]} ] || [ $idx -lt 0 ]; then
      echo "  ${CR}invalid; keeping ${current:-claude}${C0}"
      primary_set="${current:-claude}"
    else
      new="${BACKENDS[$idx]}"
      mkdir -p "$(dirname "$KEI_PRIMARY_CFG")"
      printf '# kei primary — written %s by onboarding\nprovider = "%s"\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$new" > "$KEI_PRIMARY_CFG"
      echo "  ${CG}✓${C0} primary set: ${CC}${new}${C0} → $KEI_PRIMARY_CFG"
      primary_set="$new"
    fi
    ;;
  *)
    echo "  ${CR}invalid; keeping ${current:-claude}${C0}"
    primary_set="${current:-claude}"
    ;;
esac

# ── Step 2: mcp-wire ───────────────────────────────────────────────
echo
echo "${CB}── Step 2/4 — Wire kei-mcp into installed CLIs ──${C0}"
echo
echo "kei-mcp exposes ${CC}spawn_agent${C0} + ${CC}kei_bash/kei_edit/kei_write${C0} (with"
echo "policy chain) to any MCP-capable CLI. Enables cross-CLI agent invocation"
echo "AND hook enforcement on non-Claude backends."
echo
printf "Run ${CC}kei mcp-wire${C0} now (writes to ~/.grok/, ~/.copilot/, etc.)? [Y/n]: "
read -r wire_ans
wire_ans="${wire_ans:-Y}"
case "$wire_ans" in
  y|Y|yes)
    if [ -x "$WIRE_SH" ]; then
      "$WIRE_SH"
    else
      echo "  ${CR}— $WIRE_SH not found; skip${C0}"
    fi
    ;;
  *)
    echo "  ${CD}— skipped. Run later: ${CC}kei mcp-wire${C0}${CD}${C0}"
    ;;
esac

# ── Step 3: MOONSHOT key hint ──────────────────────────────────────
echo
echo "${CB}── Step 3/4 — Live subscription limits (optional) ──${C0}"
echo
echo "${CC}kei limits${C0} probes each CLI's subscription quota. Research found that"
echo "only Kimi exposes a public API; the others are dashboard-only."
echo
if [ -n "${MOONSHOT_API_KEY:-}" ]; then
  echo "  ${CG}✓${C0} MOONSHOT_API_KEY is set — Kimi balance probing enabled"
else
  cat <<EOF
  ${CD}Optional: set ${CC}MOONSHOT_API_KEY${CD} in ${CC}~/.claude/secrets/.env${CD} to enable
  Kimi balance polling. Other CLIs: see dashboards via ${CC}kei limits${CD}.${C0}
EOF
fi

# ── Step 4: health check ───────────────────────────────────────────
echo
echo "${CB}── Step 4/4 — Health check ──${C0}"
echo
if command -v kei-doctor >/dev/null 2>&1; then
  kei-doctor 2>&1 | head -20 || true
else
  echo "  ${CD}— kei-doctor not on PATH yet. Open new shell + run: ${CC}kei-doctor${C0}"
fi

# ── Done ───────────────────────────────────────────────────────────
cat <<EOF

${CB}╔═══════════════════════════════════════════════════════════════════╗
║  Onboarding complete.                                              ║
╚═══════════════════════════════════════════════════════════════════╝${C0}

Quick-start:
  ${CC}kei${C0}                              launch ${primary_set} (your primary)
  ${CC}kei agent critic "..."${C0}           invoke an agent (DNA → primary)
  ${CC}kei agent --on=grok critic "..."${C0} invoke on a specific backend
  ${CC}kei mcp-wire --list${C0}              show enforcement tiers per CLI
  ${CC}kei limits${C0}                       quota report (where APIs exist)
  ${CC}kei pick${C0}                         re-pick primary anytime
  ${CC}kei configure${C0}                    re-pick hook packs / stack profile

Docs:  ${CD}~/.local/share/keisei/docs/encyclopedia/${C0}
Logs:  ${CD}~/.keisei-install.log${C0}

EOF
