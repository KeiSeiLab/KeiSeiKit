#!/bin/bash
# Hard block on python/python3/python2 invocations in Bash tool.
# RULE 0.2 (Rust First) — Python requires explicit architectural reason.
# Claude кroнически нарушает RULE 0.2 inline-вызовами python3 для мелких расчётов.
# Этот хук форсирует: каждый python-вызов = отдельный approval через интерфейс.
#
# How to approve: user may add a one-off permission via Claude Code's
# "allow this command" prompt, OR prefix with RULE02_BYPASS=1.
# No silent bypass; no shell aliases.
#
# Installed: 2026-04-21 after repeated inline python3 abuse
# (Pantheon CF arithmetic, Gauss-Kuzmin — could have been Rust or awk+bc).

INPUT=$(cat)
CMD=$(printf '%s' "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)

[ -z "$CMD" ] && exit 0

# Explicit bypass env var (used rarely, visible in command text — not silent)
if echo "$CMD" | grep -qE 'RULE02_BYPASS=1'; then
  exit 0
fi

# Match python invocations at token boundaries.
# Catches: python, python3, python2, python3.12, /usr/bin/python3, .venv/bin/python
# Also heredoc-only patterns: "python3 -c", "python3 << EOF", "python3 script.py"
# Also: uv run python, poetry run python, pipx run python
if echo "$CMD" | grep -qE '(^|[[:space:]/"=(|&;`])(python|python2|python3)([0-9]?\.[0-9]+)?([[:space:]]|$)'; then
  cat >&2 <<'EOF'
═══════════════════════════════════════════════════════════════════
  BLOCKED — Python invocation requires explicit approval (RULE 0.2).
═══════════════════════════════════════════════════════════════════

RULE 0.2 Rust First:
  Python не разрешается по умолчанию. Для "одноразовых расчётов"
  Claude должен предпочитать: Rust (cargo script), awk/bc/dc, jq,
  node, или существующий Rust-код в проекте.

Approved alternatives BEFORE retrying:
  • Rust one-shot:  write my-project/examples/foo.rs, cargo run
  • Shell math:     awk 'BEGIN{printf "%.10f\n", ...}'
  • Arbitrary prec: bc -l / dc
  • JSON munge:     jq '.path.to.value'
  • Project Rust:   find reusable crate function, call via cargo test

If Python is genuinely necessary (existing .py tool, library binding
that only exists in Python, one of the RULE 0.2 exceptions 1-7):
  1. State the RULE 0.2 exception number in chat.
  2. Re-run the command with prefix:  RULE02_BYPASS=1 python3 ...
  3. User will see the bypass marker in the command and can deny.

This hook installed 2026-04-21 by user request after repeated
repeated inline python3 use where Rust would suffice.
═══════════════════════════════════════════════════════════════════
EOF
  exit 2
fi

exit 0
