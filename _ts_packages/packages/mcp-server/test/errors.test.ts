import { describe, it, expect } from "vitest";
import {
  AuthError,
  McpServerError,
  RustBridgeError,
  SchemaError,
  ToolNotFoundError,
  TimeoutError,
  isMcpError,
  toErrorPayload,
} from "../src/errors.js";

describe("errors hierarchy", () => {
  it("AuthError has JSON-RPC code -32001", () => {
    const e = new AuthError();
    expect(e).toBeInstanceOf(McpServerError);
    expect(e.code).toBe(-32001);
    expect(isMcpError(e)).toBe(true);
  });

  it("ToolNotFoundError carries the tool name in data", () => {
    const e = new ToolNotFoundError("kei-foo");
    expect(e.code).toBe(-32601);
    expect((e.data as { tool: string }).tool).toBe("kei-foo");
  });

  it("RustBridgeError prefixes message", () => {
    const e = new RustBridgeError("spawn failed");
    expect(e.message).toContain("rust bridge");
  });

  it("SchemaError has JSON-RPC code -32602", () => {
    const e = new SchemaError("bad input");
    expect(e.code).toBe(-32602);
  });

  it("TimeoutError records ms and tool", () => {
    const e = new TimeoutError("kei-ledger", 1234);
    expect(e.code).toBe(-32003);
    expect((e.data as { ms: number }).ms).toBe(1234);
  });

  it("toErrorPayload handles MCP errors", () => {
    const p = toErrorPayload(new AuthError("nope"));
    expect(p.code).toBe(-32001);
    expect(p.message).toBe("nope");
  });

  it("toErrorPayload handles plain Errors", () => {
    const p = toErrorPayload(new Error("boom"));
    expect(p.code).toBe(-32000);
    expect(p.message).toBe("boom");
  });
});
