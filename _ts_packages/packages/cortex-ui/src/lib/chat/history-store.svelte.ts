// Session-scoped chat history store (NOT persisted). One instance per
// (user_id) pair, produced by `create_history_store`. Svelte 5 runes
// require this module to be `.svelte.ts` so the compiler can rewrite
// `$state`.
//
// Consumers:
//   const store = create_history_store(user_id);
//   // store.messages is a reactive array; reads inside `$derived` or
//   // `$effect` re-run on mutation.

import type { ChatMsg } from './types';

export interface ChatHistoryStore {
  readonly user_id: string;
  readonly messages: ChatMsg[];
  append(msg: ChatMsg): void;
  /** Patch the last message in-place (used for streaming-token renders). */
  replace_last(patch: Partial<Omit<ChatMsg, 'id' | 'role'>>): void;
  clear(): void;
}

export function create_history_store(user_id: string): ChatHistoryStore {
  const messages = $state<ChatMsg[]>([]);
  return {
    user_id,
    get messages() {
      return messages;
    },
    append(msg: ChatMsg) {
      messages.push(msg);
    },
    replace_last(patch) {
      const last = messages[messages.length - 1];
      if (!last) return;
      if (patch.text !== undefined) last.text = patch.text;
      if (patch.ts !== undefined) last.ts = patch.ts;
      if (patch.sentiment !== undefined) last.sentiment = patch.sentiment;
      if (patch.pending !== undefined) last.pending = patch.pending;
    },
    clear() {
      messages.length = 0;
    },
  };
}

/** Cheap, collision-unlikely id for in-memory bubble tracking. */
export function new_msg_id(): string {
  return `m_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 8)}`;
}
