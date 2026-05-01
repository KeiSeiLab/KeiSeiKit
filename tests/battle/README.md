# tests/battle — Clean-Distro Install Test Matrix

Validates `install.sh` on fresh container images. CI only runs
`--no-execute` dry-runs; these battle-tests actually execute the
installer against real distro package sets.

## Matrix (v0.22+)

| Image | libc | Dockerfile |
|---|---|---|
| `ubuntu:24.04` | glibc | `Dockerfile.install-test` (CI baseline) |
| `alpine:3.19`  | musl  | `Dockerfile.install-test-alpine` |
| `debian:12`    | glibc | `Dockerfile.install-test-debian` |

`ubuntu` is the historic baseline. `alpine` exposes musl-static-link
quirks in `rusqlite`, `git2`, `aws-sdk-s3` — crates that wrap C code and
are known to behave differently under musl. `debian` covers the most
common server deployment; its apt repo layout differs from Ubuntu, so a
Debian pass rules out a "ubuntu-specific fix" regression.

## Run one image

From repo root:

```bash
docker build -t keisei-battle-ubuntu -f tests/battle/Dockerfile.install-test .
docker run --rm                   keisei-battle-ubuntu   # minimal
docker run --rm -e PROFILE=core   keisei-battle-ubuntu
docker run --rm -e PROFILE=dev    keisei-battle-ubuntu
docker run --rm -e PROFILE=full   keisei-battle-ubuntu
```

## Run the whole matrix

```bash
for distro in ubuntu alpine debian; do
    if [ "$distro" = "ubuntu" ]; then
        DF=Dockerfile.install-test
    else
        DF=Dockerfile.install-test-$distro
    fi
    docker build -t keisei-battle-$distro -f tests/battle/$DF .
    docker run --rm -e PROFILE=full keisei-battle-$distro
done
```

Container exits 0 = green. Any other code = investigate stdout.

## What it asserts (verify.sh)

- `~/.claude/agents/_blocks`  ≥ 79
- `~/.claude/skills`          ≥ 39
- `~/.claude/hooks/*.sh`      ≥ 10 top-level
- `~/.claude/hooks/_lib/*.sh` ≥ 2 (gate.sh + test-gate.sh, v0.17)
- `hooks/_lib/test-gate.sh` self-test passes (11/11)
- `settings.json` (if created) parses as valid JSON

## Known quirks (2026-04-22)

- **`kei-artifact` crate fails** on `dev`/`full`: `copy_rust_primitive`
  (install/lib-primitives.sh) copies `src/` + `tests/` only — misses
  sibling `schemas/`, so `include_str!("../schemas/*.json")` breaks.
  Install still exits 0 (build is soft-fail); primitive binary count
  drops (`6/25` on full). Fix: copy every sibling dir the crate ships.
- **Ubuntu 24.04 rustc is 1.75** — too old for `edition = "2024"`.
  Dockerfile installs rustup stable; `apt install rustc` is NOT enough.
- **Debian 12 rustc is 1.63** — same story. Dockerfile uses rustup.
- **Alpine 3.19 rustc is 1.76** — still below edition-2024's 1.85 floor;
  some primitives may fail to build. That failure IS what this image
  catches; document as "known-issue on musl" rather than patching here.
- **Alpine musl + aws-sdk-s3 / rusqlite / git2**: static-link failures
  are EXPECTED on musl for the `s3` / SQLite-backed primitives. Treat
  as matrix signal, not regression.
- **Apple Silicon hosts**: images build linux/arm64 natively; binaries
  produced inside won't run on x86_64 hosts.
