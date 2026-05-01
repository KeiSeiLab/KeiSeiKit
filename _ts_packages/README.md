# KeiSeiKit TypeScript Packages

> v0.14.0 part B: MCP server layer + external-API adapters.

## RULE 0.2 exception

TypeScript is chosen here under **RULE 0.2 exception #4 (Browser/DOM adjacent)** because:

1. The official Model Context Protocol SDK is TypeScript-native; Rust MCP
   libraries are immature (as of 2026-04).
2. The API adapters rely on JS-native SDKs with no Rust equivalents:
   - `grammy` (type-safe Telegram bot)
   - `googleapis` (official Google API SDK for Gmail + YouTube)
   - `youtube-transcript` (Tier-1 free transcript extractor)
3. Async, JSON-heavy glue code is TypeScript's sweet spot.

**Core primitives (signing, ledger, graph, memory, refactor, etc.) remain
Rust** in `../_primitives/_rust/`. This TS layer is a THIN wrapper: it
spawns the Rust CLIs as subprocesses and exposes them as MCP tools, plus
the six adapters above that have no Rust equivalent.

## Layout

```
_ts_packages/
├── package.json              npm workspace root
├── tsconfig.base.json        strict TS 5.x
└── packages/
    ├── mcp-server/           @keisei/mcp-server
    ├── telegram-adapter/     @keisei/telegram-adapter
    ├── recall-adapter/       @keisei/recall-adapter  (Zoom via Recall.ai)
    ├── grok-adapter/         @keisei/grok-adapter    (xAI)
    ├── gmail-adapter/        @keisei/gmail-adapter
    └── youtube-adapter/      @keisei/youtube-adapter
```

## Install (for end users)

### 1. Install workspace deps

```bash
cd _ts_packages
npm install
npm run build
```

### 2. Link each package as a global CLI (optional)

```bash
npm i -g ./packages/mcp-server
npm i -g ./packages/telegram-adapter
# ... etc
```

Or install into a Claude agent directory:

```bash
npm i --prefix ~/.claude/agents/_ts_packages/packages/mcp-server \
      ./_ts_packages/packages/mcp-server
```

## Environment variables (RULE 0.8 — secrets in `~/.claude/secrets/.env`)

| Var | Package | Purpose |
|---|---|---|
| `TELEGRAM_BOT_TOKEN` | telegram-adapter | Bot API token |
| `RECALL_API_KEY` | recall-adapter | Recall.ai API key (Zoom meetings) |
| `XAI_API_KEY` | grok-adapter | xAI Grok API key |
| `GMAIL_CLIENT_ID` | gmail-adapter | Google OAuth2 client id |
| `GMAIL_CLIENT_SECRET` | gmail-adapter | Google OAuth2 client secret |
| `GMAIL_REFRESH_TOKEN` | gmail-adapter | Long-lived OAuth2 refresh token |
| `YOUTUBE_API_KEY` | youtube-adapter | YouTube Data API v3 key |
| `KEI_MCP_AUTH_TOKEN` | mcp-server | HMAC token for tool callers |
| `KEI_RUST_BIN_DIR` | mcp-server | Override directory holding Rust primitive CLIs |

All are read via `process.env`. Hardcoding tokens is **forbidden** (RULE 0.8).

## MCP server integration

The `@keisei/mcp-server` exposes the Rust primitive CLIs as MCP tools. The
pattern is one Rust binary = one MCP tool, with the `kei` meta-tool on
top that routes natural-language queries via `kei-router`.

Stdio mode (for Claude Code native integration):

```bash
npx @keisei/mcp-server --stdio
```

HTTP mode:

```bash
npx @keisei/mcp-server --port 3000 --auth-token-file ~/.claude/mcp-token
```

## Verification

```bash
npm install
npm run build --workspaces
npm run test --workspaces
```

All six packages compile under `strict: true`. Total new LOC: see commit.

## Migration notes

- Zero impact on existing KeiSeiKit users unless they opt into the MCP
  server (planned v0.14.1 installer flag `--enable-mcp`).
- The Rust primitives are unchanged; this layer only **wraps** them.
- Gmail and YouTube adapters are **new** (gaps in LBM).
