// Register external API adapters (Telegram, Recall, Grok, Gmail, YouTube)
// dynamically IF the sibling packages are installed in the runtime. Each
// adapter exports `registerAdapter(register)` by convention.

import type { ToolDefinition } from "./tool-registry.js";

export type AdapterRegistrar = (tool: ToolDefinition) => void;

interface AdapterModule {
  registerAdapter: (register: AdapterRegistrar) => void;
}

const ADAPTER_PACKAGES: readonly string[] = [
  "@keisei/telegram-adapter",
  "@keisei/recall-adapter",
  "@keisei/grok-adapter",
  "@keisei/gmail-adapter",
  "@keisei/youtube-adapter",
];

export async function loadAllAdapters(
  register: AdapterRegistrar,
  logger: (msg: string) => void = () => {},
): Promise<{ loaded: string[]; skipped: string[] }> {
  const loaded: string[] = [];
  const skipped: string[] = [];
  for (const pkg of ADAPTER_PACKAGES) {
    const ok = await tryLoadOne(pkg, register, logger);
    if (ok) loaded.push(pkg);
    else skipped.push(pkg);
  }
  return { loaded, skipped };
}

async function tryLoadOne(
  pkg: string,
  register: AdapterRegistrar,
  logger: (msg: string) => void,
): Promise<boolean> {
  try {
    const mod = (await import(pkg)) as AdapterModule;
    if (typeof mod.registerAdapter !== "function") {
      logger(`adapter ${pkg}: missing registerAdapter()`);
      return false;
    }
    mod.registerAdapter(register);
    return true;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    logger(`adapter ${pkg}: not installed (${msg})`);
    return false;
  }
}
