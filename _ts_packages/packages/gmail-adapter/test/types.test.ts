import { describe, it, expect } from "vitest";
import { ListUnreadArgs, SearchArgs, LabelArgs, GetMessageArgs } from "../src/types.js";

describe("gmail schemas", () => {
  it("ListUnreadArgs defaults max to 20", () => {
    const r = ListUnreadArgs.safeParse({});
    expect(r.success).toBe(true);
    if (r.success) expect(r.data.max).toBe(20);
  });

  it("ListUnreadArgs rejects max=0", () => {
    const r = ListUnreadArgs.safeParse({ max: 0 });
    expect(r.success).toBe(false);
  });

  it("SearchArgs rejects empty query", () => {
    const r = SearchArgs.safeParse({ query: "" });
    expect(r.success).toBe(false);
  });

  it("LabelArgs requires both id and label", () => {
    expect(LabelArgs.safeParse({ id: "x" }).success).toBe(false);
    expect(LabelArgs.safeParse({ id: "x", label: "L" }).success).toBe(true);
  });

  it("GetMessageArgs requires non-empty id", () => {
    expect(GetMessageArgs.safeParse({ id: "" }).success).toBe(false);
  });
});
