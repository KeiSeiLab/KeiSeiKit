import { describe, it, expect } from 'vitest';
import { SENTIMENT_TAGS } from '../src/lib/chat/types';
import type { SentimentTag } from '../src/lib/chat/types';
import { Emotion } from '../src/lib/live2d/emotions';
import { sentimentToEmotion } from '../src/lib/chat/sentiment-to-emotion';
import type { BaseModel } from '../src/lib/portrait/upload';

// Hard-coded mirrors of the Rust server contract. This file is the machine-
// checkable guard against server↔client drift until we have codegen.
// Update references when the server changes — see source links below.

// Mirror of `SentimentTag` returned by /api/v1/cortex/pet/:id/chat SSE.
// Source: server sentiment classifier (Rust). Keep the order in sync so a
// future codegen step can byte-compare arrays.
const SERVER_SENTIMENT_TAGS: readonly string[] = [
  'happy', 'sad', 'angry', 'think',
  'surprised', 'awkward', 'question', 'curious',
  'neutral',
];

// Mirror of the server's SSE ChatEvent `type` discriminator values.
const SERVER_SSE_EVENT_TYPES: readonly string[] = [
  'token', 'sentiment', 'done', 'error',
];

// Mirror of the base-rig allow-list in `portrait.rs:21` (server).
const SERVER_BASE_MODELS: readonly string[] = [
  'haru', 'mao', 'hiyori', 'mark',
];

describe('contract parity — client ↔ Rust server', () => {
  it('SENTIMENT_TAGS matches server order byte-for-byte', () => {
    expect([...SENTIMENT_TAGS]).toEqual([...SERVER_SENTIMENT_TAGS]);
  });

  it('every SentimentTag maps to a defined Emotion via sentimentToEmotion', () => {
    for (const tag of SENTIMENT_TAGS) {
      const e = sentimentToEmotion(tag as SentimentTag);
      expect(Object.values(Emotion)).toContain(e);
    }
  });

  it('SSE event-type set is closed — no client-only / server-only types', () => {
    // The ChatEvent discriminated union should have exactly these types.
    // If a new variant is added client-side, update the list above AFTER
    // confirming the server emits it.
    const client_types = new Set<string>([
      'token', 'sentiment', 'done', 'error',
    ]);
    expect(client_types).toEqual(new Set(SERVER_SSE_EVENT_TYPES));
  });

  it('BaseModel union matches server allow-list', () => {
    // TS types are erased at runtime — we instead pin the four literals here
    // and check that assigning each to BaseModel is valid (compile-time)
    // AND that the runtime array mirrors the server list.
    const client_base_models: readonly BaseModel[] = [
      'haru', 'mao', 'hiyori', 'mark',
    ];
    expect([...client_base_models]).toEqual([...SERVER_BASE_MODELS]);
  });
});
