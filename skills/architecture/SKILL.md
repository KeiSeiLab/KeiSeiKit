---
name: architecture
description: Multi-agent architectural analysis following the Holographic Development Chain. Use when user asks to design architecture, plan implementation, analyze system structure, or says "архитектура", "план", "спроектируй", "как лучше сделать".
argument-hint: <system or feature description>
---

# Holographic Architecture Analysis

Task: $ARGUMENTS

## Phase 0: Load Architecture Rules

BEFORE any analysis, read the rules database:
1. `~/.claude/skills/architecture-rules/references/checklist.md` — pre-implementation checklist
2. `~/.claude/skills/architecture-rules/references/patterns.md` — pattern selection guide
3. `~/.claude/skills/architecture-rules/references/antipatterns.md` — what to detect and avoid
4. `~/.claude/skills/architecture-rules/references/stack-compat.md` — technology compatibility
5. `~/.claude/skills/architecture-rules/references/duplication.md` — duplication thresholds

Apply these rules throughout ALL subsequent phases.

## Phase 1: Base Agents (Parallel)

Launch 3 agents simultaneously:

### Agent 1: Architect
Analyze structure, dependencies, patterns. If codebase exists — use Explore agent to map:
- Directory structure and module boundaries
- Data flow and control flow
- Existing patterns and conventions
- Integration points and APIs
- Return list of 5-10 key files to read

### Agent 2: Critic
Search for problems, vulnerabilities, anti-patterns:
- OWASP top 10 relevant issues
- Performance bottlenecks
- Scalability concerns
- Technical debt indicators
- Over-engineering or missing abstractions
- Return specific file:line references

### Agent 3: Validator
Verify facts and check for hallucinations:
- Do the proposed technologies actually work together?
- Are version compatibilities correct?
- Do the APIs referenced actually exist?
- Cross-check with official documentation
- Return verified facts vs unverified assumptions

## Phase 2: Arbitration

After all 3 agents return:
1. Read all key files identified by Architect
2. Compare findings — where do agents agree/disagree?
3. Resolve conflicts — pick best path with reasoning
4. Rate confidence: X% for the chosen approach

Present as:
```
## Arbitration Result

### Agents Agree On:
- [Point] — all 3 confirm

### Disagreements:
- [Point] — Architect says X, Critic says Y
- Resolution: [chosen path + why]

### Confidence: X%
```

## Phase 3: Design Options (2-3 Parallel Agents)

Launch architecture design agents with different philosophies:

### Option A: Minimal Changes
Smallest possible change, maximum reuse of existing code

### Option B: Clean Architecture
Optimal maintainability, elegant abstractions, proper separation

### Option C: Pragmatic Balance
Speed + quality trade-off, ship fast but don't accumulate debt

## Phase 4: Present & Recommend

For each option show:
- Component diagram (text-based)
- Files to create/modify
- Data flow
- Implementation phases (ordered steps)
- Estimated complexity
- Trade-offs

**Your recommendation** with reasoning and confidence %.

## Phase 5: Implementation Blueprint

After user picks an option:
1. Detailed file-by-file changes
2. Build sequence (what first, what depends on what)
3. Test strategy
4. Checkpoint plan (when to verify)

## Rules
- Architecture FIRST, code SECOND
- Find root cause, not symptoms
- Simple > clever
- 3 repeating lines > premature abstraction
- Dead code = delete, don't comment
- Each logic in separate file — defragmentation
- Checkpoints after each phase
