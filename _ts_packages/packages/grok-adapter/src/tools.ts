import { z } from "zod";
import { GrokClient } from "./client.js";

export interface GrokTool {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

const ResearchArgs = z.object({ query: z.string().min(1) });
const ImagineArgs = z.object({
  prompt: z.string().min(1),
  quality: z.enum(["standard", "pro"]).default("standard"),
});

export function buildGrokTools(client: GrokClient): GrokTool[] {
  return [
    {
      name: "grok_research",
      description: "Deep research via Grok heavy model. Returns assistant message content.",
      inputSchema: ResearchArgs,
      handler: async (raw) => {
        const args = ResearchArgs.parse(raw);
        return client.deepResearch(args.query);
      },
    },
    {
      name: "grok_imagine",
      description: "Generate an image from a prompt via Grok Imagine.",
      inputSchema: ImagineArgs,
      handler: async (raw) => {
        const args = ImagineArgs.parse(raw);
        const urls = await client.imageGenerate(args.prompt, args.quality === "pro");
        if (urls.length === 0) return "No image returned.";
        return urls.join("\n");
      },
    },
  ];
}
