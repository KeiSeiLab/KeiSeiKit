#!/bin/bash
# Pre-deploy hook: check error-patterns.json for recurring/critical issues
# Exit 0 = allow, Exit 2 = block

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | python3 -c "import json,sys; print(json.load(sys.stdin).get('tool_input',{}).get('command',''))" 2>/dev/null)

# Only check deploy-related commands
if echo "$COMMAND" | grep -qiE 'docker.*(build|up|push)|deploy|rsync.*server|ssh.*docker'; then
  PATTERNS_FILE="/Users/denis/projects/ai machine learning/error-patterns.json"

  if [ -f "$PATTERNS_FILE" ]; then
    # Find critical/recurring patterns
    WARNINGS=$(python3 -c "
import json
try:
    with open('$PATTERNS_FILE') as f:
        patterns = json.load(f)
    critical = [p for p in patterns if p.get('severity') == 'critical' or p.get('frequency') == 'recurring']
    if critical:
        print('PRE-DEPLOY WARNING - Check these known patterns:')
        for p in critical[:5]:
            print(f\"  [{p.get('severity','?')}] {p.get('id','?')}: {p.get('name','?')}\")
            print(f\"    Trigger: {p.get('trigger','?')}\")
        print()
        print('Verify these do not apply to current deploy.')
except:
    pass
" 2>/dev/null)

    if [ -n "$WARNINGS" ]; then
      echo "$WARNINGS" >&2
      # Warn but don't block (exit 0)
      exit 0
    fi
  fi
fi

exit 0
