import { describe, it, expect } from "vitest";
import { McpServer } from "../src/server.js";

describe("kei() meta-tool routing", () => {
  it("rejects empty query via zod validation", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    const resp = await srv.handle({ tool: "kei", args: { query: "" } });
    expect(resp.ok).toBe(false);
    expect(resp.error?.code).toBe(-32602);
  });

  it("rejects missing query via zod validation", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    const resp = await srv.handle({ tool: "kei", args: {} });
    expect(resp.ok).toBe(false);
    expect(resp.error?.code).toBe(-32602);
  });

  it("accepts a non-empty query and routes via kei-router (resolves with non-zero exit)", async () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    const resp = await srv.handle({ tool: "kei", args: { query: "list ledger entries" } });
    // Schema passes → meta-tool runs → router binary missing → handler formats result string.
    expect(resp.ok).toBe(true);
    expect(resp.result).toContain("router failed");
  });
});
