// Recall.ai tool definitions. Each tool wraps one client method and returns
// JSON-stringified output for the MCP transport.

import { z } from "zod";
import { RecallClient } from "./client.js";

export interface RecallTool {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

const BotIdArgs = z.object({ bot_id: z.string().min(1) });
const JoinArgs = z.object({
  meeting_url: z.string().url(),
  bot_name: z.string().optional(),
});

export function buildRecallTools(client: RecallClient): RecallTool[] {
  return [
    {
      name: "zoom_status",
      description: "Status of a deployed Recall.ai bot (bot_id required).",
      inputSchema: BotIdArgs,
      handler: async (raw) => {
        const args = BotIdArgs.parse(raw);
        return pretty(await client.getBot(args.bot_id));
      },
    },
    {
      name: "zoom_bots",
      description: "List all Recall.ai bots for this account.",
      inputSchema: z.object({}),
      handler: async () => pretty(await client.listBots()),
    },
    {
      name: "zoom_join",
      description: "Deploy a Recall.ai bot to a meeting URL.",
      inputSchema: JoinArgs,
      handler: async (raw) => {
        const args = JoinArgs.parse(raw);
        return pretty(await client.joinMeeting(args.meeting_url, args.bot_name));
      },
    },
    {
      name: "zoom_leave",
      description: "Recall an active bot from a meeting.",
      inputSchema: BotIdArgs,
      handler: async (raw) => {
        const args = BotIdArgs.parse(raw);
        return pretty(await client.leaveMeeting(args.bot_id));
      },
    },
    {
      name: "zoom_chat",
      description: "Fetch transcript for a bot's meeting.",
      inputSchema: BotIdArgs,
      handler: async (raw) => {
        const args = BotIdArgs.parse(raw);
        return pretty(await client.getTranscript(args.bot_id));
      },
    },
  ];
}

function pretty(x: unknown): string {
  return JSON.stringify(x, null, 2);
}
