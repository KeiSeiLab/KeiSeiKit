import { GmailClient } from "./client.js";
import { buildGmailTools, type GmailTool } from "./tools.js";

export { GmailClient } from "./client.js";
export { buildGmailTools } from "./tools.js";
export type { GmailTool } from "./tools.js";
export * from "./types.js";

type Registrar = (tool: GmailTool) => void;

export function registerAdapter(register: Registrar): void {
  const clientId = process.env["GMAIL_CLIENT_ID"];
  const clientSecret = process.env["GMAIL_CLIENT_SECRET"];
  const refreshToken = process.env["GMAIL_REFRESH_TOKEN"];
  if (!clientId || !clientSecret || !refreshToken) {
    throw new Error(
      "GMAIL_{CLIENT_ID,CLIENT_SECRET,REFRESH_TOKEN} env vars required; see ~/.claude/secrets/.env (RULE 0.8).",
    );
  }
  const client = new GmailClient({ clientId, clientSecret, refreshToken });
  for (const tool of buildGmailTools(client)) register(tool);
}
