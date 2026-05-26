#!/usr/bin/env bash
# kei-agent-cli — invoke a KeiSeiKit agent via an external LLM CLI backend.
#
# Usage:
#   kei run-via <backend> <agent-name> "<task>"        # by agent name
#   kei run-via <backend> --file=<path> "<task>"       # by agent file path
#   kei run-via list                                   # show backends + status
#   kei run-via --help
#
# Backends (SSoT: _primitives/cli-backends.toml, fallback table below):
#   claude     Claude Code     (claude -p)
#   grok       xAI Grok        (grok --print, native --agent supported)
#   agy        Antigravity     (agy --print) — alias: antigravity
#   copilot    GitHub Copilot  (copilot --prompt)
#   kimi       Moonshot Kimi   (stdin, TUI primary)
#   codex      OpenAI Codex    (codex -p) — register-only if not installed
#
# Reads agent prompt from ~/.claude/agents/<agent-name>.md (assembler output).
# Strips YAML frontmatter, composes with task, execs the CLI.
#
# Env overrides:
#   KEI_AGENTS_DIR       agent .md dir (default: ~/.claude/agents)
#   KEI_NATIVE_AGENT=1   prefer backend's native --agent flag (grok/claude)

set -euo pipefail

KEI_AGENTS_DIR="${KEI_AGENTS_DIR:-$HOME/.claude/agents}"
KEI_NATIVE_AGENT="${KEI_NATIVE_AGENT:-0}"

usage() { sed -n '2,20p' "$0" | sed 's|^# \{0,1\}||'; }

# ---- backend table (SSoT mirror; kept in sync with cli-backends.toml) -----
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

backend_supports_native_agent() {
  case "$1" in claude|grok) return 0 ;; *) return 1 ;; esac
}

# Invoke backend with composed prompt as argument or stdin per backend.
backend_invoke() {
  local backend="$1" prompt="$2" bin agent_name="${3:-}"
  bin=$(backend_bin "$backend") || {
    printf '[kei-agent-cli] unknown backend: %s\n' "$backend" >&2
    return 2
  }
  command -v "$bin" >/dev/null 2>&1 || {
    printf '[kei-agent-cli] %s not on PATH. Install it or pick another backend.\n' "$bin" >&2
    return 127
  }

  # Native --agent path (grok/claude) — pass agent name + task directly.
  if [ "$KEI_NATIVE_AGENT" = "1" ] \
     && [ -n "$agent_name" ] \
     && backend_supports_native_agent "$backend"; then
    printf '[kei-agent-cli] %s --agent %s\n' "$bin" "$agent_name" >&2
    exec "$bin" --agent "$agent_name" --print "${prompt##*TASK FOR THIS RUN:}"
  fi

  case "$backend" in
    claude)               exec "$bin" -p "$prompt" ;;
    grok|agy|antigravity) exec "$bin" --print "$prompt" ;;
    copilot)              exec "$bin" --prompt "$prompt" ;;
    kimi)                 # Kimi non-interactive surface is limited;
                          # stdin works against TUI default mode.
                          printf '%s\n' "$prompt" | exec "$bin" ;;
    codex)                exec "$bin" -p "$prompt" ;;
  esac
}

# ---- agent loader -------------------------------------------------------
load_agent() {
  local name="$1" path
  # explicit path via --file=
  case "$name" in
    --file=*) path="${name#--file=}" ;;
    /*|./*|*/*) path="$name" ;;
    *)         path="$KEI_AGENTS_DIR/$name.md" ;;
  esac
  if [ ! -f "$path" ]; then
    printf '[kei-agent-cli] agent not found: %s\n' "$path" >&2
    if [ -d "$KEI_AGENTS_DIR" ]; then
      printf '  Available (%s): %s\n' "$KEI_AGENTS_DIR" \
        "$(find "$KEI_AGENTS_DIR" -maxdepth 1 -name '*.md' -not -name '_*' 2>/dev/null \
           | xargs -n1 basename 2>/dev/null | sed 's/\.md$//' \
           | sort | head -8 | tr '\n' ' ')..." >&2
    fi
    return 1
  fi
  # strip YAML frontmatter (---\n...\n---) if present
  awk '
    BEGIN { in_fm=0; past_fm=0 }
    NR==1 && /^---$/ { in_fm=1; next }
    in_fm && /^---$/ { in_fm=0; past_fm=1; next }
    in_fm { next }
    { print }
  ' "$path"
}

# ---- subcommands --------------------------------------------------------
case "${1:-}" in
  ""|-h|--help|help) usage; exit 0 ;;
  list|--list)
    printf 'Backends (✓ installed, ✗ missing):\n'
    for b in claude grok agy copilot kimi codex; do
      bin=$(backend_bin "$b")
      if p=$(command -v "$bin" 2>/dev/null); then
        printf '  %-10s ✓ %s\n' "$b" "$p"
      else
        printf '  %-10s ✗ (not on PATH)\n' "$b"
      fi
    done
    printf '\nAgents (%s):\n' "$KEI_AGENTS_DIR"
    if [ -d "$KEI_AGENTS_DIR" ]; then
      find "$KEI_AGENTS_DIR" -maxdepth 1 -name '*.md' -not -name '_*' 2>/dev/null \
        | xargs -n1 basename 2>/dev/null | sed 's/\.md$/  /' | sort | column 2>/dev/null \
        || (find "$KEI_AGENTS_DIR" -maxdepth 1 -name '*.md' -not -name '_*' \
            | xargs -n1 basename | sed 's/\.md$//' | sort | head -20)
    fi
    exit 0
    ;;
esac

# ---- main ---------------------------------------------------------------
if [ $# -lt 3 ]; then
  usage
  exit 2
fi

BACKEND="$1"; AGENT_REF="$2"; shift 2
TASK="$*"

if ! AGENT_PROMPT=$(load_agent "$AGENT_REF"); then
  exit 1
fi

# Compose: agent system + task delimiter.
COMPOSED=$(printf '%s\n\n---\n\nTASK FOR THIS RUN:\n%s\n' "$AGENT_PROMPT" "$TASK")

# Derive a clean agent name for KEI_NATIVE_AGENT path.
AGENT_NAME=$(basename "${AGENT_REF#--file=}")
AGENT_NAME="${AGENT_NAME%.md}"

backend_invoke "$BACKEND" "$COMPOSED" "$AGENT_NAME"
