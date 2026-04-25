// Ported from AIRI (https://github.com/moeru-ai/airi), MIT License.
// Original: packages/stage-ui-live2d/src/composables/live2d/expression-controller.ts
// Adapted: AIRI's version maintains its own Pinia-backed parameter blend store
// to support authoring UI (ExpressionPanel) on top of exp3 files. In
// cortex-ui we delegate exp3 parsing to pixi-live2d-display's own
// ExpressionManager — the Svelte component merely drives emotion→expression
// selection. This controller tracks the lookup layer (Emotion → exp name)
// plus a small active-parameter registry for optional per-frame overrides.

import type {
  CubismCoreModelLike,
  ExpressionManagerLike,
  Live2DModelLike,
} from './types';

export type ExpressionBlendMode = 'Add' | 'Multiply' | 'Overwrite';

export interface ExpressionEntry {
  name: string;
  parameterId: string;
  blend: ExpressionBlendMode;
  /** Runtime-mutable: current blended target. */
  currentValue: number;
  /** The Cubism parameter's initial value at load time. */
  modelDefault: number;
  /** The "intended activation value" from the exp3 file. */
  targetValue: number;
}

export interface ExpressionController {
  setActive(name: string): boolean;
  clear(): void;
  listAvailable(): string[];
  /** Apply any registered manual overrides every frame. */
  applyExpressions(coreModel: CubismCoreModelLike): void;
  registerOverride(entry: ExpressionEntry): void;
  removeOverride(parameterId: string): void;
  debugActive(): string | null;
}

interface ControllerState {
  active: string | null;
  overrides: Map<string, ExpressionEntry>;
  // Previous-frame active param IDs, so we can explicitly reset stale writes
  // on the inactive→active transition (port of AIRI's `activeLastFrame`).
  lastFrame: Set<string>;
}

function initialState(): ControllerState {
  return {
    active: null,
    overrides: new Map(),
    lastFrame: new Set(),
  };
}

/**
 * Create an expression controller bound to a Live2D model.
 *
 * For Cubism 4 models loaded via pixi-live2d-display, the `expressionManager`
 * is populated from `model3.json.FileReferences.Expressions`. This wrapper
 * adds:
 *   - `setActive(name)`: idempotent, logs on unknown name.
 *   - `applyExpressions(coreModel)`: apply manual parameter overrides on top
 *     of whatever the native expression manager has already written.
 */
export function createExpressionController(
  model: Live2DModelLike,
): ExpressionController {
  const state = initialState();

  function getManager(): ExpressionManagerLike | null {
    return model.internalModel?.motionManager?.expressionManager ?? null;
  }

  function listAvailable(): string[] {
    const em = getManager();
    if (!em?.definitions) return [];
    return em.definitions.map((d, i) => d.Name ?? d.name ?? String(i));
  }

  function setActive(name: string): boolean {
    const em = getManager();
    if (!em) return false;
    const available = listAvailable();
    if (!available.includes(name)) {
      // Try numeric index as fallback (pixi-live2d-display accepts both).
      const asNum = Number(name);
      if (!Number.isInteger(asNum)) return false;
    }
    try {
      const r = em.setExpression(name);
      if (r === false) return false;
      state.active = name;
      return true;
    } catch {
      return false;
    }
  }

  function clear(): void {
    const em = getManager();
    if (em?.resetExpression) {
      try { em.resetExpression(); } catch { /* noop */ }
    }
    state.active = null;
    state.overrides.clear();
    state.lastFrame.clear();
  }

  function registerOverride(entry: ExpressionEntry): void {
    state.overrides.set(entry.parameterId, entry);
  }

  function removeOverride(parameterId: string): void {
    state.overrides.delete(parameterId);
  }

  // ---- Per-frame --------------------------------------------------------

  function applyExpressions(coreModel: CubismCoreModelLike): void {
    const activeThisFrame = new Set<string>();

    for (const entry of state.overrides.values()) {
      if (isNoop(entry)) continue;
      const value = computeBlend(entry, coreModel);
      coreModel.setParameterValueById(entry.parameterId, value);
      activeThisFrame.add(entry.parameterId);
    }

    for (const paramId of state.lastFrame) {
      if (!activeThisFrame.has(paramId)) {
        const entry = state.overrides.get(paramId);
        if (entry) coreModel.setParameterValueById(paramId, entry.modelDefault);
      }
    }

    state.lastFrame.clear();
    activeThisFrame.forEach((p) => state.lastFrame.add(p));
  }

  return {
    setActive,
    clear,
    listAvailable,
    applyExpressions,
    registerOverride,
    removeOverride,
    debugActive: () => state.active,
  };
}

// ---- Internal pure helpers (blend math, ported verbatim) ------------------

function isNoop(entry: ExpressionEntry): boolean {
  switch (entry.blend) {
    case 'Add':
      return entry.currentValue === 0;
    case 'Multiply':
      return entry.currentValue === 1;
    default:
      return entry.currentValue === entry.modelDefault;
  }
}

function computeBlend(
  entry: ExpressionEntry,
  coreModel: CubismCoreModelLike,
): number {
  switch (entry.blend) {
    case 'Add':
      return entry.modelDefault + entry.currentValue;
    case 'Multiply': {
      const frameValue = coreModel.getParameterValueById(entry.parameterId);
      return frameValue * entry.currentValue;
    }
    default:
      return entry.currentValue;
  }
}
