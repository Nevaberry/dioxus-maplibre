import { test, expect } from '@playwright/test';

test.describe('Layers', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/layers');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
    // Wait for layers to render
    await page.waitForTimeout(2_000);
  });

  test('map renders with layers', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    await expect(canvas).toBeVisible();
    // No console errors should appear for layer operations
  });

  test('toggle circle visibility', async ({ page }) => {
    const hideButton = page.locator('text=Hide Circles');
    await hideButton.click();

    // Button text should change
    await expect(page.locator('text=Show Circles')).toBeVisible({ timeout: 5_000 });

    // Click again to show
    await page.locator('text=Show Circles').click();
    await expect(page.locator('text=Hide Circles')).toBeVisible({ timeout: 5_000 });
  });

  test('change circle color', async ({ page }) => {
    const colorButton = page.locator('text=Change Circle Color');
    await colorButton.click();
    // Should not error — visual change only
    await page.waitForTimeout(500);
  });
});
