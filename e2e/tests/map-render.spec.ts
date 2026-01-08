import { test, expect } from '@playwright/test';

test.describe('Map Rendering', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('map container exists', async ({ page }) => {
    // Wait for map container to appear
    const mapContainer = page.locator('[id*="map_"][id*="_container"]');
    await expect(mapContainer).toBeVisible({ timeout: 10000 });
  });

  test('maplibre canvas renders', async ({ page }) => {
    // Wait for MapLibre canvas to be created
    const canvas = page.locator('.maplibregl-canvas');
    await expect(canvas).toBeVisible({ timeout: 15000 });
  });

  test('map tiles load', async ({ page }) => {
    // Wait for tiles to load
    await page.waitForSelector('.maplibregl-canvas', { timeout: 15000 });

    // Give tiles time to render
    await page.waitForTimeout(3000);

    // Take screenshot for visual regression
    const map = page.locator('[id*="map_"][id*="_container"]');
    await expect(map).toHaveScreenshot('map-loaded.png', {
      maxDiffPixels: 1000, // Allow some variance for tile antialiasing
      timeout: 10000,
    });
  });

  test('sidebar displays', async ({ page }) => {
    const sidebar = page.locator('h1:has-text("dioxus-maplibre")');
    await expect(sidebar).toBeVisible();
  });
});
