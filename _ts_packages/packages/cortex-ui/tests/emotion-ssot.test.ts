import { describe, it, expect } from 'vitest';
import {
  Emotion,
  EMOTION_EmotionMotionName_value,
  EMOTION_VRMExpressionName_value,
} from '../src/lib/live2d/emotions';

// SSoT invariant: enum key === CamelCase(value), i.e. every value is the
// lowercase of its key. Fix #6 (wave 23) renamed Surprise → Surprised so
// this now holds uniformly.

describe('Emotion enum — SSoT key/value alignment', () => {
  it('every key.toLowerCase() equals its value', () => {
    const entries = Object.entries(Emotion) as ReadonlyArray<[string, string]>;
    for (const [key, value] of entries) {
      expect(value).toBe(key.toLowerCase());
    }
  });

  it('every Emotion value appears as a key in EMOTION_EmotionMotionName_value', () => {
    for (const value of Object.values(Emotion)) {
      const motion = EMOTION_EmotionMotionName_value[value];
      expect(typeof motion).toBe('string');
      expect(motion.length).toBeGreaterThan(0);
    }
  });

  it('every Emotion value has an entry in EMOTION_VRMExpressionName_value (may be undefined)', () => {
    for (const value of Object.values(Emotion)) {
      // `in` check — intentionally looser than `typeof !== undefined` since
      // the record legitimately stores `undefined` for non-VRM emotions.
      expect(value in EMOTION_VRMExpressionName_value).toBe(true);
    }
  });

  it('exposes 9 emotions with unique values', () => {
    const values = Object.values(Emotion);
    expect(values.length).toBe(9);
    expect(new Set(values).size).toBe(9);
  });

  it('Surprised key (post-rename) resolves to server-wire "surprised"', () => {
    // Regression: pre-fix-#6 the key was `Surprise`, value `surprised` —
    // breaking the key/value invariant tested above.
    expect(Emotion.Surprised).toBe('surprised');
  });
});
