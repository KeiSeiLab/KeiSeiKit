// Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
// Original: packages/stage-ui-live2d/src/composables/live2d/motion-manager.ts
// Adapted from Vue Composition API (Ref + composables + plugin registry) to
// a plain TS state machine + closure. The AIRI version runs a plugin chain
// each frame hooked into pixi-live2d-display's `MotionManager.update`. Our
// cortex-ui use case is coarser: switch emotions on user action, and let
// pixi-live2d-display's built-in idle + eyeBlink loops handle breathing.
// Only idle/Tap motion groups + F01-F08 expressions are mapped.

import {
  Emotion,
  EMOTION_EmotionMotionName_value,
} from './emotions';
import type { MotionManagerLike } from './types';

/**
 * Which motion group + expression index the AIRI emotion maps to on the
 * bundled Haru model. Models with richer per-emotion motion groups
 * (`Happy` / `Sad` / ...) are handled by `MotionManager.startMotion`
 * falling through to the declared group if present.
 */
export interface EmotionBinding {
  /** Motion group name used by `manager.startMotion(group, index)`. */
  group: string;
  /** Optional index inside the group; defaults to 0 (first clip). */
  index?: number;
  /** Optional expression name inside the model's Expressions array. */
  expression?: string;
}

/** Haru-specific fallback bindings used when the model lacks emotion-named groups. */
export const HARU_EMOTION_BINDINGS: Record<Emotion, EmotionBinding> = {
  [Emotion.Neutral]: { group: 'Idle', index: 0, expression: 'f00' },
  [Emotion.Happy]: { group: 'Tap', index: 0, expression: 'f01' },
  [Emotion.Sad]: { group: 'Idle', index: 1, expression: 'f02' },
  [Emotion.Angry]: { group: 'Tap', index: 1, expression: 'f03' },
  [Emotion.Surprised]: { group: 'Tap', index: 0, expression: 'f04' },
  [Emotion.Awkward]: { group: 'Idle', index: 2, expression: 'f05' },
  [Emotion.Think]: { group: 'Idle', index: 0, expression: 'f06' },
  [Emotion.Question]: { group: 'Idle', index: 0, expression: 'f07' },
  [Emotion.Curious]: { group: 'Tap', index: 1, expression: 'f07' },
};

export interface MotionManagerState {
  currentEmotion: Emotion;
  lastGroup: string | null;
  lastIndex: number;
}

function initialState(): MotionManagerState {
  return {
    currentEmotion: Emotion.Neutral,
    lastGroup: null,
    lastIndex: 0,
  };
}

export interface EmotionMotionManagerOptions {
  /** Per-emotion bindings. Defaults to `HARU_EMOTION_BINDINGS`. */
  bindings?: Record<Emotion, EmotionBinding>;
  /**
   * Preferred motion group for a named emotion (e.g. `'Happy'`, `'Sad'`).
   * If the pixi manager reports a group by this name, it's used in place
   * of the generic binding. Defaults to the AIRI convention from
   * `EMOTION_EmotionMotionName_value`.
   */
  preferNamedGroups?: boolean;
  /** Priority passed to `startMotion`. Default 2 (NORMAL in pixi). */
  priority?: number;
}

/**
 * Return value of {@link createEmotionMotionManager}.
 */
export interface EmotionMotionManager {
  setEmotion(emotion: Emotion): Promise<boolean>;
  getCurrentEmotion(): Emotion;
  /** Re-play the last picked motion (e.g. after model reload). */
  replay(): Promise<boolean>;
  /** Reset state to Neutral. Does NOT call the manager. */
  reset(): void;
  /** Expose internal state for tests. */
  debugState(): Readonly<MotionManagerState>;
}

/**
 * Create an emotion-driven motion manager that wraps a
 * pixi-live2d-display `MotionManager`.
 *
 * The returned object is the minimum surface the Svelte component needs:
 * `setEmotion(e)` during user interaction, `getCurrentEmotion()` for
 * display, `replay()` after reload.
 */
export function createEmotionMotionManager(
  manager: MotionManagerLike,
  opts: EmotionMotionManagerOptions = {},
): EmotionMotionManager {
  const bindings = opts.bindings ?? HARU_EMOTION_BINDINGS;
  const preferNamed = opts.preferNamedGroups ?? true;
  const priority = opts.priority ?? 2;
  const state: MotionManagerState = initialState();

  async function setEmotion(emotion: Emotion): Promise<boolean> {
    const binding = pickBinding(manager, emotion, bindings, preferNamed);
    state.currentEmotion = emotion;
    state.lastGroup = binding.group;
    state.lastIndex = binding.index ?? 0;

    const ok = await playMotion(manager, binding, priority);
    if (ok && binding.expression) {
      await applyExpression(manager, binding.expression);
    }
    return ok;
  }

  function replay(): Promise<boolean> {
    return setEmotion(state.currentEmotion);
  }

  function reset(): void {
    state.currentEmotion = Emotion.Neutral;
    state.lastGroup = null;
    state.lastIndex = 0;
  }

  return {
    setEmotion,
    getCurrentEmotion: () => state.currentEmotion,
    replay,
    reset,
    debugState: () => state,
  };
}

// ---------------------------------------------------------------------------
// Helpers (kept small to honour the 30-line-per-function rule).
// ---------------------------------------------------------------------------

function pickBinding(
  manager: MotionManagerLike,
  emotion: Emotion,
  bindings: Record<Emotion, EmotionBinding>,
  preferNamed: boolean,
): EmotionBinding {
  const fallback = bindings[emotion];
  if (!preferNamed) return fallback;

  const namedGroup = EMOTION_EmotionMotionName_value[emotion];
  if (namedGroup && managerHasGroup(manager, namedGroup)) {
    return { group: namedGroup, index: 0, expression: fallback?.expression };
  }
  return fallback;
}

function managerHasGroup(manager: MotionManagerLike, group: string): boolean {
  const groups = manager.groups;
  if (!groups) return false;
  return groups.idle === group || groups.tap === group;
}

async function playMotion(
  manager: MotionManagerLike,
  binding: EmotionBinding,
  priority: number,
): Promise<boolean> {
  try {
    const result = await Promise.resolve(
      manager.startMotion(binding.group, binding.index ?? 0, priority),
    );
    return result !== false;
  } catch (err) {
    // Model may lack the requested group/index; fall back to Idle.
    try {
      await Promise.resolve(manager.startMotion('Idle', 0, priority));
      return true;
    } catch {
      return false;
    }
  }
}

async function applyExpression(
  manager: MotionManagerLike,
  name: string,
): Promise<boolean> {
  const em = manager.expressionManager;
  if (!em) return false;
  try {
    const r = await Promise.resolve(em.setExpression(name));
    return r !== false;
  } catch {
    return false;
  }
}
