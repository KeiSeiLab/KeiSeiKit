---
name: pr-review
description: Use when reviewing a pull request — Constructor Pattern awareness, security, SSOT check
arguments:
  - name: pr
    description: PR number or URL
    required: true
---

# PR Review Workflow

## Step 1: Load PR Context
- `gh pr view $pr --json title,body,files,additions,deletions,commits`
- `gh pr diff $pr`
- Read PR description and linked issues

## Step 2: High-Level Assessment
- What does this PR do? (feature, fix, refactor, docs)
- Is the scope reasonable? (single responsibility)
- Does the PR description explain WHY?

## Step 3: Code Review Checklist

### Constructor Pattern
- [ ] 1 file = 1 class = 1 responsibility
- [ ] No files >200 lines (new or modified)
- [ ] No functions >30 lines
- [ ] No overlays/patches (fixes at root cause)
- [ ] Self-extension pattern followed for new code

### SSOT
- [ ] No duplicated types, constants, or logic
- [ ] Types defined before implementation
- [ ] Single source for routes, enums, config

### Security
- [ ] No hardcoded secrets (API keys, tokens, passwords)
- [ ] No SQL injection vectors
- [ ] No XSS vectors in templates/JSX
- [ ] Input validation at system boundaries
- [ ] No sensitive data in logs

### Tests
- [ ] New code has tests
- [ ] Edge cases covered
- [ ] No test pollution (shared state between tests)

### Quality
- [ ] Error handling at boundaries
- [ ] No dead code or commented-out code
- [ ] Imports clean (no unused)
- [ ] Naming is clear and consistent

## Step 4: Write Review
- For each issue found: file, line, severity (blocker/warning/nit), suggestion
- Blockers MUST be fixed before merge
- Group by file, sorted by severity
- End with overall assessment: approve / request changes / comment
