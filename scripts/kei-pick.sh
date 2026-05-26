#!/usr/bin/env bash
# kei-pick — interactive orchestrator picker.
#
# Shows installed LLM CLIs, lets the user choose one, writes it to
# ~/.claude/config/primary.toml, then exec's it (so the shell becomes
# the picked orchestrator). Designed for `kei pick`.
#
# Non-interactive (no TTY): just shows status and exits 0.

set -eu

KEI_PRIMARY_CFG="${KEI_PRIMARY_CFG:-$HOME/.claude/config/primary.toml}"

# Mirrors scripts/kei-agent-cli.sh::backend_bin and bin/kei::backend_bin_for.
backend_bin() {
  case "$1" in
    claude)               echo "claude"  ;;
    grok)                 echo "grok"    ;;
    agy|antigravity)      echo "agy"     ;;
    copilot)              echo "copilot" ;;
    kimi)                 echo "kimi"    ;;
    codex)                echo "codex"   ;;
    *) return 1 ;;
  esac
}

backend_label() {
  case "$1" in
    claude)  echo "Claude Code         (Anthropic)" ;;
    grok)    echo "Grok Build TUI      (xAI)" ;;
    agy)     echo "Antigravity / Gemini (Google)" ;;
    copilot) echo "GitHub Copilot CLI   (Microsoft/GitHub)" ;;
    kimi)    echo "Kimi Code CLI        (Moonshot) — TUI-only for agents" ;;
    codex)   echo "Codex CLI            (OpenAI)" ;;
  esac
}

current_primary() {
  [ -f "$KEI_PRIMARY_CFG" ] || { echo "claude"; return; }
  awk -F'=' '/^provider[[:space:]]*=/ {
    gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
    gsub(/^"|"$/, "", $2)
    print $2; exit
  }' "$KEI_PRIMARY_CFG"
}

# --- list installed backends ------------------------------------------
BACKENDS=(claude grok agy copilot kimi codex)
INSTALLED=()
for b in "${BACKENDS[@]}"; do
  bin=$(backend_bin "$b")
  if command -v "$bin" >/dev/null 2>&1; then
    INSTALLED+=("$b")
  fi
done

cur=$(current_primary)

# --- non-interactive: just show status --------------------------------
# Gate on stdin (RULE TTY-INTERACTIVITY-GATE): -t 0, not -t 1.
# curl|bash tees stdout, so -t 1 false ≠ non-interactive.
if [ ! -t 0 ]; then
  echo "kei pick — non-interactive mode"
  echo "current primary: $cur"
  echo "installed backends: ${INSTALLED[*]:-none}"
  echo "(run \`kei pick\` from a real terminal for the picker)"
  exit 0
fi

# --- interactive picker -----------------------------------------------
C0="" CB="" CC="" CD=""
if [ -t 1 ]; then
  C0=$'\033[0m'
  CB=$'\033[1;38;5;39m'   # blue
  CC=$'\033[1;38;5;220m'  # gold
  CD=$'\033[2m'           # dim
fi

cat <<EOF

${CB}╔════════════════════════════════════════════╗
║   KeiSeiKit · orchestrator picker          ║
╚════════════════════════════════════════════╝${C0}

Pick the LLM CLI that becomes your primary shell.
Any agent invocation (\`kei agent <name>\`) routes here unless DNA overrides.

EOF

i=1
for b in "${BACKENDS[@]}"; do
  bin=$(backend_bin "$b")
  label=$(backend_label "$b")
  if command -v "$bin" >/dev/null 2>&1; then
    mark="${CC}✓${C0}"
  else
    mark="${CD}✗${C0}"
    label="$label ${CD}(not installed)${C0}"
  fi
  cur_mark=""
  [ "$b" = "$cur" ] && cur_mark="${CC} ← current${C0}"
  printf "  ${CB}%d${C0}) %s %-10s %s%s\n" "$i" "$mark" "$b" "$label" "$cur_mark"
  i=$((i+1))
done

echo
printf "  ${CB}q${C0}) cancel (keep current: ${CC}%s${C0})\n\n" "$cur"
printf "Pick [1-${#BACKENDS[@]}/q]: "
read -r choice
choice="${choice:-q}"

case "$choice" in
  q|Q|"") echo "cancelled."; exit 0 ;;
  [1-9])
    idx=$((choice-1))
    if [ $idx -ge ${#BACKENDS[@]} ] || [ $idx -lt 0 ]; then
      echo "invalid choice: $choice" >&2; exit 2
    fi
    new="${BACKENDS[$idx]}"
    ;;
  *) echo "invalid choice: $choice" >&2; exit 2 ;;
esac

bin=$(backend_bin "$new")
if ! command -v "$bin" >/dev/null 2>&1; then
  echo
  echo "${CC}'$new' is not installed.${C0}"
  echo "Set as primary anyway (you'll need to install it before \`kei\` will work)? [y/N]: "
  read -r confirm
  case "$confirm" in y|Y|yes) ;; *) echo "cancelled."; exit 0 ;; esac
fi

mkdir -p "$(dirname "$KEI_PRIMARY_CFG")"
printf '# kei primary — written %s\nprovider = "%s"\n' \
  "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$new" > "$KEI_PRIMARY_CFG"

echo
echo "${CC}✓${C0} primary set: $cur → ${CC}$new${C0}"
echo "  config: $KEI_PRIMARY_CFG"
echo

if [ -n "${KEI_NO_LAUNCH:-}" ]; then
  echo "(skipping launch — KEI_NO_LAUNCH set)"
  exit 0
fi

if ! command -v "$bin" >/dev/null 2>&1; then
  echo "${CD}skipping launch — $bin not on PATH; install it then run \`kei\`.${C0}"
  exit 0
fi

echo "launching $new..."
exec "$bin"
