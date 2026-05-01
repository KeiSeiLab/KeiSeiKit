# Building a single-binary `kei-mcp-server`

> KeiSeiKit v0.18 Phase 1 (exobrain) — ship the MCP server as a portable
> static binary so any machine without Node can run it off a USB drive.

## Tooling

Compile via **bun** (`bun build --compile`). Bundles the Bun runtime + JS
into one static executable — no Node, no `node_modules/` at runtime.
Requires bun `>= 1.0`. Docs (target list + flags):
[VERIFIED: https://bun.sh/docs/bundler/executables]

## Supported targets

| Platform | Arch  | `--target=`            | Output name                          |
|----------|-------|------------------------|---------------------------------------|
| Linux    | x64   | `bun-linux-x64`        | `kei-mcp-server-linux-x64`            |
| Linux    | arm64 | `bun-linux-arm64`      | `kei-mcp-server-linux-arm64`          |
| macOS    | x64   | `bun-darwin-x64`       | `kei-mcp-server-darwin-x64`           |
| macOS    | arm64 | `bun-darwin-arm64`     | `kei-mcp-server-darwin-arm64`         |
| Windows  | x64   | `bun-windows-x64`      | `kei-mcp-server-windows-x64.exe`      |

## Local build

```bash
cd _ts_packages/packages/mcp-server
bun install
bun run build:native                       # host-native
bun run build:native:darwin-arm64          # explicit cross-target
```

Output lands in `dist/`. Size ~85–95 MB per binary (bundled runtime).

## Release build (CI)

`.github/workflows/release.yml` → job `build-mcp-binary` runs the 5-target
matrix on tag push (`v*`) and attaches binaries + `.sha256` sums to the
GitHub release. Runtime requirement: **none** (static).

## Troubleshooting

- **macOS Gatekeeper (“cannot be opened because Apple cannot check it for
  malicious software”)** — remove the quarantine attribute:
  `xattr -d com.apple.quarantine ./kei-mcp-server-darwin-arm64`
- **Windows SmartScreen / AV flags** — not signed; right-click →
  Properties → Unblock, or add an AV exclusion for the binary path.
- **Missing symbol at startup** — usually a native-only dep that resolved
  at runtime on Node but cannot be bundled. Re-run `bun install`, then
  `bun build --compile ... --smol` to surface the resolution error.
- **`.js` ESM imports fail** — the mcp-server source imports via `.js`
  suffix (ESM canonical). Bun resolves these from the sibling `.ts`
  file automatically; no `tsc` pre-step needed.

## Lockfile

Since v0.19.1 the `_ts_packages` workspace ships a **single** `bun.lock`
at the workspace root (`_ts_packages/bun.lock`). Bun is a monorepo
tool — one lockfile covers all `packages/*` including mcp-server.

The release workflow runs `bun install --frozen-lockfile` from the
workspace root with NO fallback — a missing or out-of-date lockfile
fails the build on purpose. This is H4 supply-chain defense: every
release builds against the exact dependency tree recorded in the
committed lockfile, not whatever the npm registry serves that day.

**Before every release tag:**
1. `cd _ts_packages`
2. `bun install` (regenerates `bun.lock` if any `packages/*/package.json` changed)
3. Commit `bun.lock` if it changed
4. Tag the release

**If you see the build fail with "lockfile out of sync" on a tag push:**
you pushed the tag before committing an updated `bun.lock`. Fix:
generate the lockfile locally, commit, re-tag.

**Coexistence with `package-lock.json`:** the top-level CI (`ci.yml`)
still uses `npm ci` against `package-lock.json` for TypeScript tests.
Both lockfiles are committed. Audit finding L2 (dual-lockfile skew
risk) is tracked for a future `v0.20` consolidation — pick one tool
across CI + release. Until then, re-run `bun install` AND
`npm install` whenever any `packages/*/package.json` changes.
