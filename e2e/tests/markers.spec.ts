import { test, expect } from '@playwright/test';

test.describe('Markers', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/markers');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
    // Wait for markers to render
    await page.waitForTimeout(2_000);
  });

  test('markers appear in DOM', async ({ page }) => {
    // MapLibre markers are rendered as DOM elements
    const markers = page.locator('.maplibregl-marker');
    await expect(markers.first()).toBeVisible({ timeout: 10_000 });
  });

  test('add marker button works', async ({ page }) => {
    const addButton = page.locator('text=Add Marker');
    await addButton.click();

    // Count should increase
    await expect(page.locator('text=Markers on map: 5')).toBeVisible({ timeout: 5_000 });
  });

  test('remove marker button works', async ({ page }) => {
    const removeButton = page.locator('text=Remove Helsinki');
    await removeButton.click();

    // Give time for DOM update
    await page.waitForTimeout(1_000);
    // The remove should succeed without errors
  });
});
