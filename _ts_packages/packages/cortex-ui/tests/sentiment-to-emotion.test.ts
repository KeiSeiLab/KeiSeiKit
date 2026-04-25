import { describe, it, expect } from 'vitest';
import { sentimentToEmotion } from '../src/lib/chat/sentiment-to-emotion';
import { Emotion } from '../src/lib/live2d/emotions';
import { SENTIMENT_TAGS } from '../src/lib/chat/types';
import type { SentimentTag } from '../src/lib/chat/types';

describe('sentimentToEmotion', () => {
  const CASES: ReadonlyArray<[SentimentTag, Emotion]> = [
    ['happy', Emotion.Happy],
    ['sad', Emotion.Sad],
    ['angry', Emotion.Angry],
    ['think', Emotion.Think],
    ['surprised', Emotion.Surprised],
    ['awkward', Emotion.Awkward],
    ['question', Emotion.Question],
    ['curious', Emotion.Curious],
    ['neutral', Emotion.Neutral],
  ];

  it.each(CASES)('maps %s → %s', (tag, expected) => {
    expect(sentimentToEmotion(tag)).toBe(expected);
  });

  it('covers all 9 sentiment tags with zero gaps', () => {
    expect(CASES).toHaveLength(9);
    expect(SENTIMENT_TAGS).toHaveLength(9);
    const covered = new Set(CASES.map(([t]) => t));
    for (const t of SENTIMENT_TAGS) expect(covered.has(t)).toBe(true);
  });

  it('produces a unique Emotion per tag (no two tags collapse)', () => {
    const emotions = CASES.map(([, e]) => e);
    expect(new Set(emotions).size).toBe(emotions.length);
  });
});
