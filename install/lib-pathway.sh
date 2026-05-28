set -e
# shellcheck shell=bash
# lib-pathway.sh — PATH wiring for KeiSeiKit substrate.
#
# Adds two dirs to the user's shell PATH (idempotent block, marker-fenced):
#   1. ~/.cargo/bin/   — canonical Rust binary install location (v0.18+)
#   2. ~/.claude/bin/  — entry-point scripts (keisei) + symlinks
#
# Architecture (v0.18+): substrate Rust binaries install to ~/.cargo/bin/
# (universal Rust idiom, present in PATH for any rustup user). The pre-v0.18
# dual-location pattern (~/.claude/agents/.../target/release in PATH) is
# DEPRECATED — see commit 1d92f7e in KeiSeiKit-public for the migration.
# Existing users get the old block stripped and the new one written on
# next install.sh / pathway_install run.
#
# Public API:
#   pathway_detect_shell           -> bash | zsh | fish | unknown
#   pathway_install                -> dispatch + write block (default flow)
#   pathway_uninstall              -> remove block from every supported rc
#
# Requires: say / warn from lib-log.sh.
# Reads globals: $HOME_DIR, $AGENTS_DIR.

# Marker fences. Short, unmistakable, never collide with user content.
_PATHWAY_BEGIN='# >>> kei-substrate <<<'
_PATHWAY_END='# <<< kei-substrate >>>'

# Detect the user's interactive shell. Falls through to $SHELL basename so
# non-login installs (cron, CI) still get a sensible default.
pathway_detect_shell() {
  local sh
  sh="$(basename "${SHELL:-/bin/bash}" 2>/dev/null)"
  case "$sh" in
    bash|zsh|fish) printf '%s\n' "$sh" ;;
    *) printf 'unknown\n' ;;
  esac
}

# Render the bash/zsh PATH-export block. POSIX-style $PATH prepend, guarded
# to skip a duplicate prepend if already present in the current shell.
# Two paths wired:
#   1. ~/.cargo/bin/    — Rust primitives (kei-fork / kei-ledger / ...)
#   2. ~/.claude/bin/   — entry-point scripts (keisei) + symlinks
_render_posix_block() {
  local cargo_dir="$HOME_DIR/.cargo/bin"
  local bin_dir="$HOME_DIR/.claude/bin"
  cat <<EOF
$_PATHWAY_BEGIN
# Added by KeiSeiKit install.sh — Rust primitives + claude/bin on PATH.
# Remove this block (with markers) to opt out.
for _kei_path in "$cargo_dir" "$bin_dir"; do
  if [ -d "\$_kei_path" ]; then
    case ":\$PATH:" in
      *":\$_kei_path:"*) ;;
      *) export PATH="\$_kei_path:\$PATH" ;;
    esac
  fi
done
unset _kei_path
$_PATHWAY_END
EOF
}

# Render the fish-shell variant (fish_add_path is idempotent natively).
# Same two paths as posix variant.
_render_fish_block() {
  local cargo_dir="$HOME_DIR/.cargo/bin"
  local bin_dir="$HOME_DIR/.claude/bin"
  cat <<EOF
$_PATHWAY_BEGIN
# Added by KeiSeiKit install.sh — Rust primitives + claude/bin on PATH.
# Remove this block (with markers) to opt out.
for _kei_path in "$cargo_dir" "$bin_dir"
    if test -d "\$_kei_path"
        fish_add_path -p "\$_kei_path"
    end
end
$_PATHWAY_END
EOF
}

# Refuse to operate on a symlink. Replacing a symlink with `mv tmp link`
# destroys the link and writes a regular file in its place — catastrophic
# for users whose rc files are managed by a dotfile repo. MISS-10 fix:
# bail with a clear message asking the user to add the block manually
# inside the symlink target (i.e. their dotfile repo).
_refuse_if_symlink() {
  local file="$1"
  if [ -L "$file" ]; then
    local target
    target="$(readlink "$file" 2>/dev/null || printf '?\n')"
    warn "$file is a symlink (-> $target); refusing to replace."
    warn "  Add the kei-substrate block manually inside your dotfile target,"
    warn "  or unlink \"$file\" and re-run install."
    return 1
  fi
  return 0
}

# Strip an existing kei-substrate block from a file. Idempotent: zero blocks
# removed is fine. Uses awk for portability (no GNU-sed inplace).
_strip_block_from_file() {
  local file="$1"
  [ -f "$file" ] || return 0
  _refuse_if_symlink "$file" || return 1
  local tmp
  tmp="$(mktemp "$file.XXXXXX")"
  if awk -v b="$_PATHWAY_BEGIN" -v e="$_PATHWAY_END" '
    $0 == b { skip=1; next }
    $0 == e { skip=0; next }
    !skip { print }
  ' "$file" > "$tmp"; then
    mv "$tmp" "$file"
  else
    rm -f "$tmp"
    return 1
  fi
}

# Append a block to an rc file. Strips any prior copy first → idempotent.
_install_block_into_file() {
  local file="$1" block="$2"
  mkdir -p "$(dirname "$file")"
  [ -L "$file" ] && { _refuse_if_symlink "$file"; return 1; }
  [ -f "$file" ] || : > "$file"
  _strip_block_from_file "$file" || return 1
  # ensure trailing newline before append
  if [ -s "$file" ] && [ "$(tail -c 1 "$file" | xxd -p 2>/dev/null)" != "0a" ]; then
    printf '\n' >> "$file"
  fi
  printf '%s\n' "$block" >> "$file"
}

pathway_install_bashrc() {
  local rc="$HOME_DIR/.bashrc"
  local block
  block="$(_render_posix_block)"
  _install_block_into_file "$rc" "$block" || return 1
  say "  wired PATH in $rc"
}

pathway_install_zshrc() {
  local rc="$HOME_DIR/.zshrc"
  local block
  block="$(_render_posix_block)"
  _install_block_into_file "$rc" "$block" || return 1
  say "  wired PATH in $rc"
}

pathway_install_fish_config() {
  local rc="$HOME_DIR/.config/fish/config.fish"
  local block
  block="$(_render_fish_block)"
  _install_block_into_file "$rc" "$block" || return 1
  say "  wired PATH in $rc"
}

# Public dispatcher. Honors $WITH_PATHWAY=1 (forced on) / $NO_PATHWAY=1
# (forced off, no-op). Default: install when interactive TTY OR
# $WITH_PATHWAY=1 was passed.
pathway_install() {
  if [ "${NO_PATHWAY:-0}" = "1" ]; then
    say "PATH wiring skipped (--no-pathway)"
    return 0
  fi
  local sh
  sh="$(pathway_detect_shell)"
  say "wiring PATH (~/.cargo/bin + ~/.claude/bin) for shell=$sh"
  case "$sh" in
    bash) pathway_install_bashrc ;;
    zsh)  pathway_install_zshrc ;;
    fish) pathway_install_fish_config ;;
    *)
      warn "unknown shell ($sh); add to your rc manually:"
      warn "  export PATH=\"\$HOME/.cargo/bin:\$HOME/.claude/bin:\$PATH\""
      return 0
      ;;
  esac
  say "  open a new terminal or run: source ~/.${sh}rc"
}

# Remove the block from every supported rc, regardless of detected shell.
pathway_uninstall() {
  local rc
  for rc in "$HOME_DIR/.bashrc" "$HOME_DIR/.zshrc" "$HOME_DIR/.config/fish/config.fish"; do
    [ -f "$rc" ] || continue
    _strip_block_from_file "$rc"
    say "  removed PATH block from $rc"
  done
}
