import { describe, it, expect, vi, beforeEach } from 'vitest';
import { Emotion } from '../src/lib/live2d/emotions';
import {
  createEmotionMotionManager,
  HARU_EMOTION_BINDINGS,
} from '../src/lib/live2d/motion-manager';
import type { MotionManagerLike, ExpressionManagerLike } from '../src/lib/live2d/types';

function makeStub() {
  const calls: Array<{ group: string; index: number; priority: number }> = [];
  const expressionCalls: string[] = [];
  const expression: ExpressionManagerLike = {
    setExpression: vi.fn((name: string) => {
      expressionCalls.push(name);
      return true;
    }),
    resetExpression: vi.fn(),
    definitions: [
      { Name: 'f00' },
      { Name: 'f01' },
      { Name: 'f02' },
      { Name: 'f03' },
      { Name: 'f04' },
      { Name: 'f05' },
      { Name: 'f06' },
      { Name: 'f07' },
    ],
  };
  const mgr: MotionManagerLike & { expressionManager: ExpressionManagerLike } = {
    startMotion: vi.fn((group: string, index = 0, priority = 0) => {
      calls.push({ group, index, priority });
      return true;
    }),
    stopAllMotions: vi.fn(),
    groups: { idle: 'Idle', tap: 'Tap' },
    state: { currentGroup: null },
    expressionManager: expression,
  };
  return { mgr, calls, expressionCalls };
}

describe('motion-manager state machine', () => {
  beforeEach(() => vi.restoreAllMocks());

  it('starts in Neutral with no prior calls', () => {
    const { mgr } = makeStub();
    const m = createEmotionMotionManager(mgr);
    expect(m.getCurrentEmotion()).toBe(Emotion.Neutral);
    expect(m.debugState().lastGroup).toBeNull();
  });

  it('transitions idle → happy → idle, calling startMotion each step', async () => {
    const { mgr, calls, expressionCalls } = makeStub();
    const m = createEmotionMotionManager(mgr);

    await m.setEmotion(Emotion.Happy);
    expect(m.getCurrentEmotion()).toBe(Emotion.Happy);
    const happyBinding = HARU_EMOTION_BINDINGS[Emotion.Happy];
    // Haru has no 'Happy' group so binding falls back to 'Tap'.
    expect(calls.at(-1)?.group).toBe(happyBinding.group);
    expect(expressionCalls.at(-1)).toBe(happyBinding.expression);

    await m.setEmotion(Emotion.Neutral);
    expect(m.getCurrentEmotion()).toBe(Emotion.Neutral);
    expect(calls.at(-1)?.group).toBe('Idle');
    expect(calls).toHaveLength(2);
  });

  it('replay() re-invokes startMotion with the last emotion', async () => {
    const { mgr, calls } = makeStub();
    const m = createEmotionMotionManager(mgr);
    await m.setEmotion(Emotion.Surprised);
    const before = calls.length;
    await m.replay();
    expect(calls.length).toBe(before + 1);
    expect(calls.at(-1)?.group).toBe(calls.at(-2)?.group);
  });

  it('reset() clears state without touching the manager', () => {
    const { mgr } = makeStub();
    const m = createEmotionMotionManager(mgr);
    m.reset();
    expect(m.getCurrentEmotion()).toBe(Emotion.Neutral);
    expect(mgr.startMotion).not.toHaveBeenCalled();
  });

  it('uses named group when model exposes matching group name', async () => {
    const { mgr, calls } = makeStub();
    // Pretend this model has a dedicated "Happy" group.
    mgr.groups = { idle: 'Idle', tap: 'Happy' };
    const m = createEmotionMotionManager(mgr);
    await m.setEmotion(Emotion.Happy);
    expect(calls.at(-1)?.group).toBe('Happy');
  });

  it('falls back to Idle if startMotion throws', async () => {
    const calls: string[] = [];
    const mgr: MotionManagerLike = {
      startMotion: vi.fn((group: string) => {
        calls.push(group);
        if (group !== 'Idle') throw new Error('missing group');
        return true;
      }),
      stopAllMotions: vi.fn(),
      groups: { idle: 'Idle' },
    };
    const m = createEmotionMotionManager(mgr, { preferNamedGroups: false });
    await m.setEmotion(Emotion.Angry);
    expect(calls).toContain('Idle');
  });
});
