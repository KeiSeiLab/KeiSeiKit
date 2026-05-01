import { describe, it, expect, vi } from "vitest";
import { GmailClient, type GmailSurface } from "../src/client.js";
import { buildGmailTools } from "../src/tools.js";

function mkSurface(): GmailSurface {
  return {
    list: vi.fn(async () => [{ id: "m1" }]),
    get: vi.fn(async () => ({ id: "m1", snippet: "hello", payload: { headers: [] } })),
    modify: vi.fn(async () => undefined),
    trash: vi.fn(async () => undefined),
  };
}

describe("gmail tool surface", () => {
  it("registers 6 tools", () => {
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: mkSurface() });
    const names = buildGmailTools(c).map((t) => t.name);
    expect(names).toEqual([
      "gmail_list_unread",
      "gmail_get_message",
      "gmail_search",
      "gmail_label_message",
      "gmail_archive",
      "gmail_trash",
    ]);
  });

  it("gmail_list_unread formats empty list", async () => {
    const surface: GmailSurface = { ...mkSurface(), list: vi.fn(async () => []) };
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: surface });
    const tool = buildGmailTools(c).find((t) => t.name === "gmail_list_unread");
    const out = await tool!.handler({});
    expect(out).toBe("No messages.");
  });

  it("gmail_trash returns ok string", async () => {
    const surface = mkSurface();
    const c = new GmailClient({ clientId: "", clientSecret: "", refreshToken: "", gmailSurface: surface });
    const tool = buildGmailTools(c).find((t) => t.name === "gmail_trash");
    const out = await tool!.handler({ id: "m1" });
    expect(out).toContain("trashed m1");
  });
});
