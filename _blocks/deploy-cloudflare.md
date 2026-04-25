# DEPLOY — Cloudflare (Workers / Pages / R2 / KV)

**Tooling:** `wrangler` CLI (≥ 3.x). `wrangler.toml` is source of truth for bindings, NOT dashboard clicks.

**Surface map:**
- **Workers** — edge compute. `wrangler deploy`. Logs via `wrangler tail`.
- **Pages** — static sites + Pages Functions. Per-branch preview URLs automatic.
- **R2** — S3-compatible object storage. No egress fees.
- **KV** — eventually-consistent key-value config store. Reads cached at the edge.
- **D1** — SQLite at edge (beta/GA track).

**Secrets (NEVER in `wrangler.toml`):**
```
wrangler secret put API_KEY      # interactive, encrypted at rest
wrangler secret put --env prod DB_URL
```
`wrangler.toml` is committed to git; secrets live in the platform vault only.

**Self-sufficiency — CF API token scopes (request ALL up front):**
Workers KV · Workers R2 · Workers Scripts · Pages · Zone Edit · DNS · Zone Read · Zone Settings · SSL. Missing scope → ask user to add to token, NEVER ask user to click in the dashboard.

**HARD RULE — CF ToS forbids proxy-mode traffic forwarding:**
- Worker for signaling, fronting helpers, metadata lookups — OK
- Worker as a full proxy pipe (upstream ⇆ Worker ⇆ downstream as a tunnel) — FORBIDDEN. Signaling / rendezvous Workers must do metadata only, NEVER arbitrary traffic. Violation → account ban.

**Cache strategy:** `Cache-Control` headers authoritative; purge via `wrangler pages deployment` or API. `NEXT_PUBLIC_*` / `PUBLIC_*` vars ship to client — treat as non-secret.

**Forbidden:** secrets in `wrangler.toml`, full-proxy Workers (ToS), manual dashboard edits when API token has the scope, committing `.dev.vars`.
