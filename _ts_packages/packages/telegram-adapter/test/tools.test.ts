import { describe, it, expect } from "vitest";
import { TelegramClient } from "../src/client.js";
import { buildTelegramTools } from "../src/tools.js";

describe("telegram tool surface", () => {
  const c = new TelegramClient({ token: "123:ABC" });
  const tools = buildTelegramTools(c);

  it("exposes 7 tools total", () => {
    expect(tools.map((t) => t.name)).toEqual([
      "telegram_status",
      "telegram_groups",
      "telegram_contacts",
      "telegram_chat_info",
      "telegram_send",
      "telegram_send_file",
      "telegram_send_voice",
    ]);
  });

  it("telegram_groups returns placeholder text when empty", async () => {
    const tool = tools.find((t) => t.name === "telegram_groups");
    const out = await tool!.handler({});
    expect(out).toContain("No groups");
  });

  it("telegram_contacts formats seeded contact", async () => {
    c._seedContact({ userId: 42, firstName: "Bob", username: "bobby" });
    const tool = tools.find((t) => t.name === "telegram_contacts");
    const out = await tool!.handler({});
    expect(out).toContain("42");
    expect(out).toContain("Bob");
    expect(out).toContain("@bobby");
  });

  it("telegram_send rejects missing args via schema", async () => {
    const tool = tools.find((t) => t.name === "telegram_send");
    await expect(tool!.handler({})).rejects.toBeTruthy();
  });

  it("tool descriptions are non-empty", () => {
    for (const t of tools) expect(t.description.length).toBeGreaterThan(0);
  });
});
