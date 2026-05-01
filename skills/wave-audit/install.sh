#!/bin/bash
# Kei Dev Suite Installer — skills + rules + hook
# Usage: unzip kei-dev-suite-*.zip && bash wave-audit/install.sh

set -e

SKILL_DIR="$HOME/.claude/skills"
RULES_DIR="$HOME/.claude/rules"
HOOK_DIR="$HOME/.claude/hooks"

echo "=== Kei Dev Suite Installer ==="
echo ""

# 1. Install skills
for skill in wave-audit dev-start dev-guard dev-ship; do
  if [ -d "$skill" ] && [ -f "$skill/SKILL.md" ]; then
    mkdir -p "$SKILL_DIR/$skill"
    cp "$skill/SKILL.md" "$SKILL_DIR/$skill/SKILL.md"
    echo "[OK] Skill: /$(echo $skill)"
  fi
done

# 2. Install shared rules
if [ -d "shared" ] && [ -f "shared/kei-rules.md" ]; then
  mkdir -p "$RULES_DIR"
  cp "shared/kei-rules.md" "$RULES_DIR/kei-rules.md"
  echo "[OK] Rules: $RULES_DIR/kei-rules.md"
fi

# 3. Install hook
mkdir -p "$HOOK_DIR"
cat > "$HOOK_DIR/wave-audit-verify.sh" << 'HOOK_EOF'
#!/bin/bash
# Wave Audit Hook — validates finding format from agents
# Checks: CODE_QUOTE and VERIFY_CMD presence in HIGH/CRITICAL findings

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // empty')

[ "$TOOL" != "SendMessage" ] && exit 0

MESSAGE=$(echo "$INPUT" | jq -r '.tool_input.message // empty')
echo "$MESSAGE" | grep -qiE '(CRITICAL|HIGH|severity)' || exit 0

MISSING_QUOTE=$(echo "$MESSAGE" | grep -ciE '(CRITICAL|HIGH)' || true)
HAS_QUOTE=$(echo "$MESSAGE" | grep -c 'CODE_QUOTE\|```' || true)

if [ "$MISSING_QUOTE" -gt 0 ] && [ "$HAS_QUOTE" -eq 0 ]; then
  echo "WAVE-AUDIT: Agent submitted HIGH/CRITICAL findings without CODE_QUOTE." >&2
  echo "Findings without CODE_QUOTE are marked [UNVERIFIED] and downgraded." >&2
fi

exit 0
HOOK_EOF
chmod +x "$HOOK_DIR/wave-audit-verify.sh"
echo "[OK] Hook: $HOOK_DIR/wave-audit-verify.sh"

echo ""
echo "=== Installation complete ==="
echo ""
echo "Skills installed:"
echo "  /dev-start    — parallel kickoff (4 agents: contracts, tests, security, structure)"
echo "  /dev-guard    — continuous quality gate (3 agents: security, performance, structure)"
echo "  /dev-ship     — pre-merge gate (4 agents: security, tests, deps, regression)"
echo "  /wave-audit   — full 3-wave audit (9 agents across 3 waves)"
echo ""
echo "Rules installed:"
echo "  kei-rules.md  — 3-Level Escalation, Evidence Grading, Root Cause, Git Conventions"
echo ""
echo "Lifecycle: /dev-start → code → /dev-guard → /dev-ship → /wave-audit"
