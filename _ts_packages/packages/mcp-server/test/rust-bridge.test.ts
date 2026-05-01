import { describe, it, expect } from "vitest";
import { jsonArgsToCli, RustBridge } from "../src/rust-bridge.js";
import { RustBridgeError } from "../src/errors.js";

describe("jsonArgsToCli", () => {
  it("converts snake_case keys to --kebab-case flags", () => {
    expect(jsonArgsToCli({ foo_bar: "value" })).toEqual(["--foo-bar", "value"]);
  });

  it("emits booleans as presence-only flags", () => {
    expect(jsonArgsToCli({ verbose: true })).toEqual(["--verbose"]);
    expect(jsonArgsToCli({ verbose: false })).toEqual([]);
  });

  it("skips null and undefined values", () => {
    expect(jsonArgsToCli({ a: null, b: undefined, c: "x" })).toEqual(["--c", "x"]);
  });

  it("stringifies numeric values", () => {
    expect(jsonArgsToCli({ count: 42 })).toEqual(["--count", "42"]);
  });
});

describe("RustBridge binary resolution", () => {
  it("rejects illegal binary names", async () => {
    const bridge = new RustBridge({ binDir: "/tmp" });
    await expect(bridge.call({ binary: "../etc/passwd", args: [] })).rejects.toBeInstanceOf(
      RustBridgeError,
    );
  });

  it("accepts valid snake_case and kebab-case names (resolves with non-zero exit on ENOENT)", async () => {
    const bridge = new RustBridge({ binDir: "/tmp" });
    const result = await bridge.call({ binary: "kei-ledger", args: [], timeoutMs: 500 });
    // execa is configured with reject:false → a missing binary resolves with exitCode != 0
    // (validation passed — this was the assertion under test).
    expect(result.exitCode).not.toBe(0);
  });
});
