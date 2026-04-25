/**
 * diff-sentry — captures `tool_use_start{name:"edit"}` / `tool_use_result`
 * pairs from the Wave 30 chat stream and reduces them into a `PendingDiff`
 * record consumable by `DiffPane`.
 *
 * The chat stream emits these events alongside the existing token /
 * sentiment / done events (see ChatEvent in lib/chat/types.ts). Until the
 * daemon ships them in production, the PetEditor wires this sentry behind
 * a feature flag — tests inject sample events directly.
 */

export interface ToolEditInput {
  path?: string;
  old_string?: string;
  new_string?: string;
}

export interface ToolUseStartEvent {
  type: 'tool_use_start';
  name: string;
  input?: ToolEditInput;
}

export interface ToolUseResultEvent {
  type: 'tool_use_result';
  // Daemon may include the resolved oldText after disk-read; if absent we
  // fall back to whatever start.input.old_string carried.
  old_text?: string;
  new_text?: string;
}

export type ToolEvent = ToolUseStartEvent | ToolUseResultEvent;

export interface PendingDiff {
  filename: string;
  oldText: string;
  newText: string;
}

/**
 * Reduce a single tool event into a partial PendingDiff. Returns the next
 * state given the previous state. Caller (PetEditor) holds the state in a
 * `$state` rune and clears it on Apply / Reject.
 */
export function reduce_tool_event(
  prev: PendingDiff | null,
  ev: ToolEvent,
): PendingDiff | null {
  if (ev.type === 'tool_use_start') {
    if (ev.name !== 'edit') return prev; // only 'edit' tool produces diffs
    const input = ev.input ?? {};
    return {
      filename: input.path ?? '(unknown)',
      oldText: input.old_string ?? '',
      newText: simulated_post_edit(input),
    };
  }
  if (ev.type === 'tool_use_result') {
    if (!prev) return prev;
    return {
      filename: prev.filename,
      oldText: ev.old_text ?? prev.oldText,
      newText: ev.new_text ?? prev.newText,
    };
  }
  return prev;
}

/**
 * Simulate the post-edit text by performing the old_string→new_string
 * substitution on whatever `old_string` carried as a best-effort preview
 * before the daemon returns the actual `tool_use_result`.
 */
function simulated_post_edit(input: ToolEditInput): string {
  const old_s = input.old_string ?? '';
  const new_s = input.new_string ?? '';
  return old_s.replace(old_s, new_s);
}

/**
 * Type guard — narrows an arbitrary chat-stream payload to a ToolEvent.
 * Used by the chat sentry to peek at events without bloating the public
 * ChatEvent union.
 */
export function is_tool_event(x: unknown): x is ToolEvent {
  if (typeof x !== 'object' || x === null) return false;
  const t = (x as { type?: unknown }).type;
  return t === 'tool_use_start' || t === 'tool_use_result';
}
