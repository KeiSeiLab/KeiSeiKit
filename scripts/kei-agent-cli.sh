#!/usr/bin/env bash
# kei-agent-cli — invoke a KeiSeiKit agent via an external LLM CLI backend.
#
# Two entry points (both route through this script):
#
#   kei run-via <backend> <agent> "<task>"       # explicit backend
#   kei agent <agent> "<task>"                   # backend resolved from DNA:
#                                                #   1. --on=<backend> flag
#                                                #   2. agent manifest's `provider`
#                                                #   3. ~/.claude/config/primary.toml
#                                                #   4. fallback: claude
#
# Other forms:
#   kei run-via list                             # show backends + agents
#   kei agent --on=<backend> <agent> "<task>"    # override DNA backend
#   kei primary                                  # print current primary
#   kei primary <backend>                        # set primary provider
#   kei run-via --help
#
# Backends (SSoT: _primitives/cli-backends.toml):
#   claude  grok  agy  copilot  kimi  codex
#
# Reads assembled prompt from ~/.claude/agents/<agent-name>.md.
# Strips YAML frontmatter, composes with task, execs the CLI.
#
# Env overrides:
#   KEI_AGENTS_DIR       agent .md dir (default: ~/.claude/agents)
#   KEI_MANIFESTS_DIR    manifest .toml dir (default: ~/.claude/_manifests)
#   KEI_PRIMARY          override primary backend (beats config file)
#   KEI_NATIVE_AGENT=1   prefer backend's native --agent flag (grok/claude)

set -euo pipefail

KEI_AGENTS_DIR="${KEI_AGENTS_DIR:-$HOME/.claude/agents}"
KEI_MANIFESTS_DIR="${KEI_MANIFESTS_DIR:-$HOME/.claude/_manifests}"
KEI_PRIMARY_CFG="${KEI_PRIMARY_CFG:-$HOME/.claude/config/primary.toml}"
KEI_NATIVE_AGENT="${KEI_NATIVE_AGENT:-0}"

usage() { sed -n '2,32p' "$0" | sed 's|^# \{0,1\}||'; }

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

# ---- DNA resolver: agent → preferred backend --------------------------------
# Reads `provider = "..."` line from the manifest TOML if present.
manifest_provider() {
  local agent="$1" tomlf="$KEI_MANIFESTS_DIR/$1.toml"
  [ -f "$tomlf" ] || return 1
  awk -F'=' '
    /^provider[[:space:]]*=/ {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
      gsub(/^"|"$/, "", $2)
      print $2; exit
    }
  ' "$tomlf"
}

# Reads primary from config file (or KEI_PRIMARY env override).
config_primary() {
  if [ -n "${KEI_PRIMARY:-}" ]; then
    printf '%s\n' "$KEI_PRIMARY"; return 0
  fi
  [ -f "$KEI_PRIMARY_CFG" ] || return 1
  awk -F'=' '
    /^provider[[:space:]]*=/ {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
      gsub(/^"|"$/, "", $2)
      print $2; exit
    }
  ' "$KEI_PRIMARY_CFG"
}

# Resolution order: explicit --on= → manifest provider → primary → claude.
resolve_backend() {
  local agent="$1" explicit="${2:-}" out=""
  if [ -n "$explicit" ]; then printf '%s\n' "$explicit"; return 0; fi
  out=$(manifest_provider "$agent" 2>/dev/null) || true
  if [ -n "$out" ]; then printf '%s\n' "$out"; return 0; fi
  out=$(config_primary 2>/dev/null) || true
  if [ -n "$out" ]; then printf '%s\n' "$out"; return 0; fi
  printf 'claude\n'
}

# ---- backend invocation ---------------------------------------------------
backend_invoke() {
  local backend="$1" prompt="$2" agent_name="${3:-}" bin
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

  # v0.41 fix: headless subprocess invocation of claude/grok without
  # --dangerously-skip-permissions returns empty (the agent's system prompt
  # asks for Read/Grep tools, but those need permission prompts which can't
  # be answered in -p mode). Pass the flag so the agent actually executes.
  # Override via KEI_AGENT_PERMISSIVE=0 to keep the strict default.
  local permissive_claude="" permissive_grok=""
  if [ "${KEI_AGENT_PERMISSIVE:-1}" = "1" ]; then
    permissive_claude="--permission-mode=bypassPermissions"
    permissive_grok="--always-approve"
  fi

  case "$backend" in
    claude)               exec "$bin" $permissive_claude -p "$prompt" ;;
    grok)                 exec "$bin" $permissive_grok --print "$prompt" ;;
    agy|antigravity)      exec "$bin" --dangerously-skip-permissions --print "$prompt" ;;
    copilot)              exec "$bin" --prompt "$prompt" ;;
    kimi)
      # Kimi has NO one-shot print mode (smoke-tested 2026-05-26): bare `kimi`
      # opens an interactive TUI that ignores piped stdin and exits with "Bye!".
      # For headless invocation we'd need an ACP client (`kimi acp` is a JSON-RPC
      # stdio server). Until KeiSeiKit ships that client, dump the composed
      # prompt to a tmpfile and open the TUI so the user can paste it in.
      tmp=$(mktemp -t kei-agent-kimi.XXXX.md)
      printf '%s\n' "$prompt" > "$tmp"
      printf '[kei-agent-cli] kimi non-interactive is unsupported (TUI only).\n' >&2
      printf '[kei-agent-cli] composed prompt saved: %s\n' "$tmp" >&2
      printf '[kei-agent-cli] copy-paste it into Kimi after the TUI opens.\n' >&2
      printf '[kei-agent-cli] (or pipe via `kimi acp` if you have an ACP client.)\n' >&2
      exec "$bin"
      ;;
    codex)                exec "$bin" -p "$prompt" ;;
  esac
}

# ---- agent loader -------------------------------------------------------
load_agent() {
  local name="$1" path
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
  awk '
    BEGIN { in_fm=0 }
    NR==1 && /^---$/ { in_fm=1; next }
    in_fm && /^---$/ { in_fm=0; next }
    in_fm { next }
    { print }
  ' "$path"
}

# ---- primary subcommand ------------------------------------------------
handle_primary() {
  local arg="${1:-}"
  if [ -z "$arg" ]; then
    cur=$(config_primary 2>/dev/null || true)
    printf 'primary provider: %s\n' "${cur:-claude (default fallback)}"
    [ -f "$KEI_PRIMARY_CFG" ] && printf 'config: %s\n' "$KEI_PRIMARY_CFG"
    return 0
  fi
  backend_bin "$arg" >/dev/null || {
    printf '[kei-primary] unknown backend: %s\n' "$arg" >&2
    printf 'valid: claude grok agy copilot kimi codex\n' >&2
    return 2
  }
  mkdir -p "$(dirname "$KEI_PRIMARY_CFG")"
  printf '# kei primary — written %s\nprovider = "%s"\n' \
    "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$arg" > "$KEI_PRIMARY_CFG"
  printf 'primary provider set: %s → %s\n' "$arg" "$KEI_PRIMARY_CFG"
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
    cur=$(config_primary 2>/dev/null || true)
    printf '\nprimary: %s\n' "${cur:-claude (default)}"
    printf '\nAgents (%s):\n' "$KEI_AGENTS_DIR"
    if [ -d "$KEI_AGENTS_DIR" ]; then
      find "$KEI_AGENTS_DIR" -maxdepth 1 -name '*.md' -not -name '_*' 2>/dev/null \
        | xargs -n1 basename 2>/dev/null | sed 's/\.md$/  /' | sort | column 2>/dev/null \
        || (find "$KEI_AGENTS_DIR" -maxdepth 1 -name '*.md' -not -name '_*' \
            | xargs -n1 basename | sed 's/\.md$//' | sort | head -20)
    fi
    exit 0
    ;;
  primary)
    shift
    handle_primary "${1:-}"
    exit $?
    ;;
  agent)
    # Direct-invocation passthrough: `kei-agent-cli.sh agent <name> "task"`
    # behaves identically to `kei-agent-cli.sh <name> "task"` (DNA mode).
    # Lets users call either form without surprise.
    shift
    ;;
esac

# ---- main: DNA mode (no leading backend) OR explicit run-via ------------
# Detect call shape:
#   "$1" is a known backend → run-via flow (kei run-via <backend> <agent> "task")
#   "$1" starts with --on=  → DNA flow with override
#   "$1" is anything else   → DNA flow (kei agent <agent> "task")

EXPLICIT_BACKEND=""
case "${1:-}" in
  --on=*)
    EXPLICIT_BACKEND="${1#--on=}"
    shift
    ;;
  *)
    if [ $# -ge 1 ] && backend_bin "$1" >/dev/null 2>&1; then
      EXPLICIT_BACKEND="$1"
      shift
    fi
    ;;
esac

if [ $# -lt 2 ]; then
  usage
  exit 2
fi

AGENT_REF="$1"; shift
TASK="$*"

AGENT_NAME=$(basename "${AGENT_REF#--file=}")
AGENT_NAME="${AGENT_NAME%.md}"

BACKEND=$(resolve_backend "$AGENT_NAME" "$EXPLICIT_BACKEND")

if ! AGENT_PROMPT=$(load_agent "$AGENT_REF"); then
  exit 1
fi

COMPOSED=$(printf '%s\n\n---\n\nTASK FOR THIS RUN:\n%s\n' "$AGENT_PROMPT" "$TASK")

printf '[kei-agent-cli] agent=%s backend=%s (via %s)\n' \
  "$AGENT_NAME" "$BACKEND" \
  "$([ -n "$EXPLICIT_BACKEND" ] && echo explicit \
     || ([ -n "$(manifest_provider "$AGENT_NAME" 2>/dev/null)" ] && echo manifest \
         || ([ -n "$(config_primary 2>/dev/null)" ] && echo primary || echo default)))" >&2

backend_invoke "$BACKEND" "$COMPOSED" "$AGENT_NAME"
