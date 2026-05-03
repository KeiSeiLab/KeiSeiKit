#!/bin/bash
# disk-reclaim.sh — nightly orphan-worktree + stale-target/ reclaim
# RULE 0.17 (disk-headroom). Phase C step 5 logic, executed locally
# via launchd (independent of cloud /schedule).
#
# Schedule: ~/Library/LaunchAgents/io.keisei.disk-reclaim.plist (daily 03:30)
# Output:   ~/.claude/disk-reclaim.log
#
# Four guards before removing any worktree:
#   1. mtime ≥ 168h (7 days) — no file newer than reference timestamp
#   2. git status --porcelain empty
#   3. zero unpushed commits to upstream
#   4. lockfile PID dead OR no lockfile
#
# Stale target/: same 168h floor, recurse for true mtime.
#
# Pure BSD-portable: uses `stat -f` and `find -newer` only. Safe in
# launchd / non-interactive bash where shell-function `find` proxies
# (e.g. Claude Code's bfs) are not loaded.

set -u

LOG="$HOME/.claude/disk-reclaim.log"
STAMP="$HOME/.claude/disk-reclaim.stamp"

log() {
  printf '[%s] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*" >> "$LOG"
}

free_gb() {
  df -g /System/Volumes/Data 2>/dev/null | tail -1 | awk '{print $4}'
}

# Build reference file with mtime exactly 168h ago.
REF_168H="/tmp/.disk-reclaim-168h.$$"
touch -t "$(date -v-168H +%Y%m%d%H%M.%S)" "$REF_168H" 2>/dev/null || {
  log "FATAL: touch -t failed (BSD date -v required)"
  exit 1
}

free_before=$(free_gb)
log "=== START reclaim run; free_before=${free_before}G ref_168h=$(stat -f '%m' "$REF_168H")"

worktrees_pruned=0
worktrees_skip_young=0
worktrees_skip_dirty=0
worktrees_skip_unpushed=0
worktrees_skip_livepid=0
worktrees_skip_empty=0
target_pruned=0
target_kb=0

PROJECTS_ROOT="$HOME/Projects"

# Stage A — orphan worktrees
shopt -s nullglob
for proj_git in "$PROJECTS_ROOT"/*/.claude/worktrees "$PROJECTS_ROOT"/*/*/.claude/worktrees; do
  [ -d "$proj_git" ] || continue
  proj=$(dirname "$(dirname "$proj_git")")
  git_dir=$(cd "$proj" && git rev-parse --git-dir 2>/dev/null) || continue

  for wt in "$proj_git"/*/; do
    [ -d "$wt" ] || continue
    wt=${wt%/}
    wt_name=$(basename "$wt")

    # Guard 1: mtime ≥ 168h
    # Approach: find ANY file inside worktree (excluding .git/) newer than ref.
    # If find returns at least one path → worktree is "young" → skip.
    if find "$wt" -type f -newer "$REF_168H" -not -path "$wt/.git/*" -print -quit 2>/dev/null | grep -q .; then
      worktrees_skip_young=$((worktrees_skip_young + 1))
      continue
    fi

    # Guard against empty worktree — make sure SOMETHING exists at all.
    if ! find "$wt" -type f -not -path "$wt/.git/*" -print -quit 2>/dev/null | grep -q .; then
      worktrees_skip_empty=$((worktrees_skip_empty + 1))
      continue
    fi

    # Guard 2: dirty
    if ! ( cd "$wt" && [ -z "$(git status --porcelain 2>/dev/null)" ] ); then
      worktrees_skip_dirty=$((worktrees_skip_dirty + 1))
      log "  SKIP[dirty]    $wt"
      continue
    fi

    # Guard 3: unpushed (fail-safe — skip on missing upstream)
    if cd "$wt" 2>/dev/null && git rev-parse --abbrev-ref @{u} >/dev/null 2>&1; then
      unpushed=$(git log @{u}.. 2>/dev/null | wc -l | tr -d ' ')
      if [ -n "$unpushed" ] && [ "$unpushed" != "0" ]; then
        worktrees_skip_unpushed=$((worktrees_skip_unpushed + 1))
        log "  SKIP[unpushed=$unpushed] $wt"
        continue
      fi
    else
      # No upstream tracking — treat as "may have unpushed work", skip conservatively
      worktrees_skip_unpushed=$((worktrees_skip_unpushed + 1))
      log "  SKIP[no-upstream]  $wt"
      continue
    fi

    # Guard 4: live PID lock
    lockfile="$git_dir/worktrees/$wt_name/locked"
    if [ -f "$lockfile" ]; then
      pid=$(grep -oE 'pid [0-9]+' "$lockfile" 2>/dev/null | awk '{print $2}' | head -1)
      if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        worktrees_skip_livepid=$((worktrees_skip_livepid + 1))
        log "  SKIP[live PID=$pid] $wt"
        continue
      fi
      ( cd "$proj" && git worktree unlock "$wt" 2>/dev/null )
    fi

    branch=$(cd "$wt" && git branch --show-current 2>/dev/null)
    sz_mb=$(du -sk "$wt" 2>/dev/null | awk '{printf "%d", $1/1024}')
    if ( cd "$proj" && git worktree remove --force "$wt" 2>/dev/null ); then
      ( cd "$proj" && [ -n "$branch" ] && git branch -D "$branch" 2>/dev/null )
      worktrees_pruned=$((worktrees_pruned + 1))
      log "  PRUNED  size=${sz_mb}MB  $wt"
    fi
  done
done

# Stage B — stale target/ (with launchd-aware protection)
#
# Build protected-paths set from ALL launchd plists referencing
# anything under target/release. Phase 3 REM (2026-04-29 incident)
# was killed because we deleted KeiSeiKit-p3/.../target which contained
# kei-pipe and kei-phase-store binaries used by io.keisei.phase3.rem-cycle.
# Protected project ROOTS: any launchd plist references file under ~/<X>
# → all target/ inside that project are protected (recursively).
PROTECTED_ROOTS=()
add_root() {
  local r="$1"
  [ -z "$r" ] && return 0
  PROTECTED_ROOTS+=("$r")
}
for plist in "$HOME"/Library/LaunchAgents/*.plist /Library/LaunchAgents/*.plist; do
  [ -f "$plist" ] || continue
  for tier in 0 1 2 3 4; do
    arg=$(plutil -extract "ProgramArguments.$tier" raw "$plist" 2>/dev/null)
    [ -z "$arg" ] && arg=$(plutil -extract Program raw "$plist" 2>/dev/null)
    [ -z "$arg" ] && continue
    case "$arg" in
      "$HOME/Projects/"*)
        # Project root = first 2 components after $HOME/Projects/
        rest="${arg#$HOME/Projects/}"
        proj_first="${rest%%/*}"
        rest2="${rest#*/}"
        proj_second="${rest2%%/*}"
        # Single-level: ~/Projects/Foo/...  vs nested: ~/Projects/Foo/Bar/...
        # We'll be conservative — protect the immediate project dir.
        add_root "$HOME/Projects/$proj_first"
        ;;
      "$HOME/.claude/"*)
        # Anything under ~/.claude (substrate) — protect
        add_root "$HOME/.claude"
        ;;
    esac
  done
done
# Dedupe
if [ ${#PROTECTED_ROOTS[@]} -gt 0 ]; then
  IFS=$'\n' read -r -d '' -a PROTECTED_ROOTS < <(printf '%s\n' "${PROTECTED_ROOTS[@]}" | sort -u && printf '\0')
fi

is_protected() {
  local t="$1"
  for r in "${PROTECTED_ROOTS[@]}"; do
    case "$t" in
      "$r"/*|"$r") return 0 ;;
    esac
  done
  return 1
}

log "  protected project roots: ${#PROTECTED_ROOTS[@]}"
for r in "${PROTECTED_ROOTS[@]}"; do log "    $r"; done

while IFS= read -r t; do
  [ -d "$t" ] || continue

  # Protection: NEVER touch a target/ that holds a launchd-referenced binary.
  if is_protected "$t"; then
    log "  SKIP[launchd-protected] $t"
    continue
  fi

  # Test if any descendant file is newer than ref → if yes, target is "fresh"
  if find "$t" -type f -newer "$REF_168H" -print -quit 2>/dev/null | grep -q .; then
    continue
  fi

  # Empty target/? skip
  if ! find "$t" -type f -print -quit 2>/dev/null | grep -q .; then
    continue
  fi

  sz_kb=$(du -sk "$t" 2>/dev/null | awk '{print $1}')
  if rm -rf "$t" 2>/dev/null; then
    target_pruned=$((target_pruned + 1))
    target_kb=$((target_kb + sz_kb))
    log "  TARGET  -${sz_kb}KB  $t"
  fi
done < <(find "$PROJECTS_ROOT" -maxdepth 6 -type d -name "target" 2>/dev/null)

free_after=$(free_gb)
reclaimed=$((free_after - free_before))

log "=== DONE free_after=${free_after}G reclaimed=${reclaimed}G"
log "    worktrees: pruned=$worktrees_pruned young=$worktrees_skip_young empty=$worktrees_skip_empty dirty=$worktrees_skip_dirty unpushed=$worktrees_skip_unpushed livepid=$worktrees_skip_livepid"
log "    target/:   pruned=$target_pruned size_kb=$target_kb"

date +%s > "$STAMP"
rm -f "$REF_168H"

exit 0
