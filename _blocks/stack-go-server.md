# STACK — Go server

Use when the project is Go-locked (existing codebase) or the domain fits — networking daemons, agents, cloud-native tooling.

**Modules:** `go.mod` + `go.sum` committed. Go ≥ 1.22 (range-over-func, better `slices`/`maps` stdlib).

**HTTP:** prefer `net/http` stdlib + `http.ServeMux` (Go 1.22 pattern matching routes). Add a framework (chi, echo) only when the feature gap is concrete and documented — not "for ergonomics".

**Context propagation (non-negotiable):**
- Every handler, DB call, outbound request takes `ctx context.Context` as FIRST arg.
- `ctx` threads through stack without interruption — no `context.Background()` mid-call except at the edge.
- `context.WithTimeout` on every external I/O.

**Errors:**
- Return `error`; sentinels via `errors.Is`, typed via `errors.As`. NEVER `strings.Contains(err.Error(), "...")` — string match breaks on wrapping.
- Wrap with `%w`: `fmt.Errorf("ctx: %w", err)`.

**Concurrency:**
- `go vet` + `go test -race` MANDATORY in CI.
- Channels for ownership transfer, mutexes for protecting state — not both on the same data.
- Goroutines started in handlers must have a clear lifecycle (parent ctx cancellation).

**Logging:** `log/slog` (structured). NO `fmt.Println` in prod paths.

**Forbidden:** string-match on error messages; goroutine leaks (no ctx cancellation path); `init()` doing I/O; `go test` without `-race`; `panic()` as control flow in library code.
