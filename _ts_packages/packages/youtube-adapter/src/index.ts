import { YouTubeClient } from "./client.js";
import { buildYouTubeTools, type YouTubeTool } from "./tools.js";

export { YouTubeClient } from "./client.js";
export { buildYouTubeTools } from "./tools.js";
export type { YouTubeTool } from "./tools.js";
export * from "./types.js";

type Registrar = (tool: YouTubeTool) => void;

export function registerAdapter(register: Registrar): void {
  const apiKey = process.env["YOUTUBE_API_KEY"];
  if (!apiKey) {
    throw new Error(
      "YOUTUBE_API_KEY env var is missing; set it in ~/.claude/secrets/.env (RULE 0.8).",
    );
  }
  const client = new YouTubeClient({ apiKey });
  for (const tool of buildYouTubeTools(client)) register(tool);
}
