import { describe, it, expect } from 'vitest';
import {
  Emotion,
  EMOTION_VALUES,
  EMOTION_EmotionMotionName_value,
  EMOTION_VRMExpressionName_value,
  EmotionNeutralMotionName,
  moodToEmotion,
} from '../src/lib/live2d/emotions';

describe('emotions enum', () => {
  it('contains all 9 AIRI-canonical emotions', () => {
    expect(EMOTION_VALUES).toHaveLength(9);
    expect(EMOTION_VALUES).toEqual(
      expect.arrayContaining([
        Emotion.Happy,
        Emotion.Sad,
        Emotion.Angry,
        Emotion.Think,
        Emotion.Surprised,
        Emotion.Awkward,
        Emotion.Question,
        Emotion.Curious,
        Emotion.Neutral,
      ]),
    );
  });

  it('maps every emotion to a non-empty motion-name string', () => {
    for (const e of EMOTION_VALUES) {
      const name = EMOTION_EmotionMotionName_value[e];
      expect(typeof name).toBe('string');
      expect(name.length).toBeGreaterThan(0);
    }
  });

  it('Neutral maps to Idle (AIRI-canonical group name)', () => {
    expect(EMOTION_EmotionMotionName_value[Emotion.Neutral]).toBe('Idle');
    expect(EmotionNeutralMotionName).toBe('Idle');
  });

  it('VRM table leaves non-VRM-mappable emotions undefined', () => {
    expect(EMOTION_VRMExpressionName_value[Emotion.Think]).toBeUndefined();
    expect(EMOTION_VRMExpressionName_value[Emotion.Awkward]).toBeUndefined();
    expect(EMOTION_VRMExpressionName_value[Emotion.Question]).toBeUndefined();
    expect(EMOTION_VRMExpressionName_value[Emotion.Neutral]).toBeUndefined();
    expect(EMOTION_VRMExpressionName_value[Emotion.Happy]).toBe('happy');
  });

  it('Curious falls back to surprised in VRM map (AIRI convention)', () => {
    expect(EMOTION_VRMExpressionName_value[Emotion.Curious]).toBe('surprised');
  });
});

describe('moodToEmotion bridge', () => {
  it('maps cortex mood → AIRI emotion deterministically', () => {
    expect(moodToEmotion('idle')).toBe(Emotion.Neutral);
    expect(moodToEmotion('happy')).toBe(Emotion.Happy);
    expect(moodToEmotion('think')).toBe(Emotion.Think);
    expect(moodToEmotion('sleep')).toBe(Emotion.Neutral);
  });
});
