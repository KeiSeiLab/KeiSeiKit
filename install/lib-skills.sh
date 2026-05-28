set -e
# shellcheck shell=bash
# lib-skills.sh — skill directory copy loop.
#
# Skills live in $KIT_DIR/skills/<name>/ and are synced into
# $SKILLS_DIR/<name>/ on every install.
#
# Requires: say from lib-log.sh.
# Requires: backup_dir from lib-backup.sh.
# Reads globals: $KIT_DIR, $SKILLS_DIR.

install_skills() {
  [ -d "$KIT_DIR/skills" ] || return 0
  say "copying skills"
  backup_dir "$SKILLS_DIR"
  local skill_dir skill_name
  for skill_dir in "$KIT_DIR/skills/"*/; do
    [ -d "$skill_dir" ] || continue
    skill_name="$(basename "$skill_dir")"
    mkdir -p "$SKILLS_DIR/$skill_name"
    cp -rf "$skill_dir"* "$SKILLS_DIR/$skill_name/" 2>/dev/null || true
    say "  -> $skill_name"
  done
}
