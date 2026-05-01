import { z } from "zod";
import { YouTubeClient } from "./client.js";
import {
  NewVideosArgs,
  SearchArgs,
  SubscriptionsArgs,
  VideoIdArgs,
  type TranscriptLine,
  type VideoStats,
  type VideoSummary,
} from "./types.js";

export interface YouTubeTool {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

export function buildYouTubeTools(client: YouTubeClient): YouTubeTool[] {
  return [
    {
      name: "youtube_subscriptions",
      description: "List the authenticated user's channel subscriptions.",
      inputSchema: SubscriptionsArgs,
      handler: async (raw) => {
        const args = SubscriptionsArgs.parse(raw);
        return formatList(await client.subscriptions(args.max));
      },
    },
    {
      name: "youtube_new_videos",
      description: "Latest videos from a given channel (optional --since ISO8601).",
      inputSchema: NewVideosArgs,
      handler: async (raw) => {
        const args = NewVideosArgs.parse(raw);
        return formatList(await client.newVideos(args.channel_id, args.since, args.max));
      },
    },
    {
      name: "youtube_search",
      description: "Search YouTube for a query string.",
      inputSchema: SearchArgs,
      handler: async (raw) => {
        const args = SearchArgs.parse(raw);
        return formatList(await client.search(args.query, args.max));
      },
    },
    {
      name: "youtube_transcript",
      description: "Fetch the transcript (captions) of a video as plain text.",
      inputSchema: VideoIdArgs,
      handler: async (raw) => {
        const args = VideoIdArgs.parse(raw);
        return formatTranscript(await client.transcript(args.video_id));
      },
    },
    {
      name: "youtube_video_stats",
      description: "View/like/comment counts for a given video.",
      inputSchema: VideoIdArgs,
      handler: async (raw) => {
        const args = VideoIdArgs.parse(raw);
        return formatStats(await client.stats(args.video_id));
      },
    },
  ];
}

function formatList(items: VideoSummary[]): string {
  if (items.length === 0) return "No results.";
  return items.map((v) => `${v.videoId} | ${v.channel ?? "?"} | ${v.title ?? "?"}`).join("\n");
}

function formatTranscript(lines: TranscriptLine[]): string {
  if (lines.length === 0) return "No transcript available.";
  return lines.map((l) => l.text).join(" ");
}

function formatStats(s: VideoStats): string {
  return [
    `video: ${s.videoId}`,
    `views: ${s.viewCount ?? "?"}`,
    `likes: ${s.likeCount ?? "?"}`,
    `comments: ${s.commentCount ?? "?"}`,
  ].join("\n");
}
