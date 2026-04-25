// Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
// Original: packages/stage-ui-live2d/src/composables/live2d/motion-manager.ts (types)
// Adapted: Vue Ref types removed; pure structural types only.

import type { Emotion } from './emotions';

/**
 * Minimal shape we need from the Cubism 4 core model at runtime.
 * We deliberately keep this abstract so the component can run against a
 * stub (tests) or the real pixi-live2d-display Cubism4InternalModel.coreModel.
 */
export interface CubismCoreModelLike {
  getParameterValueById(id: string): number;
  setParameterValueById(id: string, value: number): void;
}

/**
 * Shape of the motion manager exposed by pixi-live2d-display.
 * Structural typing only; we never construct one of these ourselves.
 */
export interface MotionManagerLike {
  startMotion(group: string, index?: number, priority?: number): Promise<boolean> | boolean;
  stopAllMotions(): void;
  groups?: { idle?: string; tap?: string };
  state?: { currentGroup?: string | null };
}

/**
 * Shape of the expression manager exposed by pixi-live2d-display.
 */
export interface ExpressionManagerLike {
  setExpression(name: string | number): Promise<boolean> | boolean;
  resetExpression?(): void;
  definitions?: Array<{ Name?: string; name?: string }>;
}

/**
 * Shape of the Live2DModel instance we wrap.
 * We use a `Live2DModelLike` interface so the Svelte component can be
 * tested with a tiny stub instead of requiring a real WebGL canvas.
 */
export interface Live2DModelLike {
  internalModel: {
    motionManager: MotionManagerLike & {
      expressionManager?: ExpressionManagerLike;
    };
    coreModel: CubismCoreModelLike;
  };
  on?(event: string, handler: () => void): void;
  destroy?(options?: { children?: boolean; texture?: boolean; baseTexture?: boolean }): void;
}

export interface Live2DPetProps {
  /** Absolute or relative URL to the `*.model3.json` settings file. */
  modelPath: string;
  /** Current emotion; re-evaluated reactively. */
  mood?: Emotion;
  /** Width of the PIXI canvas in CSS pixels. Defaults to 256. */
  width?: number;
  /** Height of the PIXI canvas in CSS pixels. Defaults to 256. */
  height?: number;
  /** Drive `ParamMouthOpenY` (0..1). Typically fed by RMS of playing TTS
   *  audio. Default 0 (mouth closed). */
  mouthOpenY?: number;
}
