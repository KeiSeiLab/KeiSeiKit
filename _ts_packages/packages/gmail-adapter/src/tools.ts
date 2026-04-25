import { z } from "zod";
import { GmailClient } from "./client.js";
import {
  GetMessageArgs,
  LabelArgs,
  ListUnreadArgs,
  ModifyOnlyArgs,
  SearchArgs,
  type MessageSummary,
} from "./types.js";

export interface GmailTool {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

export function buildGmailTools(client: GmailClient): GmailTool[] {
  return [
    {
      name: "gmail_list_unread",
      description: "List unread messages (up to 500).",
      inputSchema: ListUnreadArgs,
      handler: async (raw) => {
        const args = ListUnreadArgs.parse(raw);
        return formatList(await client.listUnread(args.max));
      },
    },
    {
      name: "gmail_get_message",
      description: "Fetch one message by id; returns headers + snippet.",
      inputSchema: GetMessageArgs,
      handler: async (raw) => {
        const args = GetMessageArgs.parse(raw);
        return formatOne(await client.getMessage(args.id));
      },
    },
    {
      name: "gmail_search",
      description: "Search mailbox using Gmail operators (e.g. 'from:alice has:attachment').",
      inputSchema: SearchArgs,
      handler: async (raw) => {
        const args = SearchArgs.parse(raw);
        return formatList(await client.search(args.query, args.max));
      },
    },
    {
      name: "gmail_label_message",
      description: "Apply a Gmail label id to a message.",
      inputSchema: LabelArgs,
      handler: async (raw) => {
        const args = LabelArgs.parse(raw);
        await client.labelMessage(args.id, args.label);
        return `labeled ${args.id} with ${args.label}`;
      },
    },
    {
      name: "gmail_archive",
      description: "Archive a message (removes INBOX label).",
      inputSchema: ModifyOnlyArgs,
      handler: async (raw) => {
        const args = ModifyOnlyArgs.parse(raw);
        await client.archive(args.id);
        return `archived ${args.id}`;
      },
    },
    {
      name: "gmail_trash",
      description: "Move a message to Trash.",
      inputSchema: ModifyOnlyArgs,
      handler: async (raw) => {
        const args = ModifyOnlyArgs.parse(raw);
        await client.trash(args.id);
        return `trashed ${args.id}`;
      },
    },
  ];
}

function formatList(msgs: MessageSummary[]): string {
  if (msgs.length === 0) return "No messages.";
  return msgs.map(formatOne).join("\n---\n");
}

function formatOne(m: MessageSummary): string {
  const parts = [`id: ${m.id}`];
  if (m.subject) parts.push(`subject: ${m.subject}`);
  if (m.from) parts.push(`from: ${m.from}`);
  if (m.date) parts.push(`date: ${m.date}`);
  if (m.snippet) parts.push(`snippet: ${m.snippet}`);
  return parts.join("\n");
}
