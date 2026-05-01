// Gmail API client via googleapis. One class that owns an OAuth2 client
// plus a gmail.users surface. All methods return plain data (no gapi types
// leak outward). Tests inject a mock surface via the `gmailSurface` param.

import { google } from "googleapis";
import type { MessageSummary } from "./types.js";

export interface GmailClientConfig {
  clientId: string;
  clientSecret: string;
  refreshToken: string;
  gmailSurface?: GmailSurface;
}

// Narrow shape we actually use. Everything googleapis exposes is optional.
export interface GmailSurface {
  list: (q: string | undefined, max: number) => Promise<Array<{ id?: string | null; threadId?: string | null }>>;
  get: (id: string) => Promise<{ id?: string | null; threadId?: string | null; snippet?: string | null; payload?: { headers?: Array<{ name?: string | null; value?: string | null }> | null } | null }>;
  modify: (id: string, addIds: string[], removeIds: string[]) => Promise<void>;
  trash: (id: string) => Promise<void>;
}

export class GmailClient {
  private readonly surface: GmailSurface;

  constructor(cfg: GmailClientConfig) {
    this.surface = cfg.gmailSurface ?? buildDefaultSurface(cfg);
  }

  async listUnread(max: number): Promise<MessageSummary[]> {
    const ids = await this.surface.list("is:unread", max);
    return Promise.all(ids.map(async (row) => this.summarize(row.id ?? "")));
  }

  async search(query: string, max: number): Promise<MessageSummary[]> {
    const ids = await this.surface.list(query, max);
    return Promise.all(ids.map(async (row) => this.summarize(row.id ?? "")));
  }

  async getMessage(id: string): Promise<MessageSummary> {
    return this.summarize(id);
  }

  async labelMessage(id: string, label: string): Promise<void> {
    await this.surface.modify(id, [label], []);
  }

  async archive(id: string): Promise<void> {
    await this.surface.modify(id, [], ["INBOX"]);
  }

  async trash(id: string): Promise<void> {
    await this.surface.trash(id);
  }

  private async summarize(id: string): Promise<MessageSummary> {
    if (!id) return { id: "" };
    const msg = await this.surface.get(id);
    const headers = msg.payload?.headers ?? [];
    const pick = (name: string): string | undefined => headers.find((h) => h.name?.toLowerCase() === name)?.value ?? undefined;
    return {
      id: msg.id ?? id,
      threadId: msg.threadId ?? undefined,
      subject: pick("subject"),
      from: pick("from"),
      date: pick("date"),
      snippet: msg.snippet ?? undefined,
    };
  }
}

function buildDefaultSurface(cfg: GmailClientConfig): GmailSurface {
  if (!cfg.clientId || !cfg.clientSecret || !cfg.refreshToken) {
    throw new Error("GMAIL_CLIENT_ID, GMAIL_CLIENT_SECRET, GMAIL_REFRESH_TOKEN all required");
  }
  const oauth = new google.auth.OAuth2(cfg.clientId, cfg.clientSecret);
  oauth.setCredentials({ refresh_token: cfg.refreshToken });
  const gmail = google.gmail({ version: "v1", auth: oauth });
  return {
    list: async (q, max) => {
      const params: { userId: string; maxResults: number; q?: string } = { userId: "me", maxResults: max };
      if (q !== undefined) params.q = q;
      const res = await gmail.users.messages.list(params);
      const items = res.data.messages ?? [];
      return items.map((m: { id?: string | null; threadId?: string | null }) => ({
        id: m.id ?? null,
        threadId: m.threadId ?? null,
      }));
    },
    get: async (id) => {
      const res = await gmail.users.messages.get({ userId: "me", id, format: "metadata" });
      return res.data;
    },
    modify: async (id, addIds, removeIds) => {
      await gmail.users.messages.modify({ userId: "me", id, requestBody: { addLabelIds: addIds, removeLabelIds: removeIds } });
    },
    trash: async (id) => {
      await gmail.users.messages.trash({ userId: "me", id });
    },
  };
}
