import { test, expect } from '@playwright/test';

test.describe('Basic Map', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for map canvas to appear
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
  });

  test('map canvas renders', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    await expect(canvas).toBeVisible();
  });

  test('event log shows ready event', async ({ page }) => {
    const eventLog = page.locator('[data-testid="event-log"]');
    await expect(eventLog).toContainText('Map ready');
  });

  test('click updates event log', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    await canvas.click({ position: { x: 300, y: 300 } });

    const eventLog = page.locator('[data-testid="event-log"]');
    await expect(eventLog).toContainText('Click:', { timeout: 5_000 });
  });

  test('position updates on move', async ({ page }) => {
    // Position should show coordinates after map loads
    const position = page.locator('text=Position:');
    await expect(position).not.toContainText('--', { timeout: 10_000 });
  });
});
