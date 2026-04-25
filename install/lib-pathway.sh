# shellcheck shell=bash
# lib-pathway.sh — substrate PATH wiring (Wave 39).
#
# Adds $AGENTS_DIR/_primitives/_rust/target/release/ to the user's shell
# PATH so kei-fork / kei-ledger / kei-spawn / kei-agent-runtime are
# directly invocable after install. Per-shell idempotent block guarded by
# marker fences (rbenv/pyenv style) so repeated runs never duplicate.
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
_render_posix_block() {
  local target_dir="$1"
  cat <<EOF
$_PATHWAY_BEGIN
# Added by KeiSeiKit install.sh — substrate primitives on PATH.
# Remove this block (with markers) to opt out.
if [ -d "$target_dir" ]; then
  case ":\$PATH:" in
    *":$target_dir:"*) ;;
    *) export PATH="$target_dir:\$PATH" ;;
  esac
fi
$_PATHWAY_END
EOF
}

# Render the fish-shell variant (fish_add_path is idempotent natively).
_render_fish_block() {
  local target_dir="$1"
  cat <<EOF
$_PATHWAY_BEGIN
# Added by KeiSeiKit install.sh — substrate primitives on PATH.
# Remove this block (with markers) to opt out.
if test -d "$target_dir"
    fish_add_path -p "$target_dir"
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
  local target_dir="$1" rc="$HOME_DIR/.bashrc"
  local block
  block="$(_render_posix_block "$target_dir")"
  _install_block_into_file "$rc" "$block" || return 1
  say "  wired PATH in $rc"
}

pathway_install_zshrc() {
  local target_dir="$1" rc="$HOME_DIR/.zshrc"
  local block
  block="$(_render_posix_block "$target_dir")"
  _install_block_into_file "$rc" "$block" || return 1
  say "  wired PATH in $rc"
}

pathway_install_fish_config() {
  local target_dir="$1" rc="$HOME_DIR/.config/fish/config.fish"
  local block
  block="$(_render_fish_block "$target_dir")"
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
  local target_dir="$AGENTS_DIR/_primitives/_rust/target/release"
  local sh
  sh="$(pathway_detect_shell)"
  say "wiring substrate PATH for shell=$sh"
  case "$sh" in
    bash) pathway_install_bashrc "$target_dir" ;;
    zsh)  pathway_install_zshrc  "$target_dir" ;;
    fish) pathway_install_fish_config "$target_dir" ;;
    *)
      warn "unknown shell ($sh); add this to your rc manually:"
      warn "  export PATH=\"$target_dir:\$PATH\""
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
