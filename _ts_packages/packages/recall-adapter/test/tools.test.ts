import { describe, it, expect, vi } from "vitest";
import { RecallClient } from "../src/client.js";
import { buildRecallTools } from "../src/tools.js";

function makeFetchMock(payload: unknown): typeof fetch {
  return vi.fn(async () => ({
    ok: true,
    status: 200,
    async text() { return JSON.stringify(payload); },
    async json() { return payload; },
  } as unknown as Response)) as unknown as typeof fetch;
}

describe("recall tool surface", () => {
  it("exposes 5 tools", () => {
    const c = new RecallClient({ apiKey: "k", fetchImpl: makeFetchMock({}) });
    const tools = buildRecallTools(c);
    expect(tools.map((t) => t.name)).toEqual([
      "zoom_status",
      "zoom_bots",
      "zoom_join",
      "zoom_leave",
      "zoom_chat",
    ]);
  });

  it("zoom_join validates meeting_url as URL", async () => {
    const c = new RecallClient({ apiKey: "k", fetchImpl: makeFetchMock({}) });
    const tool = buildRecallTools(c).find((t) => t.name === "zoom_join");
    await expect(tool!.handler({ meeting_url: "not a url" })).rejects.toBeTruthy();
  });

  it("zoom_status returns JSON string", async () => {
    const c = new RecallClient({ apiKey: "k", fetchImpl: makeFetchMock({ id: "x" }) });
    const tool = buildRecallTools(c).find((t) => t.name === "zoom_status");
    const out = await tool!.handler({ bot_id: "x" });
    expect(out).toContain('"id": "x"');
  });

  it("zoom_bots hits list endpoint", async () => {
    const fetchImpl = makeFetchMock([{ id: "a" }]);
    const c = new RecallClient({ apiKey: "k", fetchImpl });
    const tool = buildRecallTools(c).find((t) => t.name === "zoom_bots");
    const out = await tool!.handler({});
    expect(out).toContain("a");
  });
});
