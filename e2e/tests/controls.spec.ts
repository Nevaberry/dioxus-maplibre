import { test, expect } from '@playwright/test';

test.describe('Controls', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/controls');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
    await page.waitForTimeout(2_000);
  });

  test('navigation control appears', async ({ page }) => {
    const navControl = page.locator('.maplibregl-ctrl-zoom-in');
    await expect(navControl).toBeVisible({ timeout: 10_000 });
  });

  test('scale control appears', async ({ page }) => {
    const scaleControl = page.locator('.maplibregl-ctrl-scale');
    await expect(scaleControl).toBeVisible({ timeout: 10_000 });
  });

  test('fullscreen control appears', async ({ page }) => {
    const fullscreenControl = page.locator('.maplibregl-ctrl-fullscreen');
    await expect(fullscreenControl).toBeVisible({ timeout: 10_000 });
  });
});
