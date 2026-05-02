---
name: test-gen
description: Use when generating tests for untested code — happy path, edge cases, error handling
arguments:
  - name: target
    description: File path or function name to test
    required: true
---

## When to use

- Generating unit tests for a specific function or file (happy path, edge cases, error handling).
- Writing tests first (TDD) before implementing new functionality.
- Use `/test-gen` for per-function unit tests; use `/test-matrix` for project-wide testing strategy.

> **Complements `/test-matrix`.** `/test-gen` owns per-function unit tests
> (happy / edge / error). `/test-matrix` owns project-wide testing strategy
> (fuzz / property / load / e2e / mutation) and CI wiring. Use `/test-gen`
> for a specific function, `/test-matrix` at project kickoff or when
> coverage gaps span paradigms. See `skills/test-matrix/SKILL.md`.

# Test Generation Workflow

## Step 1: Analyze Target
- Read the target file/function completely
- Identify: inputs, outputs, side effects, dependencies, error paths
- Check existing test files — don't duplicate

## Step 2: Detect Framework
- Auto-detect test framework from project:
  - Python: pytest (conftest.py, pytest.ini) or unittest
  - JS/TS: jest (jest.config), vitest (vitest.config), mocha
  - Flutter: flutter_test
  - Go: built-in testing
- Match existing test file naming: `test_*.py`, `*.test.ts`, `*_test.dart`, `*_test.go`

## Step 3: Plan Test Cases
Categories (in order of priority):
1. **Happy path** — normal expected usage (2-3 cases)
2. **Edge cases** — boundaries, empty inputs, max values (2-3 cases)
3. **Error handling** — invalid inputs, missing data, network errors (1-2 cases)
4. **Integration** — only if function interacts with external services (1 case with mock)

## Step 4: Write Tests
- Follow existing test patterns in the project
- Use descriptive test names: `test_<function>_<scenario>_<expected>`
- One assertion per test (prefer)
- Mock external dependencies, not internal logic
- Use fixtures/factories from existing test infrastructure

## Step 5: Run & Verify
- Run new tests — all must pass
- Run full test suite — no regressions
- Check coverage delta if coverage tool available

## Step 6: Commit
- `test: add tests for <target>`
