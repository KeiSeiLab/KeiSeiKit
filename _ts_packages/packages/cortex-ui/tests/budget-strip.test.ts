import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, cleanup, waitFor } from '@testing-library/svelte';
import BudgetStrip from '../src/components/BudgetStrip.svelte';

// Stub the usage-client so the component is exercised in isolation.
// The mock is reset per-test via vi.mocked(...).mockResolvedValueOnce.
vi.mock('../src/lib/usage/usage-client', () => ({
  getUsage: vi.fn(),
}));
import { getUsage } from '../src/lib/usage/usage-client';
const mockedGetUsage = vi.mocked(getUsage);

describe('BudgetStrip', () => {
  beforeEach(() => {
    mockedGetUsage.mockReset();
  });
  afterEach(() => cleanup());

  it('renders the loading skeleton on first paint', () => {
    // Pending promise so the strip never resolves the loading state.
    mockedGetUsage.mockReturnValueOnce(new Promise(() => {}));
    const { getByTestId } = render(BudgetStrip, {
      props: { config: { daemon_url: 'http://x', token: 't' } },
    });
    expect(getByTestId('budget-loading')).toBeTruthy();
  });

  it('renders today / week / month / top after a successful fetch', async () => {
    mockedGetUsage.mockResolvedValueOnce({
      today_cents: 87,
      week_cents: 432,
      month_cents: 1789,
      top_provider: 'anthropic',
      top_model: 'claude-3-5-sonnet',
    });
    const { getByTestId } = render(BudgetStrip, {
      props: { config: { daemon_url: 'http://x', token: 't' } },
    });
    await waitFor(() => {
      expect(getByTestId('budget-today').textContent).toBe('$0.87');
    });
    expect(getByTestId('budget-week').textContent).toBe('$4.32');
    expect(getByTestId('budget-month').textContent).toBe('$17.89');
    expect(getByTestId('budget-top').textContent).toContain('anthropic');
  });

  it('renders the "ledger unavailable" muted state when /usage returns null', async () => {
    mockedGetUsage.mockResolvedValueOnce(null);
    const { getByTestId } = render(BudgetStrip, {
      props: { config: { daemon_url: 'http://x', token: 't' } },
    });
    await waitFor(() => {
      expect(getByTestId('budget-unavailable')).toBeTruthy();
    });
    expect(getByTestId('budget-unavailable').textContent).toBe(
      'ledger unavailable',
    );
  });
});
