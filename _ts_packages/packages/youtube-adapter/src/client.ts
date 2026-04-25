// YouTube Data API v3 client wrapper. The surface is intentionally narrow —
// subscriptions list, search, videos.list(statistics), plus a transcript
// helper using the `youtube-transcript` package.

import { google } from "googleapis";
import type { TranscriptLine, VideoStats, VideoSummary } from "./types.js";

export interface YouTubeClientConfig {
  apiKey: string;
  surface?: YouTubeSurface;
  transcriptFn?: TranscriptFn;
}

export type TranscriptFn = (videoId: string) => Promise<TranscriptLine[]>;

export interface YouTubeSurface {
  subscriptions: (max: number) => Promise<VideoSummary[]>;
  channelVideos: (channelId: string, since: string | undefined, max: number) => Promise<VideoSummary[]>;
  search: (query: string, max: number) => Promise<VideoSummary[]>;
  stats: (videoId: string) => Promise<VideoStats>;
}

export class YouTubeClient {
  private readonly surface: YouTubeSurface;
  private readonly transcriptFn: TranscriptFn;

  constructor(cfg: YouTubeClientConfig) {
    this.surface = cfg.surface ?? buildDefaultSurface(cfg.apiKey);
    this.transcriptFn = cfg.transcriptFn ?? defaultTranscriptFn;
  }

  subscriptions(max: number): Promise<VideoSummary[]> {
    return this.surface.subscriptions(max);
  }

  newVideos(channelId: string, since: string | undefined, max: number): Promise<VideoSummary[]> {
    return this.surface.channelVideos(channelId, since, max);
  }

  search(query: string, max: number): Promise<VideoSummary[]> {
    return this.surface.search(query, max);
  }

  stats(videoId: string): Promise<VideoStats> {
    return this.surface.stats(videoId);
  }

  transcript(videoId: string): Promise<TranscriptLine[]> {
    return this.transcriptFn(videoId);
  }
}

interface TranscriptModule {
  YoutubeTranscript: {
    fetchTranscript: (videoId: string) => Promise<Array<{ text: string; offset: number; duration: number }>>;
  };
}

async function defaultTranscriptFn(videoId: string): Promise<TranscriptLine[]> {
  // Deferred import: the upstream package ships dual-module with a broken
  // CJS entry, so eager `import` at top-level fails under ESM + vitest.
  const mod = (await import("youtube-transcript")) as unknown as TranscriptModule;
  const rows = await mod.YoutubeTranscript.fetchTranscript(videoId);
  return rows.map((r) => ({ text: r.text, offset: r.offset, duration: r.duration }));
}

function buildDefaultSurface(apiKey: string): YouTubeSurface {
  if (!apiKey) throw new Error("YOUTUBE_API_KEY is required");
  const yt = google.youtube({ version: "v3", auth: apiKey });
  return {
    subscriptions: async (max) => {
      const res = await yt.subscriptions.list({ part: ["snippet"], mine: true, maxResults: max });
      return (res.data.items ?? []).map(itemToSummary);
    },
    channelVideos: async (channelId, since, max) => {
      const res = await yt.search.list({
        part: ["snippet"],
        channelId,
        order: "date",
        maxResults: max,
        ...(since !== undefined ? { publishedAfter: since } : {}),
      });
      return (res.data.items ?? []).map(itemToSummary);
    },
    search: async (query, max) => {
      const res = await yt.search.list({ part: ["snippet"], q: query, maxResults: max });
      return (res.data.items ?? []).map(itemToSummary);
    },
    stats: async (videoId) => {
      const res = await yt.videos.list({ part: ["statistics"], id: [videoId] });
      const s = res.data.items?.[0]?.statistics ?? {};
      return {
        videoId,
        viewCount: s.viewCount ?? undefined,
        likeCount: s.likeCount ?? undefined,
        commentCount: s.commentCount ?? undefined,
      };
    },
  };
}

function itemToSummary(item: { id?: { videoId?: string | null } | string | null; snippet?: { title?: string | null; channelTitle?: string | null; resourceId?: { videoId?: string | null } | null; publishedAt?: string | null } | null }): VideoSummary {
  const vid = typeof item.id === "object" && item.id !== null
    ? (item.id.videoId ?? "")
    : (item.snippet?.resourceId?.videoId ?? "");
  return {
    videoId: vid,
    title: item.snippet?.title ?? undefined,
    channel: item.snippet?.channelTitle ?? undefined,
    publishedAt: item.snippet?.publishedAt ?? undefined,
  };
}
