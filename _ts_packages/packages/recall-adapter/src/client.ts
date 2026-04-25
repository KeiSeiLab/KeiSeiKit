// Minimal client for the Recall.ai v1 REST API.
// Docs: https://docs.recall.ai/reference (verified 2026-04).

export type FetchFn = typeof fetch;

export interface RecallClientConfig {
  apiKey: string;
  baseUrl?: string;
  fetchImpl?: FetchFn;
}

const DEFAULT_BASE_URL = "https://api.recall.ai/api/v1";

export class RecallClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly fetchImpl: FetchFn;

  constructor(cfg: RecallClientConfig) {
    if (!cfg.apiKey) throw new Error("RECALL_API_KEY is required");
    this.apiKey = cfg.apiKey;
    this.baseUrl = cfg.baseUrl ?? DEFAULT_BASE_URL;
    this.fetchImpl = cfg.fetchImpl ?? fetch;
  }

  async listBots(): Promise<unknown> {
    return this.request("GET", "/bot/");
  }

  async getBot(botId: string): Promise<unknown> {
    return this.request("GET", `/bot/${encodeURIComponent(botId)}/`);
  }

  async joinMeeting(meetingUrl: string, botName = "KeiSei"): Promise<unknown> {
    return this.request("POST", "/bot/", { meeting_url: meetingUrl, bot_name: botName });
  }

  async leaveMeeting(botId: string): Promise<unknown> {
    return this.request("POST", `/bot/${encodeURIComponent(botId)}/leave_call/`);
  }

  async getTranscript(botId: string): Promise<unknown> {
    return this.request("GET", `/bot/${encodeURIComponent(botId)}/transcript/`);
  }

  private async request(method: string, path: string, body?: unknown): Promise<unknown> {
    const headers: Record<string, string> = {
      Authorization: `Token ${this.apiKey}`,
      Accept: "application/json",
    };
    const init: RequestInit = { method, headers };
    if (body !== undefined) {
      headers["Content-Type"] = "application/json";
      init.body = JSON.stringify(body);
    }
    const res = await this.fetchImpl(`${this.baseUrl}${path}`, init);
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`recall ${method} ${path} -> ${res.status}: ${text}`);
    }
    return res.json();
  }
}
