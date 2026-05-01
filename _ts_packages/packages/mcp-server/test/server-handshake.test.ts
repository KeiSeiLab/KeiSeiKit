import { describe, it, expect } from "vitest";
import { McpServer } from "../src/server.js";

describe("server handshake + tool listing", () => {
  it("listTools returns every primitive plus kei", () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    const tools = srv.listTools();
    const names = new Set(tools.map((t) => t.name));
    expect(names.has("kei")).toBe(true);
    expect(names.has("kei-ledger")).toBe(true);
    expect(names.has("kei-memory")).toBe(true);
    expect(tools.length).toBeGreaterThanOrEqual(14);
  });

  it("every listed tool has a non-empty description", () => {
    const srv = new McpServer({ rustBinDir: "/tmp/stub" });
    for (const t of srv.listTools()) {
      expect(t.description.length).toBeGreaterThan(0);
    }
  });
});
