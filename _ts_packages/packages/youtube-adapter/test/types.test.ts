import { describe, it, expect } from "vitest";
import { SubscriptionsArgs, NewVideosArgs, SearchArgs, VideoIdArgs } from "../src/types.js";

describe("youtube schemas", () => {
  it("SubscriptionsArgs defaults max to 25", () => {
    const r = SubscriptionsArgs.safeParse({});
    expect(r.success).toBe(true);
    if (r.success) expect(r.data.max).toBe(25);
  });

  it("SubscriptionsArgs rejects max > 50", () => {
    const r = SubscriptionsArgs.safeParse({ max: 51 });
    expect(r.success).toBe(false);
  });

  it("NewVideosArgs requires channel_id", () => {
    expect(NewVideosArgs.safeParse({}).success).toBe(false);
    expect(NewVideosArgs.safeParse({ channel_id: "UC1" }).success).toBe(true);
  });

  it("SearchArgs rejects empty query", () => {
    expect(SearchArgs.safeParse({ query: "" }).success).toBe(false);
  });

  it("VideoIdArgs requires non-empty id", () => {
    expect(VideoIdArgs.safeParse({ video_id: "" }).success).toBe(false);
    expect(VideoIdArgs.safeParse({ video_id: "abc" }).success).toBe(true);
  });
});
