import { test, expect } from '@playwright/test';

test.describe('Events', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
  });

  test('ready event fires', async ({ page }) => {
    const eventLog = page.locator('[data-testid="event-log"]');
    await expect(eventLog).toContainText('Map ready', { timeout: 10_000 });
  });

  test('click event fires with coordinates', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    await canvas.click({ position: { x: 400, y: 300 } });

    const eventLog = page.locator('[data-testid="event-log"]');
    await expect(eventLog).toContainText('Click:', { timeout: 5_000 });
  });
});
