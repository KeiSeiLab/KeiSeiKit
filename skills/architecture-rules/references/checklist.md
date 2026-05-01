# Pre-Implementation Architecture Checklist

## Run BEFORE writing any code

### 1. Requirements Clarity
- [ ] What EXACTLY should this do? (not vague "improve X")
- [ ] What are the inputs and outputs?
- [ ] What are the edge cases?
- [ ] What happens on failure?
- [ ] What are the performance requirements?

### 2. Existing Code Check
- [ ] Does similar functionality already exist in codebase?
- [ ] Can existing code be extended instead of writing new?
- [ ] What patterns does the codebase already use?
- [ ] What conventions must be followed?

### 3. Duplication Prevention
- [ ] Search codebase for similar patterns (grep, find)
- [ ] Check shared/ and lib/ for reusable utilities
- [ ] If creating new utility — will 2+ modules use it?
- [ ] If yes → put in shared/. If no → keep local

### 4. Dependency Direction
- [ ] Draw dependency arrows — do they point inward (clean)?
- [ ] No circular dependencies?
- [ ] Upper layers don't know about lower layer implementation?
- [ ] Domain logic has ZERO framework imports?

### 5. Pattern Selection
- [ ] Consult patterns.md — which pattern fits?
- [ ] Is the pattern justified by complexity? (don't over-engineer)
- [ ] Does the codebase already use this pattern?
- [ ] If new pattern — is it worth the learning curve for team?

### 6. Stack Compatibility
- [ ] Check stack-compat.md for version constraints
- [ ] No anti-compatible combinations?
- [ ] New dependency needed? Check bundle size, maintenance status, license
- [ ] Prefer stdlib/existing deps over new packages

### 7. Security
- [ ] Input validation at system boundaries
- [ ] SQL/NoSQL injection prevention (parameterized queries)
- [ ] XSS prevention (sanitize output)
- [ ] Auth/authz checks on all endpoints
- [ ] No secrets in code (use env vars)
- [ ] No internal IDs exposed in public APIs

### 8. Data Flow
- [ ] Where does data enter the system?
- [ ] How is it validated and transformed?
- [ ] Where is it stored?
- [ ] How is it retrieved and presented?
- [ ] What happens to stale/expired data?

### 9. Error Handling Strategy
- [ ] What errors are expected? (user input, network, DB)
- [ ] What errors are unexpected? (bugs)
- [ ] Error response format consistent with existing API?
- [ ] Errors logged with enough context to debug?
- [ ] No swallowed errors (empty catch blocks)

### 10. Testability
- [ ] Can each component be tested in isolation?
- [ ] Dependencies injectable (not hardcoded)?
- [ ] Side effects isolated (DB, network, filesystem)?
- [ ] Happy path + edge cases + error cases covered?

## Run AFTER writing code

### 11. Post-Implementation Review
- [ ] Re-check duplication — did I create any new duplicates?
- [ ] Anti-pattern scan (see antipatterns.md)
- [ ] All functions < 50 lines? (extract if not)
- [ ] All files < 300 lines? (split if not)
- [ ] Cyclomatic complexity < 10 per function?
- [ ] No dead code left behind
- [ ] No commented-out code
- [ ] No TODO/FIXME without ticket reference

### 12. Performance Basics
- [ ] No N+1 queries (check ORM calls in loops)
- [ ] Indexes on queried columns
- [ ] No blocking I/O in async paths
- [ ] Large lists paginated
- [ ] Heavy operations use queue (not request handler)

### 13. Deploy Readiness
- [ ] Check error-patterns.json for recurring issues
- [ ] Environment variables documented
- [ ] Database migrations created and tested
- [ ] Rollback plan if deploy fails
- [ ] Health check endpoint works

## Complexity Budget

| Metric | Green | Yellow | Red |
|--------|-------|--------|-----|
| Function length | < 30 lines | 30-50 lines | > 50 lines |
| File length | < 200 lines | 200-300 lines | > 300 lines |
| Function params | 1-3 | 4-5 | > 5 (use object) |
| Nesting depth | 1-2 | 3 | > 3 (extract) |
| Cyclomatic complexity | 1-5 | 6-10 | > 10 (simplify) |
| Dependencies per module | 1-5 | 6-8 | > 8 (split module) |
| Test coverage | > 80% | 60-80% | < 60% |
