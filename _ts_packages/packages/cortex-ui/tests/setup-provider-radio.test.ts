import { describe, it, expect, vi, afterEach, beforeEach } from 'vitest';
import { render, cleanup, fireEvent, waitFor } from '@testing-library/svelte';

// SetupCustomPet renders <PortraitUploader> only after current_config is
// non-null; the upload module is fetch-only and safe in jsdom, so no
// stubbing is needed. config.ts's load_config touches localStorage on
// import, which beforeEach() resets cleanly.
import Setup from '../src/routes/Setup.svelte';

beforeEach(() => {
  localStorage.clear();
  window.history.replaceState({}, '', '/');
});

afterEach(() => cleanup());

describe('Setup — provider radio + cwd field', () => {
  it('renders all three provider radios with anthropic checked by default', async () => {
    const { getByTestId } = render(Setup, {
      props: { on_saved: () => {} },
    });
    const anthropic = getByTestId('provider-anthropic') as HTMLInputElement;
    const openai = getByTestId('provider-openai') as HTMLInputElement;
    const kimi = getByTestId('provider-kimi') as HTMLInputElement;
    expect(anthropic).toBeTruthy();
    expect(openai).toBeTruthy();
    expect(kimi).toBeTruthy();
    expect(anthropic.checked).toBe(true);
    expect(openai.checked).toBe(false);
    expect(kimi.checked).toBe(false);
  });

  it('switching provider radios updates the bound state', async () => {
    const { getByTestId } = render(Setup, {
      props: { on_saved: () => {} },
    });
    const openai = getByTestId('provider-openai') as HTMLInputElement;
    await fireEvent.click(openai);
    await waitFor(() => {
      expect(openai.checked).toBe(true);
    });
    const anthropic = getByTestId('provider-anthropic') as HTMLInputElement;
    expect(anthropic.checked).toBe(false);
  });

  it('persists provider + cwd to localStorage on Save', async () => {
    const on_saved = vi.fn();
    const { getByTestId, container } = render(Setup, {
      props: { on_saved },
    });

    // pick kimi
    const kimi = getByTestId('provider-kimi') as HTMLInputElement;
    await fireEvent.click(kimi);

    // set cwd
    const cwd_input = getByTestId('cwd-input') as HTMLInputElement;
    await fireEvent.input(cwd_input, { target: { value: '/Users/me/proj' } });

    // fill the required token so submit() doesn't bail early
    const tok = container.querySelector('#token') as HTMLInputElement;
    await fireEvent.input(tok, { target: { value: 'tkn' } });

    const submit = container.querySelector(
      'button[type="submit"]',
    ) as HTMLButtonElement;
    await fireEvent.click(submit);

    await waitFor(() => {
      expect(localStorage.getItem('kei-cortex-provider')).toBe('kimi');
    });
    expect(localStorage.getItem('kei-cortex-cwd')).toBe('/Users/me/proj');
    expect(on_saved).toHaveBeenCalled();
  });

  it('reads ?cwd= URL param on first load when no localStorage value', () => {
    window.history.replaceState({}, '', '/?cwd=/from/url');
    const { getByTestId } = render(Setup, {
      props: { on_saved: () => {} },
    });
    const cwd_input = getByTestId('cwd-input') as HTMLInputElement;
    expect(cwd_input.value).toBe('/from/url');
  });
});
