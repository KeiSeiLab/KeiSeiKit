// Tool definitions for the Telegram adapter. Each tool is a small wrapper
// around TelegramClient + a zod schema, returning a string to MCP.

import { z } from "zod";
import { TelegramClient } from "./client.js";
import {
  ChatInfoArgs,
  SendFileArgs,
  SendTextArgs,
  SendVoiceArgs,
} from "./types.js";

export interface TelegramTool {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

export function buildTelegramTools(client: TelegramClient): TelegramTool[] {
  return [
    {
      name: "telegram_status",
      description: "Telegram bot identity and connectivity.",
      inputSchema: z.object({}),
      handler: async () => {
        const s = await client.status();
        return `bot=@${s.username} id=${s.id}`;
      },
    },
    {
      name: "telegram_groups",
      description: "List groups the bot has observed.",
      inputSchema: z.object({}),
      handler: async () => {
        const gs = client.listGroups();
        if (gs.length === 0) return "No groups tracked yet.";
        return gs.map((g) => `${g.chatId} | ${g.title} [${g.type}]`).join("\n");
      },
    },
    {
      name: "telegram_contacts",
      description: "List known Telegram contacts.",
      inputSchema: z.object({}),
      handler: async () => {
        const cs = client.listContacts();
        if (cs.length === 0) return "No contacts yet.";
        return cs.map(formatContact).join("\n");
      },
    },
    {
      name: "telegram_chat_info",
      description: "Chat metadata for a given chat ID or @username.",
      inputSchema: ChatInfoArgs,
      handler: async (raw) => {
        const args = ChatInfoArgs.parse(raw);
        const info = await client.chatInfo(args.chat);
        return `id=${info.id}\ntitle=${info.title}\ntype=${info.type}`;
      },
    },
    {
      name: "telegram_send",
      description: "Send a text message.",
      inputSchema: SendTextArgs,
      handler: async (raw) => {
        const args = SendTextArgs.parse(raw);
        const id = await client.sendText(args.chat, args.text);
        return `sent message_id=${id}`;
      },
    },
    {
      name: "telegram_send_file",
      description: "Send a document, photo, or video file.",
      inputSchema: SendFileArgs,
      handler: async (raw) => {
        const args = SendFileArgs.parse(raw);
        const id = await dispatchFile(client, args);
        return `sent ${args.kind} message_id=${id}`;
      },
    },
    {
      name: "telegram_send_voice",
      description: "Send a pre-recorded voice note file.",
      inputSchema: SendVoiceArgs,
      handler: async (raw) => {
        const args = SendVoiceArgs.parse(raw);
        const id = await client.sendVoice(args.chat, args.file, args.caption);
        return `sent voice message_id=${id}`;
      },
    },
  ];
}

function formatContact(c: {
  userId: number;
  firstName: string;
  lastName?: string | undefined;
  username?: string | undefined;
}): string {
  const name = c.lastName ? `${c.firstName} ${c.lastName}` : c.firstName;
  const handle = c.username ? ` @${c.username}` : "";
  return `${c.userId} | ${name}${handle}`;
}

async function dispatchFile(
  client: TelegramClient,
  args: { chat: string | number; file: string; kind: "document" | "photo" | "video"; caption?: string | undefined },
): Promise<number> {
  if (args.kind === "photo") return client.sendPhoto(args.chat, args.file, args.caption);
  if (args.kind === "video") return client.sendVideo(args.chat, args.file, args.caption);
  return client.sendDocument(args.chat, args.file, args.caption);
}
