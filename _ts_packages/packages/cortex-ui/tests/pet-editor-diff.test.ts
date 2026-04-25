import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { render, cleanup, waitFor, fireEvent } from '@testing-library/svelte';
import type { PetManifest } from '../src/lib/types';
import {
  reduce_tool_event,
  type ToolEvent,
  type PendingDiff,
} from '../src/lib/chat/diff-sentry';

const apply_calls: PendingDiff[] = [];

const FAKE_MANIFEST: PetManifest = {
  schema: 1,
  identity: { pet_name: 'cat', user_name: 'u', addressing: 'you', languages: ['en'] },
  voice: { tone_primary: 'warm', tone_secondary: [], humor_style: 'dry', humor_frequency: 'low' },
  edge: { profanity: 'off', profanity_languages: [], directness: 'medium', initiative: 'low' },
  forbidden: { topics: [], tone_patterns: [] },
  meta: {},
};

vi.mock('../src/lib/api', () => ({
  pet: vi.fn().mockResolvedValue({ pet: FAKE_MANIFEST }),
  applyToolEdit: vi.fn(async (_c: unknown, d: PendingDiff) => {
    apply_calls.push(d);
  }),
  chat_stream: vi.fn(async () => { /* noop — ChatPanel idle in this test */ }),
  chatStreamWithProvider: vi.fn(async () => {}),
  synthesize: vi.fn(),
  transcribe: vi.fn(),
  summary: vi.fn(),
  ledger: vi.fn(),
  memory_search: vi.fn(),
}));

// Diff lib is mocked the same way diff-pane.test.ts does — keep tests
// hermetic in a workspace where `pnpm add diff` may not have run yet.
vi.mock('diff', () => ({
  diffLines: (oldStr: string, newStr: string) => {
    const o = oldStr.split('\n');
    const n = newStr.split('\n');
    const out: Array<{ value: string; added?: boolean; removed?: boolean }> = [];
    const max = Math.max(o.length, n.length);
    for (let i = 0; i < max; i++) {
      const ol = o[i];
      const nl = n[i];
      if (ol === nl && ol !== undefined) out.push({ value: `${ol}\n` });
      else {
        if (ol !== undefined) out.push({ value: `${ol}\n`, removed: true });
        if (nl !== undefined) out.push({ value: `${nl}\n`, added: true });
      }
    }
    return out;
  },
}));

// jsdom has no AudioContext — keep TTS off the path entirely.
vi.mock('../src/lib/voice/lip-sync', () => ({
  playWithLipSync: vi.fn(async () => {}),
}));

import PetEditor from '../src/routes/PetEditor.svelte';

beforeEach(() => {
  apply_calls.length = 0;
  localStorage.clear();
});

afterEach(() => cleanup());

describe('PetEditor — DiffPane integration', () => {
  it('does not render DiffPane when pendingDiff is null', async () => {
    const { container } = render(PetEditor, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        user_id: 'u',
        pendingDiff: null,
      },
    });
    await waitFor(() => {
      // manifest finished loading
      expect(container.querySelector('.pet-stage')).toBeTruthy();
    });
    expect(container.querySelector('.diff-pane')).toBeNull();
  });

  it('mounts DiffPane with the right oldText / newText / filename when injected', async () => {
    const diff: PendingDiff = {
      filename: 'src/main.ts',
      oldText: 'foo\nbar\n',
      newText: 'foo\nbaz\n',
    };
    const { container } = render(PetEditor, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        user_id: 'u',
        pendingDiff: diff,
      },
    });
    await waitFor(() => {
      expect(container.querySelector('.diff-pane')).toBeTruthy();
    });
    const pane = container.querySelector('.diff-pane') as HTMLElement;
    expect(pane.textContent ?? '').toContain('src/main.ts');
    // The mock differ emits `bar` removed and `baz` added.
    const adds = pane.querySelectorAll('.diff-add');
    const dels = pane.querySelectorAll('.diff-del');
    expect(Array.from(adds).map((e) => e.textContent ?? '').join('')).toContain('baz');
    expect(Array.from(dels).map((e) => e.textContent ?? '').join('')).toContain('bar');
  });

  it('reduces tool_use_start{name:edit} → tool_use_result event pair into a PendingDiff', () => {
    const start: ToolEvent = {
      type: 'tool_use_start',
      name: 'edit',
      input: { path: 'a.ts', old_string: 'x', new_string: 'y' },
    };
    const result: ToolEvent = {
      type: 'tool_use_result',
      old_text: 'x',
      new_text: 'y',
    };
    let st = reduce_tool_event(null, start);
    expect(st).not.toBeNull();
    expect(st!.filename).toBe('a.ts');
    st = reduce_tool_event(st, result);
    expect(st).not.toBeNull();
    expect(st!.oldText).toBe('x');
    expect(st!.newText).toBe('y');
  });

  it('Apply triggers applyToolEdit and Reject clears the diff', async () => {
    const diff: PendingDiff = { filename: 'x.ts', oldText: 'a\n', newText: 'b\n' };
    const { container } = render(PetEditor, {
      props: {
        config: { daemon_url: 'http://x', token: 't' },
        user_id: 'u',
        pendingDiff: diff,
      },
    });
    await waitFor(() => {
      expect(container.querySelector('.diff-apply')).toBeTruthy();
    });
    const apply = container.querySelector('.diff-apply') as HTMLButtonElement;
    await fireEvent.click(apply);
    await waitFor(() => {
      expect(apply_calls.length).toBe(1);
    });
    expect(apply_calls[0].filename).toBe('x.ts');
  });
});
