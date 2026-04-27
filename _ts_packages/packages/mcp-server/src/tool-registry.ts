// Tool registry: auto-register each Rust primitive CLI as one MCP tool.
// Plus the meta-tool kei(query) that routes natural language via kei-router.

import { z } from "zod";
import { jsonArgsToCli, RustBridge } from "./rust-bridge.js";
import { ToolNotFoundError } from "./errors.js";

export interface ToolDefinition {
  name: string;
  description: string;
  inputSchema: z.ZodObject<Record<string, z.ZodTypeAny>>;
  handler: (args: Record<string, unknown>) => Promise<string>;
}

// Primitive CLIs exposed 1:1 as tools. Each Rust binary accepts flags as
// --kebab-case; tool names stay snake_case for MCP convention.
export const RUST_PRIMITIVE_TOOLS: ReadonlyArray<{ binary: string; desc: string }> = [
  { binary: "kei-ledger", desc: "Append-only event ledger; sign, verify, append, list." },
  { binary: "kei-memory", desc: "Local key-value memory store with SQLite backend." },
  { binary: "kei-store", desc: "Content-addressed blob store." },
  { binary: "kei-graph-check", desc: "Validate graph invariants in a project." },
  { binary: "kei-refactor-engine", desc: "Apply structural refactors from a plan file." },
  { binary: "kei-conflict-scan", desc: "Scan a tree for merge/rebase conflict markers." },
  { binary: "kei-migrate", desc: "Run schema or directory migrations." },
  { binary: "kei-changelog", desc: "Generate changelog from commit/tag history." },
  { binary: "ssh-check", desc: "Validate SSH config + known_hosts consistency." },
  { binary: "firewall-diff", desc: "Diff two firewall rule dumps." },
  { binary: "tokens-sync", desc: "Sync design tokens from Figma export to code." },
  { binary: "visual-diff", desc: "Compare rendered screenshots pixel-wise." },
  { binary: "mock-render", desc: "Render HTML mock templates for preview." },
  { binary: "kei-gdrive-import", desc: "Classify a Google Drive folder as PROJECT/AMBIGUOUS/NOT-A-PROJECT/ALREADY-REPO via 8-marker scoring (Cargo.toml, package.json, pyproject.toml, go.mod, pom.xml, build.gradle, Gemfile, composer.json). Subcommands: classify <path> [--remote], scan-tree <root> [--remote]. Remote mode shells to rclone lsf — no download." },
];

export function buildRegistry(bridge: RustBridge): Map<string, ToolDefinition> {
  const map = new Map<string, ToolDefinition>();
  for (const t of RUST_PRIMITIVE_TOOLS) map.set(t.binary, wrapPrimitive(bridge, t));
  map.set("kei", buildKeiMetaTool(bridge));
  return map;
}

function wrapPrimitive(
  bridge: RustBridge,
  entry: { binary: string; desc: string },
): ToolDefinition {
  return {
    name: entry.binary,
    description: entry.desc,
    inputSchema: z.object({ args: z.record(z.unknown()).optional() }),
    handler: async (rawArgs) => {
      const parsed = (rawArgs["args"] as Record<string, unknown> | undefined) ?? {};
      const cli = jsonArgsToCli(parsed);
      const result = await bridge.call({ binary: entry.binary, args: cli });
      if (result.exitCode !== 0) {
        return `exit=${result.exitCode}\nstderr=${result.stderr}\nstdout=${result.stdout}`;
      }
      return result.stdout;
    },
  };
}

function buildKeiMetaTool(bridge: RustBridge): ToolDefinition {
  return {
    name: "kei",
    description:
      "Meta-tool: routes a natural-language query to the right primitive via kei-router.",
    inputSchema: z.object({ query: z.string().min(1) }),
    handler: async (rawArgs) => {
      const query = String(rawArgs["query"] ?? "");
      const result = await bridge.call({
        binary: "kei-router",
        args: ["--query", query],
      });
      if (result.exitCode !== 0) {
        return `router failed exit=${result.exitCode}\nstderr=${result.stderr}`;
      }
      return result.stdout;
    },
  };
}

export function lookupTool(
  registry: ReadonlyMap<string, ToolDefinition>,
  name: string,
): ToolDefinition {
  const t = registry.get(name);
  if (!t) throw new ToolNotFoundError(name);
  return t;
}
