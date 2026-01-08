import { test, expect } from '@playwright/test';

test.describe('Markers', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for map to be ready
    await page.waitForSelector('.maplibregl-canvas', { timeout: 15000 });
    // Wait for markers to render
    await page.waitForTimeout(2000);
  });

  test('markers render on map', async ({ page }) => {
    // There should be 4 markers (Helsinki, Tampere, Turku, Oulu)
    const markers = page.locator('.maplibregl-marker');
    await expect(markers).toHaveCount(4, { timeout: 10000 });
  });

  test('clicking marker opens popup', async ({ page }) => {
    // Click the first marker
    const marker = page.locator('.maplibregl-marker').first();
    await marker.click();

    // Popup should appear
    const popup = page.locator('.maplibregl-popup');
    await expect(popup).toBeVisible({ timeout: 5000 });
  });

  test('popup contains marker name', async ({ page }) => {
    // Click a marker
    const marker = page.locator('.maplibregl-marker').first();
    await marker.click();

    // Popup should contain text
    const popupContent = page.locator('.maplibregl-popup-content');
    await expect(popupContent).toContainText(/Helsinki|Tampere|Turku|Oulu/);
  });

  test('marker click updates sidebar', async ({ page }) => {
    // Click a marker
    const marker = page.locator('.maplibregl-marker').first();
    await marker.click();

    // Sidebar should show marker ID
    const markerInfo = page.locator('text=helsinki').or(
      page.locator('text=tampere')
    ).or(
      page.locator('text=turku')
    ).or(
      page.locator('text=oulu')
    );
    await expect(markerInfo).toBeVisible({ timeout: 5000 });
  });
});
