#!/usr/bin/env bash
# Guard against destructive actions that could damage running experiments.
# Returns JSON with block decision if destructive command detected.

CMD=$(jq -r '.tool_input.command // empty')

# Check if command contains destructive patterns
if echo "$CMD" | grep -qEi '(^|\s|sudo\s+)(pkill|kill|killall)\b|rm\s+-rf?\b|reboot|shutdown|systemctl\s+(stop|restart)|docker\s+(rm|stop|kill)|drop\s+table|truncate|git\s+reset\s+--hard|git\s+clean\s+-f'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"ask","permissionDecisionReason":"⚠️ Destructive action detected. Verify this will not damage a running experiment or data collection."}}'
fi
