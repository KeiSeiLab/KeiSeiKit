// YouTube Data API v3 tool I/O types.

import { z } from "zod";

export const SubscriptionsArgs = z.object({
  max: z.number().int().positive().max(50).default(25),
});
export type SubscriptionsArgs = z.infer<typeof SubscriptionsArgs>;

export const NewVideosArgs = z.object({
  channel_id: z.string().min(1),
  since: z.string().optional(),
  max: z.number().int().positive().max(50).default(10),
});
export type NewVideosArgs = z.infer<typeof NewVideosArgs>;

export const SearchArgs = z.object({
  query: z.string().min(1),
  max: z.number().int().positive().max(50).default(10),
});
export type SearchArgs = z.infer<typeof SearchArgs>;

export const VideoIdArgs = z.object({
  video_id: z.string().min(1),
});
export type VideoIdArgs = z.infer<typeof VideoIdArgs>;

export interface VideoSummary {
  videoId: string;
  title?: string | undefined;
  channel?: string | undefined;
  publishedAt?: string | undefined;
}

export interface VideoStats {
  videoId: string;
  viewCount?: string | undefined;
  likeCount?: string | undefined;
  commentCount?: string | undefined;
}

export interface TranscriptLine {
  text: string;
  offset: number;
  duration: number;
}
