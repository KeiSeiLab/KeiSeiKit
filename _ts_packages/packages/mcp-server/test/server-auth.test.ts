import { describe, it, expect } from "vitest";
import { McpServer } from "../src/server.js";

describe("server auth", () => {
  it("rejects calls without a token when auth is enabled", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub", authToken: "secret" });
    const resp = await srv.handle({ tool: "kei-ledger", args: { args: {} } });
    expect(resp.ok).toBe(false);
    expect(resp.error?.code).toBe(-32001);
  });

  it("rejects calls with a wrong token", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub", authToken: "secret" });
    const resp = await srv.handle({
      tool: "kei-ledger",
      args: { args: {} },
      authHeader: "wrong",
    });
    expect(resp.ok).toBe(false);
    expect(resp.error?.code).toBe(-32001);
  });

  it("allows calls when auth is disabled (stdio mode)", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    const resp = await srv.handle({ tool: "does-not-exist", args: {} });
    // auth passes → fails on tool lookup instead
    expect(resp.ok).toBe(false);
    expect(resp.error?.code).toBe(-32601);
  });
});
