---
name: fix-issue
description: Use when fixing a GitHub issue — reproduce, trace root cause, fix, add regression test
disable-model-invocation: true
arguments:
  - name: issue
    description: GitHub issue number or URL
    required: true
---

# Fix Issue Workflow

## Step 1: Load Issue Context
- Fetch issue details: `gh issue view $issue --json title,body,labels,comments`
- Read all comments for additional context
- Identify: expected behavior, actual behavior, reproduction steps

## Step 2: Reproduce
- Set up reproduction environment based on issue description
- Write a failing test that captures the bug (TDD approach)
- If cannot reproduce — ask user for more context, do NOT guess

## Step 3: Trace Root Cause
- Use Grep/Glob to find relevant code paths
- Trace execution flow from entry point to failure
- Identify the ROOT CAUSE, not symptoms
- Check DECISIONS.md for related past decisions

## Step 4: Checkpoint
- `git commit` current state: `checkpoint: before fix-issue #$issue`

## Step 5: Fix
- Fix at the root cause level, NOT overlay/patch
- Follow Constructor Pattern: if fix grows file >200 lines, decompose
- Ensure fix doesn't break other functionality

## Step 6: Verify
- Run the failing test — must pass now
- Run full test suite — no regressions
- If test suite doesn't exist, create at minimum:
  - Test for the specific bug (regression test)
  - Test for the happy path of affected function

## Step 7: Commit
- `fix: <description of what was fixed> (closes #$issue)`
- Update TODO.md if issue was tracked there
