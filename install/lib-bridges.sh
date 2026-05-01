# shellcheck shell=bash
# lib-bridges.sh — copy bridge templates + optional --with-bridges render into $PWD.
#
# Templates are SSoT from the kit (always refreshed). The render step is
# skipped when invoked inside the KeiSeiKit repo itself.
#
# Requires: say / warn from lib-log.sh.
# Requires: backup_dir from lib-backup.sh.
# Reads globals: $KIT_DIR, $AGENTS_DIR.

install_bridges() {
  [ -d "$KIT_DIR/_bridges" ] || return 0
  say "copying bridge templates -> $AGENTS_DIR/_bridges/"
  mkdir -p "$AGENTS_DIR/_bridges"
  backup_dir "$AGENTS_DIR/_bridges"
  cp -f "$KIT_DIR/_bridges/"*.tmpl "$AGENTS_DIR/_bridges/"
  cp -f "$KIT_DIR/_bridges/README.md" "$AGENTS_DIR/_bridges/"
  cp -f "$KIT_DIR/_bridges/emit.sh" "$AGENTS_DIR/_bridges/emit.sh"
  chmod +x "$AGENTS_DIR/_bridges/emit.sh"
}

# Render cross-tool bridges into $PWD via the kit's emit.sh script.
# No-op when the caller is sitting inside the KeiSeiKit repo itself.
render_bridges() {
  if [[ -f "./install.sh" && -d "./_bridges" ]]; then
    warn "not generating bridges — you are in the KeiSeiKit repo, not a project directory"
    return 0
  fi
  say "rendering cross-tool bridges into $PWD"
  "$KIT_DIR/_bridges/emit.sh" "$PWD"
}
