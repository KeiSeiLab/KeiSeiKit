import { describe, it, expect, vi } from "vitest";
import { RecallClient } from "../src/client.js";

function makeFetchMock(payload: unknown, ok = true, status = 200): typeof fetch {
  return vi.fn(async () => {
    return {
      ok,
      status,
      async text() { return JSON.stringify(payload); },
      async json() { return payload; },
    } as unknown as Response;
  }) as unknown as typeof fetch;
}

describe("RecallClient", () => {
  it("rejects empty API key", () => {
    expect(() => new RecallClient({ apiKey: "" })).toThrow(/RECALL_API_KEY/);
  });

  it("listBots calls GET /bot/", async () => {
    const fetchImpl = makeFetchMock([{ id: "b1" }]);
    const c = new RecallClient({ apiKey: "k", fetchImpl });
    const out = (await c.listBots()) as Array<{ id: string }>;
    expect(out[0]?.id).toBe("b1");
    expect(fetchImpl).toHaveBeenCalledOnce();
  });

  it("joinMeeting POSTs meeting_url", async () => {
    const fetchImpl = makeFetchMock({ id: "new" });
    const c = new RecallClient({ apiKey: "k", fetchImpl });
    const out = await c.joinMeeting("https://zoom.us/j/123");
    expect((out as { id: string }).id).toBe("new");
  });

  it("propagates non-2xx responses as Error", async () => {
    const fetchImpl = makeFetchMock({ detail: "forbidden" }, false, 403);
    const c = new RecallClient({ apiKey: "k", fetchImpl });
    await expect(c.getBot("b1")).rejects.toThrow(/403/);
  });
});
