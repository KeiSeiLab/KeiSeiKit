---
name: refactor
description: Use when refactoring code while preserving behavior — checkpoint, extract, test, audit
arguments:
  - name: target
    description: File or module to refactor
    required: true
  - name: goal
    description: "What the refactoring should achieve"
    required: false
---

# Refactor Workflow

## Step 1: Understand Current State
- Read target file(s) completely
- Identify existing tests for the target
- Run existing tests — baseline must pass
- Document current behavior (inputs → outputs)

## Step 2: Plan Refactoring
- Identify what violates Constructor Pattern:
  - File >200 lines → decompose
  - Function >30 lines → extract
  - Multiple responsibilities → split
  - Duplicated code → single source
  - Overlay/patches → root formula
- List concrete changes BEFORE making them

## Step 3: Checkpoint
- `checkpoint: before refactor $target`

## Step 4: Refactor (Incremental)
- ONE structural change at a time
- After each change: run tests
- If test breaks → fix immediately or revert
- Preserve all public interfaces unless explicitly changing them

## Step 5: Audit
- Check: does refactored code follow Constructor Pattern?
- Check: no overlay, no dead code, no orphaned imports
- Check: SSOT — no duplicated logic
- Check: file sizes within limits

## Step 6: Final Verification
- Run full test suite — all pass
- Diff review: no accidental behavior changes
- No new dependencies added without reason

## Step 7: Commit
- `refactor: <what was restructured and why>`
