# shellcheck shell=bash
# lib-hooks.sh — hook file copy + settings.json jq-merge.
#
# Hooks are logic (not config) → always refreshed, every install.
# settings.json merge is idempotent: it groups by matcher and unions .hooks
# by unique command so repeated runs never duplicate entries.
#
# Requires: say / warn / err from lib-log.sh.
# Requires: backup_file from lib-backup.sh.
# Reads globals: $KIT_DIR, $HOOKS_DIR, $HOME_DIR.

# Copy every *.sh hook from the kit into $HOOKS_DIR, +x, with per-file backup.
# Also copies hooks/_lib/*.sh → $HOOKS_DIR/_lib/ (v0.17 gate-extract). The
# hooks dot-source "$(dirname "$0")/_lib/gate.sh" — the _lib/ sub-tree MUST
# land alongside the hook scripts or the gate becomes a no-op (fail-open).
install_hooks() {
  say "copying hooks -> $HOOKS_DIR/"
  local hook_count=0 hook_src h
  for hook_src in "$KIT_DIR/hooks/"*.sh; do
    [ -f "$hook_src" ] || continue
    h="$(basename "$hook_src")"
    backup_file "$HOOKS_DIR/$h"
    cp -f "$hook_src" "$HOOKS_DIR/$h"
    chmod +x "$HOOKS_DIR/$h"
    hook_count=$((hook_count+1))
  done
  say "  installed $hook_count hook(s)"

  # v0.17 — shared hook library (gate.sh + test-gate.sh)
  if [ -d "$KIT_DIR/hooks/_lib" ]; then
    mkdir -p "$HOOKS_DIR/_lib"
    local lib_count=0 lib_src lib_name
    for lib_src in "$KIT_DIR/hooks/_lib/"*.sh; do
      [ -f "$lib_src" ] || continue
      lib_name="$(basename "$lib_src")"
      cp -f "$lib_src" "$HOOKS_DIR/_lib/$lib_name"
      chmod +x "$HOOKS_DIR/_lib/$lib_name"
      lib_count=$((lib_count+1))
    done
    say "  installed $lib_count hook library file(s) -> $HOOKS_DIR/_lib/"
  fi
}

# Merge settings-snippet.json into ~/.claude/settings.json non-interactively
# via jq. On first run (no settings.json) we strip _comment and drop in the
# snippet verbatim. On subsequent runs we group by matcher and dedupe .hooks
# by command so re-runs are true no-ops.
# jq-merge snippet into existing target. group_by matcher + dedup by command
# so re-runs are no-ops. Args: $1=snippet, $2=target.
_jq_merge_hooks() {
  local snippet="$1" target="$2" tmp
  tmp="$(mktemp "$target.XXXXXX")"
  jq --slurpfile snip "$snippet" '
    # Normalize a command path: expand leading ~/ to $HOME so tilde and
    # absolute forms compare equal (prevents duplicate hook registration).
    def norm: if startswith("~/") then env.HOME + .[1:] else . end;

    . as $orig
    | ($snip[0] | del(._comment)) as $add
    | reduce ($add.hooks | keys[]) as $phase ($orig;
        .hooks[$phase] = (
          ((.hooks[$phase] // []) + ($add.hooks[$phase] // []))
          | group_by(.matcher)
          | map(
              .[0].matcher as $m
              | {
                  matcher: $m,
                  hooks: (
                    map(.hooks // []) | add
                    # Reduce into object keyed by normalised command.
                    # Last entry wins → snippet (appended last) overrides
                    # existing on collision, preserving all extra fields.
                    | reduce .[] as $h (
                        {};
                        . + { (($h.command // "") | norm): $h }
                      )
                    | [.[]]
                  )
                }
            )
        )
      )
    # statusLine (KeiSei tamagotchi): set ONLY when the target has none.
    # Never clobber an existing statusLine. Fresh-install path drops the
    # snippet verbatim, so this only matters when merging into a
    # pre-existing settings.json.
    | if (.statusLine // null) == null and ($add.statusLine // null) != null
      then .statusLine = $add.statusLine
      else . end
  ' "$target" > "$tmp"
  if [ -s "$tmp" ] && jq -e . "$tmp" >/dev/null 2>&1; then
    mv "$tmp" "$target"
    say "merged hooks into $target (idempotent)"
  else
    rm -f "$tmp"
    err "jq-merge produced invalid output; $target unchanged"
    return 1
  fi
}

activate_hooks() {
  local snippet="$KIT_DIR/settings-snippet.json"
  local target="$HOME_DIR/.claude/settings.json"
  [ -f "$snippet" ] || { warn "no snippet at $snippet"; return 0; }
  if [ ! -f "$target" ]; then
    local tmp
    tmp="$(mktemp "$target.XXXXXX")"
    jq 'del(._comment)' "$snippet" > "$tmp"
    mv "$tmp" "$target"
    say "created $target from snippet (no prior settings.json)"
    return 0
  fi
  backup_file "$target"
  _jq_merge_hooks "$snippet" "$target"
}

# Flag-or-prompt dispatcher, mirroring the v0.15 behavior:
#   --activate-hooks          → always activate, no prompt
#   no existing settings.json → activate silently (drop in snippet)
#   TTY stdin+stdout          → interactive [y/N] prompt
#   otherwise                 → skip (manual-merge hint printed by summary)
# Sets global DID_ACTIVATE=1 when activation ran + succeeded.
maybe_activate_hooks() {
  local settings_file="$HOME_DIR/.claude/settings.json"
  DID_ACTIVATE=0
  if [ "$ACTIVATE_HOOKS" = "1" ]; then
    say "activating hooks (--activate-hooks)"
    activate_hooks && DID_ACTIVATE=1
  elif [ ! -f "$settings_file" ]; then
    say "no existing settings.json; installing snippet"
    activate_hooks && DID_ACTIVATE=1
  elif [ -t 0 ] && [ -t 1 ]; then
    if [ "$COLOR" = "1" ]; then
      printf '\033[1;36m[install]\033[0m activate hooks now? [y/N] '
    else
      printf '[install] activate hooks now? [y/N] '
    fi
    local reply
    read -r reply
    case "$reply" in
      y|Y|yes|YES) activate_hooks && DID_ACTIVATE=1 ;;
      *) say "skipping hook activation" ;;
    esac
  fi
}
