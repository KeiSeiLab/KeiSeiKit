import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, cleanup } from '@testing-library/svelte';
import Live2DPet from '../src/components/Live2DPet.svelte';
import { Emotion } from '../src/lib/live2d/emotions';

// Stub pixi-live2d-display/cubism4: Live2DModel.from() returns a fake model
// with the structural shape the motion manager needs.
vi.mock('pixi-live2d-display/cubism4', () => {
  const fakeModel = {
    x: 0,
    y: 0,
    width: 100,
    height: 100,
    scale: { set: vi.fn() },
    internalModel: {
      motionManager: {
        startMotion: vi.fn(() => true),
        stopAllMotions: vi.fn(),
        groups: { idle: 'Idle', tap: 'Tap' },
        state: { currentGroup: null },
        expressionManager: {
          setExpression: vi.fn(() => true),
          resetExpression: vi.fn(),
          definitions: [],
        },
      },
      coreModel: {
        getParameterValueById: vi.fn(() => 0),
        setParameterValueById: vi.fn(),
      },
    },
    destroy: vi.fn(),
  };
  return {
    Live2DModel: {
      from: vi.fn(async () => fakeModel),
    },
  };
});

// Stub pixi.js Application — no WebGL in jsdom.
vi.mock('pixi.js', () => ({
  Application: vi.fn().mockImplementation(() => ({
    stage: { addChild: vi.fn() },
    destroy: vi.fn(),
  })),
}));

describe('Live2DPet component', () => {
  afterEach(() => cleanup());

  it('mounts without throwing and renders a canvas element', () => {
    const { container } = render(Live2DPet, {
      props: {
        modelPath: './live2d-models/haru/haru_greeter_t03.model3.json',
        mood: Emotion.Neutral,
        width: 128,
        height: 128,
      },
    });
    const canvas = container.querySelector('canvas.live2d-canvas');
    expect(canvas).not.toBeNull();
    // jsdom exposes no WebGL → component sets load_error and renders hint.
    // The canvas still mounts either way; that's the contract we need.
    const width = canvas?.getAttribute('width');
    expect(width).toBe('128');
  });

  it('includes an accessible aria-label on the canvas', () => {
    const { container } = render(Live2DPet, {
      props: {
        modelPath: './live2d-models/haru/haru_greeter_t03.model3.json',
        mood: Emotion.Happy,
      },
    });
    const canvas = container.querySelector('canvas') as HTMLCanvasElement | null;
    expect(canvas?.getAttribute('aria-label')).toContain('happy');
  });

  it('renders a graceful error hint when WebGL is unavailable (jsdom path)', async () => {
    const { container, findByRole } = render(Live2DPet, {
      props: {
        modelPath: './live2d-models/haru/haru_greeter_t03.model3.json',
        mood: Emotion.Neutral,
        width: 256,
        height: 256,
      },
    });
    const hint = await findByRole('status');
    expect(hint.textContent).toMatch(/WebGL/i);
    expect(container.querySelector('canvas')).not.toBeNull();
  });
});
