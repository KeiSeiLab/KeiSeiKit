import { describe, it, expect, vi } from 'vitest';
import {
  createExpressionController,
  type ExpressionEntry,
} from '../src/lib/live2d/expression-controller';
import type { Live2DModelLike } from '../src/lib/live2d/types';

function makeStubModel() {
  const paramValues = new Map<string, number>();
  const getParameterValueById = vi.fn((id: string) => paramValues.get(id) ?? 0);
  const setParameterValueById = vi.fn((id: string, v: number) => {
    paramValues.set(id, v);
  });
  const setExpression = vi.fn(() => true);
  const resetExpression = vi.fn();
  const model: Live2DModelLike = {
    internalModel: {
      motionManager: {
        startMotion: vi.fn(() => true),
        stopAllMotions: vi.fn(),
        expressionManager: {
          setExpression,
          resetExpression,
          definitions: [{ Name: 'f00' }, { Name: 'f01' }],
        },
      },
      coreModel: { getParameterValueById, setParameterValueById },
    },
  };
  return { model, setExpression, resetExpression, paramValues, setParameterValueById };
}

describe('expression-controller', () => {
  it('lists expression definitions from the manager', () => {
    const { model } = makeStubModel();
    const c = createExpressionController(model);
    expect(c.listAvailable()).toEqual(['f00', 'f01']);
  });

  it('setActive records the active name and calls setExpression', () => {
    const { model, setExpression } = makeStubModel();
    const c = createExpressionController(model);
    expect(c.setActive('f01')).toBe(true);
    expect(c.debugActive()).toBe('f01');
    expect(setExpression).toHaveBeenCalledWith('f01');
  });

  it('clear() invokes resetExpression and drops all overrides', () => {
    const { model, resetExpression } = makeStubModel();
    const c = createExpressionController(model);
    c.setActive('f00');
    c.registerOverride(sampleEntry('ParamAngleX', 'Overwrite', 10, 0));
    c.clear();
    expect(resetExpression).toHaveBeenCalled();
    expect(c.debugActive()).toBeNull();
  });

  it('applyExpressions writes Add-blended value on top of modelDefault', () => {
    const { model, setParameterValueById } = makeStubModel();
    const c = createExpressionController(model);
    c.registerOverride(sampleEntry('ParamMouthForm', 'Add', 0.3, 0));
    c.applyExpressions(model.internalModel.coreModel);
    // Expected write: default (0) + currentValue (0.3) = 0.3
    expect(setParameterValueById).toHaveBeenCalledWith('ParamMouthForm', 0.3);
  });

  it('applyExpressions resets param on active→inactive transition', () => {
    const { model, setParameterValueById } = makeStubModel();
    const c = createExpressionController(model);
    const entry = sampleEntry('ParamEyeLOpen', 'Multiply', 0.3, 1);
    c.registerOverride(entry);
    c.applyExpressions(model.internalModel.coreModel); // active
    // Now turn the entry into a noop (Multiply * 1 = noop)
    entry.currentValue = 1;
    c.applyExpressions(model.internalModel.coreModel); // should reset to 1 (default)
    const calls = setParameterValueById.mock.calls.filter(
      ([id]) => id === 'ParamEyeLOpen',
    );
    // Last call on ParamEyeLOpen must be the reset-to-modelDefault (1).
    expect(calls.at(-1)?.[1]).toBe(1);
  });
});

function sampleEntry(
  id: string,
  blend: ExpressionEntry['blend'],
  current: number,
  def: number,
): ExpressionEntry {
  return {
    name: id,
    parameterId: id,
    blend,
    currentValue: current,
    modelDefault: def,
    targetValue: current,
  };
}
