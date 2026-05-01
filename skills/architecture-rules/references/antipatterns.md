# Anti-Patterns — What to Detect and Fix

## Code-Level Anti-Patterns

### God Object / God Class
- **Signal**: Class with 500+ lines, 10+ methods, touches many concerns
- **Detection**: `wc -l` on class files, count public methods
- **Fix**: Extract into focused classes by responsibility (SRP)

### Spaghetti Code
- **Signal**: No clear flow, deeply nested callbacks, goto-like jumps
- **Detection**: Cyclomatic complexity > 10 per function, nesting > 4 levels
- **Fix**: Extract functions, use early returns, flatten with async/await

### Golden Hammer
- **Signal**: Same pattern/technology used for everything regardless of fit
- **Detection**: "We always use X" without considering alternatives
- **Fix**: Choose tool for the job, not job for the tool

### Copy-Paste Programming
- **Signal**: Nearly identical code blocks (>10 lines similar)
- **Detection**: See duplication.md for detection rules
- **Fix**: Extract common logic, use strategy pattern or composition

### Premature Optimization
- **Signal**: Complex caching, custom data structures, bit manipulation without benchmarks
- **Detection**: Optimization without profiling evidence
- **Fix**: Profile first, optimize bottlenecks, keep it simple

### Premature Abstraction
- **Signal**: Abstract class with single implementation, interfaces used once
- **Detection**: Base class + single subclass, unused generic parameters
- **Rule**: 3 concrete uses → then abstract. Not before.

### Stringly Typed
- **Signal**: Magic strings everywhere: `type === "admin"`, `status === "pending"`
- **Detection**: String literals in comparisons, switch on strings
- **Fix**: Enums, union types, constants

### Boolean Blindness
- **Signal**: `doThing(true, false, true)` — what do these mean?
- **Detection**: Functions with 2+ boolean params
- **Fix**: Use options object: `doThing({ verbose: true, dryRun: false })`

### Callback Hell
- **Signal**: Nested callbacks 3+ levels deep
- **Fix**: async/await, Promise chains, or event-driven architecture

## Architecture-Level Anti-Patterns

### Big Ball of Mud
- **Signal**: No discernible structure, everything depends on everything
- **Detection**: Any module can import from any other module
- **Fix**: Define module boundaries, enforce dependency direction

### Distributed Monolith
- **Signal**: Microservices that MUST deploy together, shared DB
- **Detection**: Service A can't function without Service B up
- **Fix**: Define clear service boundaries, separate data stores, use events

### Anemic Domain Model
- **Signal**: Classes with only getters/setters, logic in "service" classes
- **Detection**: Models have no methods, services have all behavior
- **Fix**: Move behavior into domain objects where it belongs

### Circular Dependencies
- **Signal**: A imports B imports C imports A
- **Detection**: `madge --circular` for JS/TS, module dependency graph
- **Fix**: Extract shared interface, use events, dependency inversion

### Leaky Abstraction
- **Signal**: Implementation details leak through interface (DB errors in API response)
- **Detection**: Lower layer concepts visible in upper layers
- **Fix**: Map errors at boundaries, abstract return types

### Shotgun Surgery
- **Signal**: One change requires editing 10+ files across modules
- **Detection**: Git: "one logical change = commits in many directories"
- **Fix**: Consolidate related logic, improve cohesion

### Feature Envy
- **Signal**: Method uses more data from another class than its own
- **Detection**: Count external vs internal field accesses
- **Fix**: Move method to the class whose data it mostly uses

### Lava Flow
- **Signal**: Dead code everyone is afraid to remove ("might need it")
- **Detection**: Code not executed in tests, no callers found in codebase
- **Fix**: DELETE IT. Git has history if you need it back.

## Database Anti-Patterns

### N+1 Queries
- **Signal**: Loop that queries DB once per item
- **Detection**: Query count correlates with result count
- **Fix**: JOIN, IN clause, or ORM eager loading

### SELECT *
- **Signal**: Fetching all columns when only 2-3 needed
- **Fix**: Explicit column selection, DTO projection

### Missing Indexes
- **Signal**: Slow queries on WHERE/JOIN/ORDER BY columns
- **Detection**: EXPLAIN ANALYZE, query execution time > 100ms
- **Fix**: Add indexes on frequently queried columns

### Storing JSON in SQL
- **Signal**: JSONB columns queried with complex paths
- **When OK**: Truly schema-less data, write-once-read-many
- **When NOT OK**: Regular queries on JSON fields → normalize to columns

### God Table
- **Signal**: Table with 30+ columns, nullable columns everywhere
- **Fix**: Normalize, extract related groups into separate tables

## API Anti-Patterns

### Chatty API
- **Signal**: Client needs 5+ requests to render one page
- **Fix**: Aggregate endpoints, GraphQL, BFF pattern

### Anemic REST
- **Signal**: Only CRUD operations, no business verbs
- **Fix**: Add action endpoints: `POST /orders/{id}/cancel`

### Breaking Changes Without Versioning
- **Signal**: Clients break after API update
- **Fix**: Version APIs, deprecation policy, backward compat period

### Exposing Internal IDs
- **Signal**: Sequential integer IDs in URLs (security risk)
- **Fix**: UUIDs or opaque identifiers for public APIs

## Detection Priority

When reviewing code, check in this order:
1. **Security** — injection, XSS, auth bypass (immediate fix)
2. **Circular deps** — structural rot spreads fast
3. **God objects** — break SRP first, everything else follows
4. **N+1 queries** — performance killer in production
5. **Duplication** — see duplication.md for threshold rules
6. **Dead code** — remove immediately
7. **Missing abstractions** — extract when 3+ duplicates found
8. **Naming** — only fix if genuinely confusing
