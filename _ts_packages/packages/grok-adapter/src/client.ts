// Minimal xAI Grok client. The public endpoints follow the OpenAI
// compatible shape: https://api.x.ai/v1/chat/completions and /images.
// Shapes verified against https://docs.x.ai/api (2026-04).

export type FetchFn = typeof fetch;

export interface GrokClientConfig {
  apiKey: string;
  baseUrl?: string;
  fetchImpl?: FetchFn;
  researchModel?: string;
  imageModel?: string;
}

const DEFAULT_BASE = "https://api.x.ai/v1";

export class GrokClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly fetchImpl: FetchFn;
  private readonly researchModel: string;
  private readonly imageModel: string;

  constructor(cfg: GrokClientConfig) {
    if (!cfg.apiKey) throw new Error("XAI_API_KEY is required");
    this.apiKey = cfg.apiKey;
    this.baseUrl = cfg.baseUrl ?? DEFAULT_BASE;
    this.fetchImpl = cfg.fetchImpl ?? fetch;
    this.researchModel = cfg.researchModel ?? "grok-4-heavy";
    this.imageModel = cfg.imageModel ?? "grok-2-image";
  }

  async deepResearch(query: string): Promise<string> {
    const body = {
      model: this.researchModel,
      messages: [{ role: "user", content: query }],
    };
    const data = (await this.postJson("/chat/completions", body)) as ChatCompletionResponse;
    const first = data.choices[0];
    return first?.message?.content ?? "";
  }

  async imageGenerate(prompt: string, pro = false): Promise<string[]> {
    const body = {
      model: this.imageModel,
      prompt,
      n: 1,
      quality: pro ? "pro" : "standard",
    };
    const data = (await this.postJson("/images/generations", body)) as ImageResponse;
    return (data.data ?? []).map((d) => d.url).filter((u): u is string => typeof u === "string");
  }

  private async postJson(path: string, body: unknown): Promise<unknown> {
    const res = await this.fetchImpl(`${this.baseUrl}${path}`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${this.apiKey}`,
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(`grok ${path} -> ${res.status}: ${text}`);
    }
    return res.json();
  }
}

interface ChatCompletionResponse {
  choices: Array<{ message: { content: string } }>;
}

interface ImageResponse {
  data: Array<{ url?: string }>;
}
