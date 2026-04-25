import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { render, cleanup, waitFor, fireEvent } from '@testing-library/svelte';
import ChatPanel from '../src/components/ChatPanel.svelte';
import type { ChatEvent } from '../src/lib/chat/types';

// The api module is a thin wrapper around fetch; stub it so the component
// emits the scripted event sequence deterministically and synthesize is
// observable from the test.
const synthesize_calls: Array<[string, string]> = [];
let scripted_events: ChatEvent[] = [];

vi.mock('../src/lib/api', () => ({
  chat_stream: async (
    _c: unknown,
    _u: string,
    _m: string,
    _conv: string | undefined,
    onEvent: (e: ChatEvent) => void,
    _signal?: AbortSignal,
  ) => {
    for (const e of scripted_events) onEvent(e);
  },
  synthesize: async (_c: unknown, u: string, t: string) => {
    synthesize_calls.push([u, t]);
    return new Blob(['fake-audio'], { type: 'audio/mpeg' });
  },
}));

// Stub lip-sync so no real AudioContext is required in jsdom.
vi.mock('../src/lib/voice/lip-sync', () => ({
  playWithLipSync: vi.fn(async () => {}),
}));

async function type_and_send(
  container: HTMLElement,
  text: string,
): Promise<void> {
  const ta = container.querySelector(
    '#chat-textarea',
  ) as HTMLTextAreaElement;
  // fireEvent.input drives Svelte's bind:value through the input event.
  await fireEvent.input(ta, { target: { value: text } });
  await fireEvent.keyDown(ta, { key: 'Enter' });
}

describe('ChatPanel — error bubble + in_flight reset + TTS guard', () => {
  beforeEach(() => {
    synthesize_calls.length = 0;
    scripted_events = [];
  });
  afterEach(() => cleanup());

  it('renders "⚠ <msg>" bubble, resets in_flight, and skips TTS on error', async () => {
    scripted_events = [
      { type: 'token', text: 'hello' },
      { type: 'error', message: 'boom' },
    ];
    const { container } = render(ChatPanel, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        user_id: 'u',
      },
    });
    await type_and_send(container, 'hi');

    const panel = container.querySelector('.chat-panel') as HTMLElement;
    await waitFor(() => {
      expect(panel.textContent ?? '').toContain('⚠ boom');
    });

    // in_flight reset → typing indicator gone.
    expect(panel.textContent ?? '').not.toContain('pet is typing');

    // TTS must NOT fire on error.
    await new Promise((r) => setTimeout(r, 0));
    expect(synthesize_calls).toHaveLength(0);
  });

  it('calls synthesize on successful done event (user gesture opened gate)', async () => {
    scripted_events = [
      { type: 'token', text: 'hi there' },
      { type: 'done', conversation_id: 'c-1' },
    ];
    const { container } = render(ChatPanel, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        user_id: 'u',
      },
    });
    await type_and_send(container, 'ping');

    const panel = container.querySelector('.chat-panel') as HTMLElement;
    await waitFor(() => {
      expect(panel.textContent ?? '').toContain('hi there');
    });
    // Flush the floating speak_reply() microtask chain.
    await new Promise((r) => setTimeout(r, 0));
    expect(synthesize_calls).toHaveLength(1);
    expect(synthesize_calls[0][1]).toBe('hi there');
  });
});
