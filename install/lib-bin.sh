set -e
# shellcheck shell=bash
# lib-bin.sh — install entry-point scripts from kit's `bin/` into
# `~/.claude/bin/`. Today this is just `keisei` (TUI splash → claude),
# but the cube is generic so future entry points (e.g. `keisei-status`,
# `keisei-doctor`) can ship the same way.
#
# Constructor Pattern: 1 file = 1 concern (bin install + symlink),
# <200 LOC.
#
# Public API:
#   install_bin                    -> copy kit/bin/* → ~/.claude/bin/<name>
#                                     (symlink preferred, copy as fallback)
#
# Reads globals: $KIT_DIR, $HOME_DIR.
# Requires: say / warn / err from lib-log.sh.

# Target directory for entry-point scripts. Stays under ~/.claude/ so
# everything KeiSeiKit installs lives in one tree.
_keisei_bin_dir() {
  printf '%s\n' "$HOME_DIR/.claude/bin"
}

# Install all executable scripts from $KIT_DIR/bin/ into ~/.claude/bin/.
# Idempotent: re-running replaces existing symlinks/files.
#
# Strategy:
#   - Symlink when possible (lets `keisei --help` reflect kit edits live).
#   - Fall back to copy if symlink would dangle (e.g. install + delete kit).
install_bin() {
  local src="$KIT_DIR/bin"
  local dst
  dst="$(_keisei_bin_dir)"

  if [ ! -d "$src" ]; then
    say "no bin/ in kit — skipping"
    return 0
  fi

  mkdir -p "$dst"

  local count=0
  for f in "$src"/*; do
    [ -f "$f" ] || continue
    local name
    name="$(basename "$f")"
    local target="$dst/$name"

    # Replace existing entry deterministically — symlink > copy.
    if [ -L "$target" ] || [ -f "$target" ]; then
      rm -f "$target"
    fi

    if ln -s "$f" "$target" 2>/dev/null; then
      :
    else
      cp -f "$f" "$target"
    fi
    chmod +x "$target" 2>/dev/null || chmod +x "$f"
    count=$((count + 1))
  done

  if [ "$count" -gt 0 ]; then
    say "bin: installed $count entry-point script(s) → $dst"
  else
    say "bin: nothing to install"
  fi
}

# Print one-line status for the summary banner.
bin_summary() {
  local dst
  dst="$(_keisei_bin_dir)"
  if [ -d "$dst" ]; then
    local n
    n=$(find "$dst" -maxdepth 1 -type l -o -type f 2>/dev/null | wc -l | tr -d ' ')
    printf 'bin: %s entry-point(s) under %s\n' "$n" "$dst"
  fi
}
