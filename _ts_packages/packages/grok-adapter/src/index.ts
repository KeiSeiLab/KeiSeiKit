import { GrokClient } from "./client.js";
import { buildGrokTools, type GrokTool } from "./tools.js";

export { GrokClient } from "./client.js";
export { buildGrokTools } from "./tools.js";
export type { GrokTool } from "./tools.js";

type Registrar = (tool: GrokTool) => void;

export function registerAdapter(register: Registrar): void {
  const apiKey = process.env["XAI_API_KEY"];
  if (!apiKey) {
    throw new Error(
      "XAI_API_KEY env var is missing; set it in ~/.claude/secrets/.env (RULE 0.8).",
    );
  }
  const client = new GrokClient({ apiKey });
  for (const tool of buildGrokTools(client)) register(tool);
}
