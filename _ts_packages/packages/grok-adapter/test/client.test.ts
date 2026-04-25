import { describe, it, expect, vi } from "vitest";
import { GrokClient } from "../src/client.js";

function makeFetchMock(payload: unknown, ok = true, status = 200): typeof fetch {
  return vi.fn(async () => ({
    ok,
    status,
    async text() { return JSON.stringify(payload); },
    async json() { return payload; },
  } as unknown as Response)) as unknown as typeof fetch;
}

describe("GrokClient", () => {
  it("rejects empty API key", () => {
    expect(() => new GrokClient({ apiKey: "" })).toThrow(/XAI_API_KEY/);
  });

  it("deepResearch returns assistant content", async () => {
    const fetchImpl = makeFetchMock({ choices: [{ message: { content: "hello" } }] });
    const c = new GrokClient({ apiKey: "k", fetchImpl });
    const out = await c.deepResearch("q");
    expect(out).toBe("hello");
  });

  it("imageGenerate extracts URLs from response", async () => {
    const fetchImpl = makeFetchMock({ data: [{ url: "https://x/img.png" }] });
    const c = new GrokClient({ apiKey: "k", fetchImpl });
    const urls = await c.imageGenerate("a cat", true);
    expect(urls).toEqual(["https://x/img.png"]);
  });
});
