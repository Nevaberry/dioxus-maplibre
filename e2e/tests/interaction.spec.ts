import { test, expect } from '@playwright/test';

test.describe('Interaction', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/interaction');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
    await page.waitForTimeout(3_000);
  });

  test('map renders with interactive layer', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    await expect(canvas).toBeVisible();
  });

  test('sidebar shows interaction instructions', async ({ page }) => {
    await expect(page.locator('text=Hover over circles')).toBeVisible();
    await expect(page.locator('text=Click circles')).toBeVisible();
  });
});
