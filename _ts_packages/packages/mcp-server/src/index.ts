#!/usr/bin/env node
// Entry point: parse argv, select transport (stdio or HTTP), start McpServer.

import fs from "node:fs/promises";
import path from "node:path";
import { McpServer } from "./server.js";

interface CliArgs {
  stdio: boolean;
  port?: number;
  authTokenFile?: string;
  rustBinDir: string;
}

function parseArgv(argv: readonly string[]): CliArgs {
  const out: CliArgs = {
    stdio: false,
    rustBinDir: process.env["KEI_RUST_BIN_DIR"] ?? defaultBinDir(),
  };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--stdio") out.stdio = true;
    else if (a === "--port") out.port = Number(argv[++i] ?? "");
    else if (a === "--auth-token-file") {
      const v = argv[++i];
      if (v !== undefined) out.authTokenFile = v;
    } else if (a === "--rust-bin-dir") {
      const v = argv[++i];
      if (v !== undefined) out.rustBinDir = v;
    }
  }
  return out;
}

function defaultBinDir(): string {
  const home = process.env["HOME"] ?? "";
  return path.join(home, ".claude", "agents", "_primitives", "_rust", "target", "release");
}

async function readTokenFile(p: string | undefined): Promise<string | undefined> {
  if (!p) return process.env["KEI_MCP_AUTH_TOKEN"];
  const raw = await fs.readFile(p, "utf8");
  return raw.trim();
}

async function main(): Promise<void> {
  const args = parseArgv(process.argv.slice(2));
  const token = args.stdio ? undefined : await readTokenFile(args.authTokenFile);
  const server = new McpServer({
    rustBinDir: args.rustBinDir,
    ...(token !== undefined ? { authToken: token } : {}),
  });
  await server.loadAdapters((m) => process.stderr.write(`[adapters] ${m}\n`));
  if (args.stdio) await runStdio(server);
  else await runHttp(server, args.port ?? 3000);
}

async function runStdio(server: McpServer): Promise<void> {
  process.stderr.write(`[keisei-mcp] stdio mode; ${server.listTools().length} tools\n`);
  process.stdin.setEncoding("utf8");
  for await (const chunk of process.stdin) {
    for (const line of String(chunk).split("\n")) {
      const trimmed = line.trim();
      if (!trimmed) continue;
      const resp = await dispatchStdioLine(server, trimmed);
      process.stdout.write(resp + "\n");
    }
  }
}

async function dispatchStdioLine(server: McpServer, line: string): Promise<string> {
  try {
    const payload = JSON.parse(line) as { tool: string; args?: Record<string, unknown> };
    const call = payload.args !== undefined
      ? { tool: payload.tool, args: payload.args }
      : { tool: payload.tool };
    const resp = await server.handle(call);
    return JSON.stringify(resp);
  } catch (err) {
    return JSON.stringify({ ok: false, error: { code: -32700, message: String(err) } });
  }
}

async function runHttp(server: McpServer, port: number): Promise<void> {
  const http = await import("node:http");
  const srv = http.createServer((req, res) => void handleHttp(server, req, res));
  srv.listen(port, () =>
    process.stderr.write(`[keisei-mcp] http :${port}; ${server.listTools().length} tools\n`),
  );
}

async function handleHttp(server: McpServer, req: import("node:http").IncomingMessage, res: import("node:http").ServerResponse): Promise<void> {
  if (req.method !== "POST") {
    res.writeHead(405);
    res.end();
    return;
  }
  const chunks: Buffer[] = [];
  for await (const c of req) chunks.push(c as Buffer);
  try {
    const body = JSON.parse(Buffer.concat(chunks).toString("utf8")) as {
      tool: string;
      args?: Record<string, unknown>;
    };
    const authHeader = req.headers["authorization"];
    const header = typeof authHeader === "string" ? authHeader.replace(/^Bearer\s+/i, "") : undefined;
    const resp = await server.handle({
      tool: body.tool,
      ...(body.args !== undefined ? { args: body.args } : {}),
      ...(header !== undefined ? { authHeader: header } : {}),
    });
    res.writeHead(resp.ok ? 200 : 400, { "content-type": "application/json" });
    res.end(JSON.stringify(resp));
  } catch (err) {
    res.writeHead(400, { "content-type": "application/json" });
    res.end(JSON.stringify({ ok: false, error: { code: -32700, message: String(err) } }));
  }
}

main().catch((err: unknown) => {
  process.stderr.write(`[keisei-mcp] fatal: ${String(err)}\n`);
  process.exit(1);
});
