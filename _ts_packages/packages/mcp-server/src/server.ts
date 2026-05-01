// MCP server assembly: wire registry + adapters + auth into a JSON-RPC dispatcher.
// Transport-agnostic; index.ts chooses stdio or HTTP.

import crypto from "node:crypto";
import { z } from "zod";
import { buildRegistry, lookupTool, type ToolDefinition } from "./tool-registry.js";
import { RustBridge } from "./rust-bridge.js";
import { loadAllAdapters } from "./adapters.js";
import { AuthError, SchemaError, toErrorPayload } from "./errors.js";

export interface ServerConfig {
  rustBinDir: string;
  authToken?: string;
  timeoutMs?: number;
}

export interface JsonRpcCall {
  tool: string;
  args?: Record<string, unknown>;
  authHeader?: string | undefined;
}

export interface JsonRpcResponse {
  ok: boolean;
  result?: string;
  error?: { code: number; message: string; data?: unknown };
}

export class McpServer {
  private readonly registry: Map<string, ToolDefinition>;
  private readonly authToken: string | undefined;

  constructor(cfg: ServerConfig) {
    const bridge = new RustBridge({
      binDir: cfg.rustBinDir,
      ...(cfg.timeoutMs !== undefined ? { defaultTimeoutMs: cfg.timeoutMs } : {}),
    });
    this.registry = buildRegistry(bridge);
    this.authToken = cfg.authToken;
  }

  async loadAdapters(logger?: (msg: string) => void): Promise<{ loaded: string[]; skipped: string[] }> {
    return loadAllAdapters((tool) => this.registry.set(tool.name, tool), logger);
  }

  listTools(): Array<{ name: string; description: string }> {
    return Array.from(this.registry.values()).map((t) => ({
      name: t.name,
      description: t.description,
    }));
  }

  async handle(call: JsonRpcCall): Promise<JsonRpcResponse> {
    try {
      this.checkAuth(call.authHeader);
      const tool = lookupTool(this.registry, call.tool);
      const args = this.validateArgs(tool, call.args ?? {});
      const out = await tool.handler(args);
      return { ok: true, result: out };
    } catch (err) {
      return { ok: false, error: toErrorPayload(err) };
    }
  }

  private checkAuth(header: string | undefined): void {
    if (!this.authToken) return; // auth disabled (stdio mode)
    if (!header) throw new AuthError("missing auth token");
    if (!safeEqual(header, this.authToken)) throw new AuthError("invalid auth token");
  }

  private validateArgs(
    tool: ToolDefinition,
    raw: Record<string, unknown>,
  ): Record<string, unknown> {
    const parsed = tool.inputSchema.safeParse(raw);
    if (!parsed.success) {
      throw new SchemaError(parsed.error.message, { tool: tool.name });
    }
    return parsed.data as Record<string, unknown>;
  }
}

function safeEqual(a: string, b: string): boolean {
  const ba = Buffer.from(a);
  const bb = Buffer.from(b);
  if (ba.length !== bb.length) return false;
  return crypto.timingSafeEqual(ba, bb);
}

// Exported for tests
export const __testing__ = { safeEqual, schema: z };
