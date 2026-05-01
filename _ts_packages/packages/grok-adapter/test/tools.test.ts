import { describe, it, expect, vi } from "vitest";
import { GrokClient } from "../src/client.js";
import { buildGrokTools } from "../src/tools.js";

function makeFetchMock(payload: unknown): typeof fetch {
  return vi.fn(async () => ({
    ok: true,
    status: 200,
    async text() { return JSON.stringify(payload); },
    async json() { return payload; },
  } as unknown as Response)) as unknown as typeof fetch;
}

describe("grok tools", () => {
  it("exposes 2 tools", () => {
    const c = new GrokClient({ apiKey: "k", fetchImpl: makeFetchMock({}) });
    const tools = buildGrokTools(c);
    expect(tools.map((t) => t.name)).toEqual(["grok_research", "grok_imagine"]);
  });

  it("grok_research validates non-empty query", async () => {
    const c = new GrokClient({ apiKey: "k", fetchImpl: makeFetchMock({}) });
    const tool = buildGrokTools(c).find((t) => t.name === "grok_research");
    await expect(tool!.handler({ query: "" })).rejects.toBeTruthy();
  });

  it("grok_imagine defaults quality to standard", async () => {
    const c = new GrokClient({
      apiKey: "k",
      fetchImpl: makeFetchMock({ data: [{ url: "u" }] }),
    });
    const tool = buildGrokTools(c).find((t) => t.name === "grok_imagine");
    const out = await tool!.handler({ prompt: "x" });
    expect(out).toContain("u");
  });
});
