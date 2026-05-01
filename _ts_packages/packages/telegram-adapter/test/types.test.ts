import { describe, it, expect } from "vitest";
import { SendTextArgs, SendFileArgs, SendVoiceArgs, ChatInfoArgs } from "../src/types.js";

describe("zod schemas", () => {
  it("SendTextArgs accepts numeric chat id", () => {
    const r = SendTextArgs.safeParse({ chat: 12345, text: "hi" });
    expect(r.success).toBe(true);
  });

  it("SendTextArgs accepts string chat handle", () => {
    const r = SendTextArgs.safeParse({ chat: "@username", text: "hi" });
    expect(r.success).toBe(true);
  });

  it("SendTextArgs rejects empty text", () => {
    const r = SendTextArgs.safeParse({ chat: 1, text: "" });
    expect(r.success).toBe(false);
  });

  it("SendFileArgs defaults kind to document", () => {
    const r = SendFileArgs.safeParse({ chat: 1, file: "/x" });
    expect(r.success).toBe(true);
    if (r.success) expect(r.data.kind).toBe("document");
  });

  it("SendFileArgs rejects unknown kind", () => {
    const r = SendFileArgs.safeParse({ chat: 1, file: "/x", kind: "sticker" });
    expect(r.success).toBe(false);
  });

  it("SendVoiceArgs requires file path", () => {
    const r = SendVoiceArgs.safeParse({ chat: 1 });
    expect(r.success).toBe(false);
  });

  it("ChatInfoArgs requires chat", () => {
    const r = ChatInfoArgs.safeParse({});
    expect(r.success).toBe(false);
  });
});
