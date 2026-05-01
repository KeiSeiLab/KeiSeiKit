import { describe, it, expect } from "vitest";
import { buildRegistry, lookupTool, RUST_PRIMITIVE_TOOLS } from "../src/tool-registry.js";
import { RustBridge } from "../src/rust-bridge.js";
import { ToolNotFoundError } from "../src/errors.js";

describe("tool registry", () => {
  const bridge = new RustBridge({ binDir: "/tmp/stub" });
  const registry = buildRegistry(bridge);

  it("registers one tool per Rust primitive", () => {
    for (const t of RUST_PRIMITIVE_TOOLS) {
      expect(registry.has(t.binary)).toBe(true);
    }
  });

  it("registers the kei meta-tool", () => {
    const t = lookupTool(registry, "kei");
    expect(t.name).toBe("kei");
    expect(t.description).toContain("Meta-tool");
  });

  it("lookupTool throws ToolNotFoundError for unknown names", () => {
    expect(() => lookupTool(registry, "nonexistent-tool")).toThrow(ToolNotFoundError);
  });

  it("tool description is non-empty for each primitive", () => {
    for (const t of RUST_PRIMITIVE_TOOLS) {
      expect(t.desc.length).toBeGreaterThan(10);
    }
  });
});
