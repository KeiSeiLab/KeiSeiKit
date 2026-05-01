---
name: quick-api
description: Use when scaffolding a new API endpoint — types, test, handler, validation, route, docs
disable-model-invocation: true
arguments:
  - name: endpoint
    description: "Endpoint description, e.g. 'POST /api/users — create user'"
    required: true
---

# Quick API Scaffold Workflow

## Step 1: Detect Framework
- Auto-detect from project:
  - Python: FastAPI (main.py, routers/), Flask, Django
  - JS/TS: Next.js (app/api/), Express, Hono
  - Go: net/http, gin, echo
- Find existing endpoint examples to match patterns

## Step 2: API Contract (Types First)
- Define request/response types BEFORE implementation
- Place types where project keeps them (types/, schemas/, models/)
- Include validation rules in type definitions
- Document in OpenAPI format if project uses it

## Step 3: Write Test First
- Create test file matching project conventions
- Test cases:
  - Happy path (valid request → expected response)
  - Validation error (invalid input → 400/422)
  - Auth error if endpoint requires auth (no token → 401)
  - Not found if applicable (→ 404)

## Step 4: Checkpoint
- `checkpoint: before quick-api $endpoint`

## Step 5: Implement Handler
- Follow existing handler patterns exactly
- Input validation using project's validation approach
- Error handling matching existing patterns
- Keep handler thin — business logic in service layer if project uses one

## Step 6: Register Route
- Add route in the project's routing configuration
- Match URL pattern, middleware, and auth guards from existing routes

## Step 7: Verify
- Run tests — all pass
- Test manually with curl if appropriate
- Check that OpenAPI/Swagger docs update if auto-generated

## Step 8: Commit
- `feat: add $endpoint endpoint`
