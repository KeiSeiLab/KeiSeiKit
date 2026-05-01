---
name: session-budget
description: Use when tracking token/cost budget per session — monitors API usage and compute costs
arguments:
  - name: action
    description: "Action: start, check, report, set-limit"
    required: true
  - name: limit
    description: "Budget limit in USD (for set-limit action)"
    required: false
---

# Session Budget Tracker

## Actions

### start
Initialize budget tracking for this session:
- Note session start time
- Record any known API costs (Modal, fal.ai, Apify, etc.)
- Set default limit: $5 unless overridden

### check
Before any paid API call:
- Calculate estimated cost of the operation
- Compare against remaining budget
- If over budget: WARN user and ask for confirmation
- If under budget: proceed and log the cost

### report
Generate session cost report:
- List all API calls made and their costs
- Total spend this session
- Remaining budget
- Comparison to previous sessions (if available in memory)

### set-limit
Set custom budget limit for the session:
- Store limit value
- Warn at 80% usage
- Block at 100% unless user overrides

## Cost Reference (approximate)
| Service | Unit | Cost |
|---------|------|------|
| Modal GPU (A10G) | per hour | $1.10 |
| Modal GPU (A100) | per hour | $3.73 |
| fal.ai Flux Pro | per image | $0.05 |
| fal.ai Kling | per video | $0.30-0.90 |
| Apify Actor | per run | varies |
| ElevenLabs TTS | per 1K chars | $0.30 |

## Integration
- Works with api-cost-guard rule (validates before compute)
- Saves session summary to context-store at session end
