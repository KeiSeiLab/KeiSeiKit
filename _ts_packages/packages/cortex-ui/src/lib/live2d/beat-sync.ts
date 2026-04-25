// Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
// Original: packages/stage-ui-live2d/src/composables/live2d/beat-sync.ts
// Adapted: STUB for Wave 21. The full 359-LOC AIRI implementation drives
// head yaw/roll from audio BPM with semi-implicit Euler springs and four
// stylistic patterns ('punchy-v' | 'balanced-v' | 'swing-lr' | 'sway-sine').
// cortex-ui does not yet have TTS/audio wired in, so every method here is
// a no-op that matches the AIRI shape so call-sites compile unchanged.
// Full port scheduled for Wave 22 (TTS integration).

export type BeatSyncStyleName =
  | 'punchy-v'
  | 'balanced-v'
  | 'swing-lr'
  | 'sway-sine';

export interface BeatBaseAngles {
  x: number;
  y: number;
  z: number;
}

export interface BeatSyncController {
  readonly targetX: number;
  readonly targetY: number;
  readonly targetZ: number;
  readonly velocityX: number;
  readonly velocityY: number;
  readonly velocityZ: number;
  updateTargets(now: number): void;
  scheduleBeat(timestamp?: number | null): void;
  setStyle(style: BeatSyncStyleName): void;
  getStyle(): BeatSyncStyleName;
  setAutoStyleShift(enabled: boolean): void;
  debugState(): {
    primed: boolean;
    patternStarted: boolean;
    bpm: number | null;
    style: BeatSyncStyleName;
  };
}

export function createBeatSyncController(): BeatSyncController {
  let style: BeatSyncStyleName = 'balanced-v';
  return {
    targetX: 0,
    targetY: 0,
    targetZ: 0,
    velocityX: 0,
    velocityY: 0,
    velocityZ: 0,
    updateTargets: () => { /* stub: Wave 22 will drive head sway. */ },
    scheduleBeat: () => { /* stub: Wave 22 will consume TTS beats. */ },
    setStyle: (s) => { style = s; },
    getStyle: () => style,
    setAutoStyleShift: () => { /* stub */ },
    debugState: () => ({
      primed: false,
      patternStarted: false,
      bpm: null,
      style,
    }),
  };
}
