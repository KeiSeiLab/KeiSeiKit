// Bridge from server sentiment tag → AIRI Emotion enum used by Live2DPet.
// The mapping is 1-to-1; this function centralises the contract so the
// string literals never leak into UI code.

import { Emotion } from '../live2d/emotions';
import type { SentimentTag } from './types';

const TAG_TO_EMOTION: Record<SentimentTag, Emotion> = {
  happy: Emotion.Happy,
  sad: Emotion.Sad,
  angry: Emotion.Angry,
  think: Emotion.Think,
  surprised: Emotion.Surprised,
  awkward: Emotion.Awkward,
  question: Emotion.Question,
  curious: Emotion.Curious,
  neutral: Emotion.Neutral,
};

export function sentimentToEmotion(tag: SentimentTag): Emotion {
  return TAG_TO_EMOTION[tag];
}
