# DEPLOY — Docker

**Dockerfile — multi-stage MANDATORY** (build tools never ship to prod image):
```
FROM rust:1.80 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin myapp

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/myapp /myapp
USER nonroot:nonroot
HEALTHCHECK --interval=30s --timeout=3s CMD ["/myapp", "--healthcheck"]
ENTRYPOINT ["/myapp"]
```

**Base image:** `distroless` (preferred, no shell — smaller attack surface) or `alpine` (if musl compat) or `debian:slim`. NEVER `ubuntu:latest` for prod.

**File ops:**
- `COPY` — deterministic. NEVER `ADD` (auto-extracts tars, fetches URLs — surprising behavior).
- `.dockerignore` committed. Includes `.git`, `target/`, `node_modules/`, `.env*`, `secrets/`.

**Secrets:**
- NEVER `ENV SECRET=...` — leaks into image layers forever.
- Build-time secrets via `--secret id=foo,src=./foo.txt` (BuildKit).
- Runtime secrets via env injection from orchestrator / docker-compose `secrets:` (Swarm) / K8s Secret.

**User:** `USER nonroot` (distroless provides it) or explicit `RUN useradd -u 10001 app && USER app`. Running as root = CVE amplifier.

**Healthcheck:** MANDATORY. Orchestrator uses it for readiness/liveness; without it, failed containers stay "up".

**docker-compose:** LOCAL DEV ONLY. For prod, the orchestrator (ECS, Fargate, K8s, Nomad, Docker Swarm) owns the deployment. Typical prod pattern: single container listening on internal port, behind nginx reverse proxy on a public port, colocated on a shared host.

**Forbidden:** `ADD` for local files (use `COPY`); `USER root` in final stage; secrets in `ENV` or `ARG`; missing `HEALTHCHECK`; `docker-compose` as prod orchestrator; `:latest` tags in prod manifests; single-stage Dockerfile that ships build toolchain.
