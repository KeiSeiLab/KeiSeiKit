<script lang="ts">
  import { onDestroy } from 'svelte';
  import ChatBubble from './ChatBubble.svelte';
  import ChatInput from './ChatInput.svelte';
  import {
    create_history_store,
    new_msg_id,
  } from '../lib/chat/history-store.svelte';
  import type { ChatEvent, SentimentTag } from '../lib/chat/types';
  import type { CortexConfig } from '../lib/config';
  import { chat_stream, synthesize } from '../lib/api';
  import { playWithLipSync } from '../lib/voice/lip-sync';
  import type { RmsCallback } from '../lib/voice/types';

  interface Props {
    config: CortexConfig;
    user_id: string;
    /** Per-event mood update → Live2DPet. */
    onsentiment?: (tag: SentimentTag) => void;
    /** Per-frame RMS 0..1 driving `mouthOpenY`; 0 between utterances. */
    onrms?: RmsCallback;
  }

  const { config, user_id, onsentiment, onrms }: Props = $props();

  const store = create_history_store(user_id);
  let conversation_id = $state<string | undefined>(undefined);
  let in_flight = $state(false);
  let speaking = $state(false);
  let error = $state<string | null>(null);
  let log_el = $state<HTMLDivElement | null>(null);
  // Safari autoplay gate: flipped inside send() (Enter / click IS a gesture).
  let has_user_interacted = $state(false);
  // Per-send cancellation; onDestroy aborts whatever is still in flight.
  let abort_controller: AbortController | null = null;

  const messages = $derived(store.messages);
  const last_pet_sentiment = $derived(last_sentiment(messages));

  $effect(() => {
    void messages.length;
    void messages[messages.length - 1]?.text;
    if (log_el) log_el.scrollTop = log_el.scrollHeight;
  });

  onDestroy(() => {
    abort_controller?.abort();
    abort_controller = null;
  });

  function last_sentiment(msgs: readonly { role: string; sentiment?: SentimentTag }[]):
    SentimentTag | null {
    for (let i = msgs.length - 1; i >= 0; i--) {
      const m = msgs[i];
      if (m.role === 'pet' && m.sentiment) return m.sentiment;
    }
    return null;
  }

  function last_pet_text(): string {
    for (let i = store.messages.length - 1; i >= 0; i--) {
      const m = store.messages[i];
      if (m.role === 'pet' && m.text) return m.text;
    }
    return '';
  }

  async function speak_reply(text: string): Promise<void> {
    if (speaking || !text.trim()) return;
    if (!has_user_interacted) return; // Safari autoplay guard.
    speaking = true;
    try {
      const blob = await synthesize(config, user_id, text);
      await playWithLipSync(blob, (v) => onrms?.(v));
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      onrms?.(0);
    } finally {
      speaking = false;
      onrms?.(0);
    }
  }

  function stop_tts(): void {
    if (speaking) {
      speaking = false;
      onrms?.(0);
    }
  }

  function render_error_bubble(msg: string): void {
    error = msg;
    store.replace_last({ pending: false, text: `⚠ ${msg}` });
    stop_tts();
  }

  function handle_event(e: ChatEvent): void {
    if (e.type === 'token') {
      const last = store.messages[store.messages.length - 1];
      const next = (last?.text ?? '') + e.text;
      store.replace_last({ text: next });
      return;
    }
    if (e.type === 'sentiment') {
      store.replace_last({ sentiment: e.tag });
      onsentiment?.(e.tag);
      return;
    }
    if (e.type === 'done') {
      // Snapshot the final text synchronously (store may mutate mid-TTS).
      const final_text = last_pet_text();
      conversation_id = e.conversation_id;
      store.replace_last({ pending: false, ts: Date.now() });
      void speak_reply(final_text);
      return;
    }
    if (e.type === 'error') {
      render_error_bubble(e.message);
    }
  }

  async function send(text: string): Promise<void> {
    if (in_flight) return;
    has_user_interacted = true; // Enter / click is the gesture — Safari ok.
    abort_controller?.abort();
    abort_controller = new AbortController();
    const signal = abort_controller.signal;
    error = null;
    in_flight = true;
    store.append({ id: new_msg_id(), role: 'user', text, ts: Date.now() });
    store.append({
      id: new_msg_id(), role: 'pet', text: '',
      ts: Date.now(), pending: true,
    });
    try {
      await chat_stream(config, user_id, text, conversation_id, handle_event, signal);
    } finally {
      in_flight = false;
    }
  }
</script>

<section class="chat-panel" aria-label="Chat with pet">
  <div
    bind:this={log_el}
    class="chat-log"
    role="log"
    aria-live="polite"
    aria-relevant="additions text"
  >
    {#if messages.length === 0}
      <p class="muted chat-empty">Say hi to start a conversation.</p>
    {/if}
    {#each messages as m (m.id)}
      <ChatBubble msg={m} show_sentiment={m.id === messages[messages.length - 1]?.id} />
    {/each}
  </div>
  {#if error}
    <div class="error chat-error" role="alert">{error}</div>
  {/if}
  <div class="chat-status muted" aria-live="polite">
    {#if in_flight}
      <span>pet is typing…</span>
    {:else if speaking}
      <span aria-hidden="true">🔊</span>
      <span>pet is speaking…</span>
    {:else if last_pet_sentiment}
      <span>last sentiment: <strong>{last_pet_sentiment}</strong></span>
    {/if}
  </div>
  <ChatInput
    {config}
    disabled={in_flight}
    placeholder={in_flight ? 'Waiting for reply…' : 'Type a message…'}
    onsubmit={send}
  />
</section>

<style>
  .chat-panel {
    display: flex; flex-direction: column;
    width: 100%; max-width: 560px;
    border: 1px solid var(--border); border-radius: 10px;
    background: var(--card); overflow: hidden;
    min-height: 320px; max-height: 520px;
  }
  .chat-log {
    flex: 1; overflow-y: auto; padding: 12px;
    display: flex; flex-direction: column; gap: 8px;
    background: var(--bg);
  }
  .chat-empty { text-align: center; margin: auto; }
  .chat-status {
    padding: 4px 12px; font-size: 12px; min-height: 20px;
    display: flex; gap: 4px; align-items: center;
  }
  .chat-error { color: var(--danger); margin: 6px 8px; }
</style>
