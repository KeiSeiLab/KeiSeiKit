import { describe, it, expect } from "vitest";
import { TelegramClient } from "../src/client.js";

describe("TelegramClient cache", () => {
  it("rejects empty token at construction", () => {
    expect(() => new TelegramClient({ token: "" })).toThrow(/TELEGRAM_BOT_TOKEN/);
  });

  it("listGroups is empty by default", () => {
    const c = new TelegramClient({ token: "123:ABC" });
    expect(c.listGroups()).toEqual([]);
  });

  it("listContacts is empty by default", () => {
    const c = new TelegramClient({ token: "123:ABC" });
    expect(c.listContacts()).toEqual([]);
  });

  it("_seedContact and _seedGroup populate caches", () => {
    const c = new TelegramClient({ token: "123:ABC" });
    c._seedContact({ userId: 99, firstName: "Alice" });
    c._seedGroup({ chatId: -100, title: "Test", type: "supergroup" });
    expect(c.listContacts()).toHaveLength(1);
    expect(c.listGroups()).toHaveLength(1);
  });
});
