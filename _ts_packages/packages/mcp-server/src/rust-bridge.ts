// Bridge layer: spawn Rust primitive CLIs and marshal JSON args <-> CLI flags.
// One Rust binary = one MCP tool. Subprocess lifecycle is isolated per call.

import { execa } from "execa";
import path from "node:path";
import { RustBridgeError, TimeoutError } from "./errors.js";

const DEFAULT_TIMEOUT_MS = 30_000;

export interface RustCallRequest {
  binary: string;
  args: readonly string[];
  stdin?: string;
  timeoutMs?: number;
}

export interface RustCallResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

export interface RustBridgeConfig {
  binDir: string;
  defaultTimeoutMs?: number;
}

export class RustBridge {
  private readonly binDir: string;
  private readonly defaultTimeoutMs: number;

  constructor(cfg: RustBridgeConfig) {
    this.binDir = cfg.binDir;
    this.defaultTimeoutMs = cfg.defaultTimeoutMs ?? DEFAULT_TIMEOUT_MS;
  }

  async call(req: RustCallRequest): Promise<RustCallResult> {
    const binPath = this.resolveBin(req.binary);
    const timeoutMs = req.timeoutMs ?? this.defaultTimeoutMs;
    try {
      const opts = {
        timeout: timeoutMs,
        reject: false as const,
        env: process.env,
        ...(req.stdin !== undefined ? { input: req.stdin } : {}),
      };
      const child = execa(binPath, [...req.args], opts);
      const result = await child;
      if (result.timedOut) throw new TimeoutError(req.binary, timeoutMs);
      return {
        stdout: typeof result.stdout === "string" ? result.stdout : "",
        stderr: typeof result.stderr === "string" ? result.stderr : "",
        exitCode: result.exitCode ?? -1,
      };
    } catch (err) {
      if (err instanceof TimeoutError) throw err;
      const msg = err instanceof Error ? err.message : String(err);
      throw new RustBridgeError(msg, { binary: req.binary });
    }
  }

  private resolveBin(binary: string): string {
    if (!/^[a-z0-9][a-z0-9_-]*$/i.test(binary)) {
      throw new RustBridgeError(`invalid binary name: ${binary}`);
    }
    return path.join(this.binDir, binary);
  }
}

// Convert a JSON object of named args to CLI flags: {foo_bar: "v"} => ["--foo-bar", "v"]
export function jsonArgsToCli(args: Record<string, unknown>): string[] {
  const out: string[] = [];
  for (const [key, raw] of Object.entries(args)) {
    if (raw === undefined || raw === null) continue;
    const flag = `--${key.replace(/_/g, "-")}`;
    if (typeof raw === "boolean") {
      if (raw) out.push(flag);
      continue;
    }
    out.push(flag, String(raw));
  }
  return out;
}
