// Tool I/O types for the Telegram adapter. Kept separate so both the
// adapter and consumers can import schemas without pulling grammy.

import { z } from "zod";

export const TelegramChatRef = z.union([
  z.number().int(),
  z.string().min(1),
]);
export type TelegramChatRef = z.infer<typeof TelegramChatRef>;

export const SendTextArgs = z.object({
  chat: TelegramChatRef,
  text: z.string().min(1),
});
export type SendTextArgs = z.infer<typeof SendTextArgs>;

export const SendFileArgs = z.object({
  chat: TelegramChatRef,
  file: z.string().min(1),
  kind: z.enum(["document", "photo", "video"]).default("document"),
  caption: z.string().optional(),
});
export type SendFileArgs = z.infer<typeof SendFileArgs>;

export const SendVoiceArgs = z.object({
  chat: TelegramChatRef,
  file: z.string().min(1),
  caption: z.string().optional(),
});
export type SendVoiceArgs = z.infer<typeof SendVoiceArgs>;

export const ChatInfoArgs = z.object({
  chat: TelegramChatRef,
});
export type ChatInfoArgs = z.infer<typeof ChatInfoArgs>;

export interface ContactRecord {
  userId: number;
  firstName: string;
  lastName?: string | undefined;
  username?: string | undefined;
  lastSeen?: number | undefined;
}

export interface GroupRecord {
  chatId: number;
  title: string;
  type: string;
  lastMsg?: number | undefined;
}
