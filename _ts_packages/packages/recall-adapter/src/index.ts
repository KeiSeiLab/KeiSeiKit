import { RecallClient } from "./client.js";
import { buildRecallTools, type RecallTool } from "./tools.js";

export { RecallClient } from "./client.js";
export { buildRecallTools } from "./tools.js";
export type { RecallTool } from "./tools.js";

type Registrar = (tool: RecallTool) => void;

export function registerAdapter(register: Registrar): void {
  const apiKey = process.env["RECALL_API_KEY"];
  if (!apiKey) {
    throw new Error(
      "RECALL_API_KEY env var is missing; set it in ~/.claude/secrets/.env (RULE 0.8).",
    );
  }
  const client = new RecallClient({ apiKey });
  for (const tool of buildRecallTools(client)) register(tool);
}
