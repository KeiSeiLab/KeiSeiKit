#!/usr/bin/env node
// Entry point: parse argv, select transport (stdio or HTTP), start McpServer.

import fs from "node:fs/promises";
import path from "node:path";
import { McpServer } from "./server.js";

interface CliArgs {
  stdio: boolean;
  port?: number;
  bind: string;
  authTokenFile?: string;
  rustBinDir: string;
}

function cliFail(msg: string): never {
  process.stderr.write(`[keisei-mcp] error: ${msg}\n`);
  process.exit(2);
}

function takeValue(argv: readonly string[], i: number, flag: string): string {
  const v = argv[i + 1];
  if (v === undefined || v.startsWith("--")) cliFail(`${flag} requires a value (got ${v ?? "EOF"})`);
  return v;
}

function parsePort(raw: string): number {
  const n = Number(raw);
  if (!Number.isInteger(n) || n < 1 || n > 65535) cliFail(`--port must be 1..65535 (got ${raw})`);
  return n;
}

function parseArgv(argv: readonly string[]): CliArgs {
  const out: CliArgs = {
    stdio: false,
    bind: "127.0.0.1",
    rustBinDir: process.env["KEI_RUST_BIN_DIR"] ?? defaultBinDir(),
  };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--stdio") out.stdio = true;
    else if (a === "--port") { out.port = parsePort(takeValue(argv, i, "--port")); i++; }
    else if (a === "--bind") { out.bind = takeValue(argv, i, "--bind"); i++; }
    else if (a === "--auth-token-file") { out.authTokenFile = takeValue(argv, i, "--auth-token-file"); i++; }
    else if (a === "--rust-bin-dir") { out.rustBinDir = takeValue(argv, i, "--rust-bin-dir"); i++; }
    else if (a !== undefined && a.startsWith("--")) cliFail(`unknown flag ${a}`);
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
  else await runHttp(server, args.port ?? 3000, args.bind);
}

async function runStdio(server: McpServer): Promise<void> {
  process.stderr.write(`[keisei-mcp] stdio mode; ${server.listTools().length} tools\n`);
  process.stdin.setEncoding("utf8");
  await dispatchStdio(server);
}

async function dispatchStdio(server: McpServer): Promise<void> {
  let pending = "";
  for await (const chunk of process.stdin) {
    pending += String(chunk);
    let nl = pending.indexOf("\n");
    while (nl !== -1) {
      const frame = pending.slice(0, nl).trim();
      pending = pending.slice(nl + 1);
      if (frame.length > 0) {
        const resp = await dispatchStdioLine(server, frame);
        process.stdout.write(resp + "\n");
      }
      nl = pending.indexOf("\n");
    }
  }
  const tail = pending.trim();
  if (tail.length > 0) {
    const resp = await dispatchStdioLine(server, tail);
    process.stdout.write(resp + "\n");
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

function bindErrorMessage(err: NodeJS.ErrnoException, port: number, bindAddr: string): string {
  if (err.code === "EADDRINUSE") return `port ${port} already in use on ${bindAddr}`;
  if (err.code === "EACCES") return `permission denied to bind ${bindAddr}:${port} (try port >=1024)`;
  return `bind error: ${String(err)}`;
}

async function runHttp(server: McpServer, port: number, bindAddr: string): Promise<void> {
  const http = await import("node:http");
  const srv = http.createServer((req, res) => void handleHttp(server, req, res));
  // Bind to 127.0.0.1 by default; pass --bind 0.0.0.0 to expose on all interfaces.
  await new Promise<void>((resolve, reject) => {
    srv.once("error", (err: NodeJS.ErrnoException) => reject(new Error(bindErrorMessage(err, port, bindAddr))));
    srv.listen(port, bindAddr, () => {
      process.stderr.write(`[keisei-mcp] http ${bindAddr}:${port}; ${server.listTools().length} tools\n`);
      resolve();
    });
  });
}

async function handleHttp(server: McpServer, req: import("node:http").IncomingMessage, res: import("node:http").ServerResponse): Promise<void> {
  if (req.method !== "POST") {
    res.writeHead(405);
    res.end();
    return;
  }
  const MAX_BODY = 1 * 1024 * 1024; // 1 MiB
  const chunks = await readBodyCapped(req, res, MAX_BODY);
  if (chunks === null) return;
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

async function readBodyCapped(
  req: import("node:http").IncomingMessage,
  res: import("node:http").ServerResponse,
  maxBody: number,
): Promise<Buffer[] | null> {
  const chunks: Buffer[] = [];
  let total = 0;
  try {
    for await (const c of req) {
      total += (c as Buffer).length;
      if (total > maxBody) {
        res.writeHead(413, { "content-type": "application/json" });
        res.end(JSON.stringify({ ok: false, error: { code: -32600, message: "request body exceeds 1 MiB" } }));
        req.socket.destroy();
        return null;
      }
      chunks.push(c as Buffer);
    }
  } catch (e) {
    if (!res.headersSent) {
      res.writeHead(400, { "content-type": "application/json" });
      res.end(JSON.stringify({ ok: false, error: { code: -32700, message: `request stream error: ${String(e)}` } }));
    }
    return null;
  }
  return chunks;
}

main().catch((err: unknown) => {
  process.stderr.write(`[keisei-mcp] fatal: ${String(err)}\n`);
  process.exit(1);
});
