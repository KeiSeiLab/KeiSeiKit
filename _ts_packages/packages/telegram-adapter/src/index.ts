// Public entry: exports registerAdapter() for the MCP server loader,
// plus the class + tool builder for programmatic use.

import { TelegramClient } from "./client.js";
import { buildTelegramTools, type TelegramTool } from "./tools.js";

export { TelegramClient } from "./client.js";
export { buildTelegramTools } from "./tools.js";
export type { TelegramTool } from "./tools.js";
export * from "./types.js";

type Registrar = (tool: TelegramTool) => void;

export function registerAdapter(register: Registrar): void {
  const token = process.env["TELEGRAM_BOT_TOKEN"];
  if (!token) {
    throw new Error(
      "TELEGRAM_BOT_TOKEN env var is missing; set it in ~/.claude/secrets/.env (RULE 0.8).",
    );
  }
  const client = new TelegramClient({ token });
  for (const tool of buildTelegramTools(client)) register(tool);
}
