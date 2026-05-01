#!/usr/bin/env bash
# kei-sleep-queue.sh — v0.12.0 "sleep on it" queue CRUD helper.
# Commands: add / list / show / done / fail / purge.
# Env: KEI_MEMORY_REPO_PATH (sourced from ~/.claude/secrets/.env).
set -u

SECRETS_FILE="${HOME}/.claude/secrets/.env"
[ -f "$SECRETS_FILE" ] && [ -z "${KEI_MEMORY_REPO_PATH:-}" ] && \
    . "$SECRETS_FILE" 2>/dev/null || true

REPO_PATH="${KEI_MEMORY_REPO_PATH:-}"
QUEUE_DIR="${REPO_PATH}/sleep-queue"
DONE_DIR="${REPO_PATH}/sleep-queue-done"
FAIL_DIR="${REPO_PATH}/sleep-queue-failed"
SYNC_SH="${HOME}/.claude/agents/_primitives/kei-sleep-sync.sh"

err() { printf 'kei-sleep-queue: %s\n' "$*" >&2; }
die() { err "$*"; exit 1; }

ensure_repo() {
    [ -n "$REPO_PATH" ] || die "KEI_MEMORY_REPO_PATH not set (run /sleep-setup)"
    [ -d "${REPO_PATH}/.git" ] || die "sync-repo not initialised at $REPO_PATH"
    mkdir -p "$QUEUE_DIR" "$DONE_DIR" "$FAIL_DIR" 2>/dev/null || true
}

gen_uuid() {
    if command -v uuidgen >/dev/null 2>&1; then uuidgen | tr 'A-Z' 'a-z'
    else printf '%s-%s' "$(date -u +%s)" "${RANDOM}${RANDOM}"; fi
}

iso_utc() { date -u +%Y-%m-%dT%H:%M:%SZ; }
push_async() { [ -x "$SYNC_SH" ] && "$SYNC_SH" >/dev/null 2>&1 || true; }

# Find a queue file by uuid prefix in dir; echoes path or returns 1.
find_by_uuid() {
    local uuid="$1" dir="$2" f
    [ -d "$dir" ] || return 1
    for f in "$dir/${uuid}-"*.md "$dir/${uuid}.md"; do
        [ -f "$f" ] && { printf '%s\n' "$f"; return 0; }
    done
    return 1
}

# Extract a frontmatter field value (first match) from a file.
fm_field() { awk -F': ' -v k="^$2:" '$0 ~ k {print $2; exit}' "$1"; }

# Parse "<N>m" → N (minutes), or die with context.
parse_minutes() {
    local raw="$1" label="$2" stripped="${1%m}"
    case "$stripped" in ''|*[!0-9]*) die "bad $label: $raw (expected <N>m)" ;; esac
    printf '%s\n' "$stripped"
}

# Priority defaults: TIME_BUDGET_MINUTES, CHECKPOINT_EVERY_MINUTES, MARATHON.
priority_defaults() {
    case "$1" in
        quick)    printf '15 0 false\n' ;;
        standard) printf '60 20 false\n' ;;
        deep)     printf '240 30 false\n' ;;
        marathon) printf '480 30 true\n' ;;
        weekly)   printf '60 20 false\n' ;;
        *) die "bad --priority: $1 (expected quick|standard|deep|marathon|weekly)" ;;
    esac
}

# Validate add flags; sets ADD_* including time-budget / checkpoint / marathon.
parse_add_flags() {
    ADD_TYPE=""; ADD_PRIORITY=""; ADD_FORMAT=""; ADD_PROMPT=""
    ADD_TIME_BUDGET=""; ADD_CHECKPOINT=""; ADD_MARATHON=""; ADD_NO_TIMEOUT=0
    while [ $# -gt 0 ]; do
        case "$1" in
            --type)             ADD_TYPE="$2"; shift 2 ;;
            --priority)         ADD_PRIORITY="$2"; shift 2 ;;
            --format)           ADD_FORMAT="$2"; shift 2 ;;
            --prompt-file)      ADD_PROMPT="$2"; shift 2 ;;
            --time-budget)      ADD_TIME_BUDGET="$(parse_minutes "$2" --time-budget)"; shift 2 ;;
            --checkpoint-every) ADD_CHECKPOINT="$(parse_minutes "$2" --checkpoint-every)"; shift 2 ;;
            --no-timeout)       ADD_NO_TIMEOUT=1; shift ;;
            --marathon)         ADD_MARATHON="true"; shift ;;
            *) die "unknown flag: $1" ;;
        esac
    done
    case "$ADD_TYPE"     in deep|pipeline|pattern|compare|custom) ;; *) die "bad --type: $ADD_TYPE" ;; esac
    case "$ADD_FORMAT"   in md|adr|checklist|table) ;; *) die "bad --format: $ADD_FORMAT" ;; esac
    [ -n "$ADD_PROMPT" ] && [ -f "$ADD_PROMPT" ] || die "missing --prompt-file"
    resolve_priority_fields
}

# Apply priority defaults for any ADD_* fields that weren't overridden.
resolve_priority_fields() {
    local defaults budget cp marathon
    defaults="$(priority_defaults "$ADD_PRIORITY")"
    budget="$(printf '%s' "$defaults" | awk '{print $1}')"
    cp="$(printf '%s' "$defaults" | awk '{print $2}')"
    marathon="$(printf '%s' "$defaults" | awk '{print $3}')"
    [ -z "$ADD_TIME_BUDGET" ] && ADD_TIME_BUDGET="$budget"
    [ -z "$ADD_CHECKPOINT" ]  && ADD_CHECKPOINT="$cp"
    [ -z "$ADD_MARATHON" ]    && ADD_MARATHON="$marathon"
    [ "$ADD_NO_TIMEOUT" = "1" ] && ADD_TIME_BUDGET="null"
    if [ "$ADD_MARATHON" = "true" ] && [ "$ADD_PRIORITY" != "marathon" ]; then
        err "warning: --marathon set but --priority=$ADD_PRIORITY (expected marathon)"
    fi
}

cmd_add() {
    parse_add_flags "$@"
    ensure_repo
    local uuid ts file
    uuid="$(gen_uuid)"
    ts="$(iso_utc)"
    file="${QUEUE_DIR}/${uuid}-$(date -u +%s).md"
    {
        printf -- '---\n'
        printf -- 'uuid: %s\n' "$uuid"
        printf -- 'submitted_at: %s\n' "$ts"
        printf -- 'type: %s\n' "$ADD_TYPE"
        printf -- 'priority: %s\n' "$ADD_PRIORITY"
        printf -- 'format: %s\n' "$ADD_FORMAT"
        printf -- 'time_budget_minutes: %s\n' "$ADD_TIME_BUDGET"
        printf -- 'checkpoint_every_minutes: %s\n' "$ADD_CHECKPOINT"
        printf -- 'marathon: %s\n' "$ADD_MARATHON"
        printf -- 'status: pending\n---\n\n'
        cat "$ADD_PROMPT"
        printf '\n'
    } > "$file" || die "write failed: $file"
    printf '%s\n%s\n' "$uuid" "$file"
    push_async
}

cmd_list() {
    local filter="pending" dir
    [ $# -gt 0 ] && case "$1" in
        --pending) filter="pending" ;;
        --done)    filter="done" ;;
        --failed)  filter="failed" ;;
        *) die "unknown filter: $1" ;;
    esac
    ensure_repo
    case "$filter" in
        pending) dir="$QUEUE_DIR" ;; done) dir="$DONE_DIR" ;; failed) dir="$FAIL_DIR" ;;
    esac
    printf '%-36s  %-10s  %-8s  %-9s  %s\n' UUID SUBMITTED TYPE PRIORITY FILE
    local f u s t p
    for f in "$dir"/*.md; do
        [ -f "$f" ] || continue
        u="$(fm_field "$f" uuid)"
        s="$(fm_field "$f" submitted_at | cut -c1-10)"
        t="$(fm_field "$f" type)"
        p="$(fm_field "$f" priority)"
        printf '%-36s  %-10s  %-8s  %-9s  %s\n' "${u:--}" "${s:--}" "${t:--}" "${p:--}" "$f"
    done
}

cmd_show() {
    [ $# -ge 1 ] || die "usage: show <uuid>"
    ensure_repo
    local uuid="$1" f dir
    for dir in "$QUEUE_DIR" "$DONE_DIR" "$FAIL_DIR"; do
        f="$(find_by_uuid "$uuid" "$dir" 2>/dev/null)" && { cat "$f"; return 0; }
    done
    die "uuid not found: $uuid"
}

cmd_done() {
    [ $# -ge 1 ] || die "usage: done <uuid>"
    ensure_repo
    local src dest uuid="$1"
    src="$(find_by_uuid "$uuid" "$QUEUE_DIR")" || die "pending uuid not found: $uuid"
    dest="${DONE_DIR}/${uuid}.md"
    sed 's/^status: pending$/status: done/' "$src" > "$dest" || die "write failed: $dest"
    rm -f "$src"
    printf 'moved: %s -> %s\n' "$src" "$dest"
    push_async
}

cmd_fail() {
    local uuid="" reason=""
    while [ $# -gt 0 ]; do
        case "$1" in
            --reason) reason="$2"; shift 2 ;;
            *) [ -z "$uuid" ] && { uuid="$1"; shift; } || die "unknown arg: $1" ;;
        esac
    done
    [ -n "$uuid" ] || die "usage: fail <uuid> --reason <text>"
    ensure_repo
    local src dest
    src="$(find_by_uuid "$uuid" "$QUEUE_DIR")" || die "pending uuid not found: $uuid"
    dest="${FAIL_DIR}/${uuid}.md"
    {
        sed 's/^status: pending$/status: failed/' "$src"
        printf '\n---\n## Failure reason\n\n%s\n' "${reason:-(no reason given)}"
    } > "$dest" || die "write failed: $dest"
    rm -f "$src"
    printf 'moved: %s -> %s\n' "$src" "$dest"
    push_async
}

cmd_purge() {
    local days=""
    while [ $# -gt 0 ]; do
        case "$1" in
            --older-than) days="${2%d}"; shift 2 ;;
            *) die "unknown flag: $1" ;;
        esac
    done
    case "$days" in ''|*[!0-9]*) die "--older-than <N>d required (N integer)" ;; esac
    ensure_repo
    local removed=0 f dir
    for dir in "$DONE_DIR" "$FAIL_DIR"; do
        while IFS= read -r f; do
            rm -f "$f" && removed=$((removed + 1))
        done < <(find "$dir" -maxdepth 1 -type f -name '*.md' -mtime "+$days" 2>/dev/null)
    done
    printf 'purged %d file(s) older than %sd\n' "$removed" "$days"
    push_async
}

usage() {
    cat >&2 <<'EOF'
kei-sleep-queue.sh — v0.12 sleep-on-it queue helper

  add --type <deep|pipeline|pattern|compare|custom>
      --priority <quick|standard|deep|marathon|weekly>
      --format <md|adr|checklist|table>
      --prompt-file <path>
      [--time-budget <N>m]       override minutes from priority default
      [--checkpoint-every <M>m]  override partial-result cadence
      [--no-timeout]             time_budget_minutes: null (run until done)
      [--marathon]               explicit marathon flag
  list [--pending|--done|--failed]
  show <uuid>
  done <uuid>
  fail <uuid> --reason <text>
  purge --older-than <N>d

Env: KEI_MEMORY_REPO_PATH (required)
EOF
    exit 1
}

main() {
    [ $# -ge 1 ] || usage
    local sub="$1"; shift
    case "$sub" in
        add)   cmd_add   "$@" ;;
        list)  cmd_list  "$@" ;;
        show)  cmd_show  "$@" ;;
        done)  cmd_done  "$@" ;;
        fail)  cmd_fail  "$@" ;;
        purge) cmd_purge "$@" ;;
        -h|--help|help) usage ;;
        *) die "unknown command: $sub" ;;
    esac
}

main "$@"
