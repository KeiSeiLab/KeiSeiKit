import { describe, it, expect, vi } from "vitest";
import { YouTubeClient, type YouTubeSurface } from "../src/client.js";
import { buildYouTubeTools } from "../src/tools.js";

function makeSurface(): YouTubeSurface {
  return {
    subscriptions: vi.fn(async () => []),
    channelVideos: vi.fn(async () => []),
    search: vi.fn(async () => [{ videoId: "v1", title: "T", channel: "C" }]),
    stats: vi.fn(async () => ({ videoId: "v1", viewCount: "9" })),
  };
}

describe("youtube tool surface", () => {
  it("registers 5 tools", () => {
    const c = new YouTubeClient({ apiKey: "k", surface: makeSurface(), transcriptFn: async () => [] });
    const names = buildYouTubeTools(c).map((t) => t.name);
    expect(names).toEqual([
      "youtube_subscriptions",
      "youtube_new_videos",
      "youtube_search",
      "youtube_transcript",
      "youtube_video_stats",
    ]);
  });

  it("youtube_subscriptions handles empty list", async () => {
    const c = new YouTubeClient({ apiKey: "k", surface: makeSurface(), transcriptFn: async () => [] });
    const tool = buildYouTubeTools(c).find((t) => t.name === "youtube_subscriptions");
    const out = await tool!.handler({});
    expect(out).toBe("No results.");
  });

  it("youtube_search formats result line", async () => {
    const c = new YouTubeClient({ apiKey: "k", surface: makeSurface(), transcriptFn: async () => [] });
    const tool = buildYouTubeTools(c).find((t) => t.name === "youtube_search");
    const out = await tool!.handler({ query: "rust" });
    expect(out).toContain("v1");
    expect(out).toContain("T");
  });

  it("youtube_transcript joins lines with spaces", async () => {
    const c = new YouTubeClient({
      apiKey: "k",
      surface: makeSurface(),
      transcriptFn: async () => [
        { text: "hello", offset: 0, duration: 1 },
        { text: "world", offset: 1, duration: 1 },
      ],
    });
    const tool = buildYouTubeTools(c).find((t) => t.name === "youtube_transcript");
    const out = await tool!.handler({ video_id: "x" });
    expect(out).toBe("hello world");
  });
});
