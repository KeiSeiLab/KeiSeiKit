// Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
// Original: packages/stage-ui-live2d/src/constants/emotions.ts
// Copy-as-is; no Vue/Svelte API surface here, just an enum + lookup tables.
//
// SSoT invariant: every enum key is CamelCase(value). Server wire-form uses
// the lowercase past-participle ('surprised'), so key 'Surprised' aligns
// key.toLowerCase() === value. Consumers referenced the old 'Surprise' key
// (fix #6, wave 23 UI hardening) — callers MUST use Emotion.Surprised now.

export enum Emotion {
  Happy = 'happy',
  Sad = 'sad',
  Angry = 'angry',
  Think = 'think',
  Surprised = 'surprised',
  Awkward = 'awkward',
  Question = 'question',
  Curious = 'curious',
  Neutral = 'neutral',
}

export const EMOTION_VALUES = Object.values(Emotion);

export const EmotionHappyMotionName = 'Happy';
export const EmotionSadMotionName = 'Sad';
export const EmotionAngryMotionName = 'Angry';
export const EmotionAwkwardMotionName = 'Awkward';
export const EmotionThinkMotionName = 'Think';
export const EmotionSurprisedMotionName = 'Surprise';
export const EmotionQuestionMotionName = 'Question';
export const EmotionNeutralMotionName = 'Idle';
export const EmotionCuriousMotionName = 'Curious';

export const EMOTION_EmotionMotionName_value: Record<Emotion, string> = {
  [Emotion.Happy]: EmotionHappyMotionName,
  [Emotion.Sad]: EmotionSadMotionName,
  [Emotion.Angry]: EmotionAngryMotionName,
  [Emotion.Think]: EmotionThinkMotionName,
  [Emotion.Surprised]: EmotionSurprisedMotionName,
  [Emotion.Awkward]: EmotionAwkwardMotionName,
  [Emotion.Question]: EmotionQuestionMotionName,
  [Emotion.Neutral]: EmotionNeutralMotionName,
  [Emotion.Curious]: EmotionCuriousMotionName,
};

export const EMOTION_VRMExpressionName_value: Record<Emotion, string | undefined> = {
  [Emotion.Happy]: 'happy',
  [Emotion.Sad]: 'sad',
  [Emotion.Angry]: 'angry',
  [Emotion.Think]: undefined,
  [Emotion.Surprised]: 'surprised',
  [Emotion.Awkward]: undefined,
  [Emotion.Question]: undefined,
  [Emotion.Neutral]: undefined,
  [Emotion.Curious]: 'surprised',
};

/**
 * Cortex mood (4 buckets used by PetEditor) → Live2D Emotion.
 * This is the bridge from the existing pet-editor sprite moods to the
 * finer-grained AIRI emotion enum.
 */
export type CortexMood = 'idle' | 'happy' | 'think' | 'sleep';

export function moodToEmotion(mood: CortexMood): Emotion {
  switch (mood) {
    case 'happy':
      return Emotion.Happy;
    case 'think':
      return Emotion.Think;
    case 'sleep':
      return Emotion.Neutral; // no "Sleep" emotion; closest resting state.
    case 'idle':
    default:
      return Emotion.Neutral;
  }
}
