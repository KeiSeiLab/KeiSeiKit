// Typed error hierarchy for MCP server. One class per failure mode.
// Keeps the main handler branches flat and the JSON-RPC error codes consistent.

export class McpServerError extends Error {
  public readonly code: number;
  public readonly data: unknown;

  constructor(message: string, code: number, data?: unknown) {
    super(message);
    this.name = new.target.name;
    this.code = code;
    this.data = data;
  }
}

export class AuthError extends McpServerError {
  constructor(message = "unauthorized", data?: unknown) {
    super(message, -32001, data);
  }
}

export class ToolNotFoundError extends McpServerError {
  constructor(toolName: string) {
    super(`tool not found: ${toolName}`, -32601, { tool: toolName });
  }
}

export class RustBridgeError extends McpServerError {
  constructor(message: string, data?: unknown) {
    super(`rust bridge: ${message}`, -32002, data);
  }
}

export class SchemaError extends McpServerError {
  constructor(message: string, data?: unknown) {
    super(`schema: ${message}`, -32602, data);
  }
}

export class TimeoutError extends McpServerError {
  constructor(toolName: string, ms: number) {
    super(`tool ${toolName} timed out after ${ms}ms`, -32003, { tool: toolName, ms });
  }
}

export function isMcpError(err: unknown): err is McpServerError {
  return err instanceof McpServerError;
}

export function toErrorPayload(err: unknown): { code: number; message: string; data?: unknown } {
  if (isMcpError(err)) {
    const payload: { code: number; message: string; data?: unknown } = {
      code: err.code,
      message: err.message,
    };
    if (err.data !== undefined) payload.data = err.data;
    return payload;
  }
  const message = err instanceof Error ? err.message : String(err);
  return { code: -32000, message };
}
