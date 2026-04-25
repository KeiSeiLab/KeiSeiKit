import { describe, it, expect, vi } from "vitest";
import { YouTubeClient, type YouTubeSurface } from "../src/client.js";

function makeSurface(): YouTubeSurface {
  return {
    subscriptions: vi.fn(async () => [{ videoId: "v1", title: "Sub channel", channel: "c1" }]),
    channelVideos: vi.fn(async () => [{ videoId: "v2", title: "Latest" }]),
    search: vi.fn(async () => [{ videoId: "v3", title: "Result" }]),
    stats: vi.fn(async () => ({ videoId: "v1", viewCount: "100", likeCount: "5", commentCount: "1" })),
  };
}

describe("YouTubeClient", () => {
  it("subscriptions delegates to surface", async () => {
    const s = makeSurface();
    const c = new YouTubeClient({ apiKey: "k", surface: s });
    const out = await c.subscriptions(10);
    expect(out[0]?.videoId).toBe("v1");
    expect(s.subscriptions).toHaveBeenCalledWith(10);
  });

  it("transcript uses injected fn", async () => {
    const c = new YouTubeClient({
      apiKey: "k",
      surface: makeSurface(),
      transcriptFn: async () => [{ text: "hi", offset: 0, duration: 1 }],
    });
    const out = await c.transcript("vid");
    expect(out).toHaveLength(1);
  });

  it("stats returns video statistics", async () => {
    const c = new YouTubeClient({ apiKey: "k", surface: makeSurface() });
    const out = await c.stats("v1");
    expect(out.viewCount).toBe("100");
  });
});
