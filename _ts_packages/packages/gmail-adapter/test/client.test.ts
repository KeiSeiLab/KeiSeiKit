import { describe, it, expect, vi } from "vitest";
import { GmailClient, type GmailSurface } from "../src/client.js";

function makeSurface(): GmailSurface {
  return {
    list: vi.fn(async () => [{ id: "m1", threadId: "t1" }, { id: "m2" }]),
    get: vi.fn(async (id: string) => ({
      id,
      snippet: `snip-${id}`,
      payload: {
        headers: [
          { name: "Subject", value: `subj-${id}` },
          { name: "From", value: "alice@example.com" },
        ],
      },
    })),
    modify: vi.fn(async () => undefined),
    trash: vi.fn(async () => undefined),
  };
}

describe("GmailClient", () => {
  it("listUnread returns summarized messages", async () => {
    const surface = makeSurface();
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: surface });
    const out = await c.listUnread(10);
    expect(out).toHaveLength(2);
    expect(out[0]?.subject).toBe("subj-m1");
  });

  it("labelMessage calls modify with addIds only", async () => {
    const surface = makeSurface();
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: surface });
    await c.labelMessage("m1", "IMPORTANT");
    expect(surface.modify).toHaveBeenCalledWith("m1", ["IMPORTANT"], []);
  });

  it("archive removes INBOX label", async () => {
    const surface = makeSurface();
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: surface });
    await c.archive("m1");
    expect(surface.modify).toHaveBeenCalledWith("m1", [], ["INBOX"]);
  });
});
