import { test, expect } from '@playwright/test';

test.describe('Map Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for map to be ready
    await page.waitForSelector('.maplibregl-canvas', { timeout: 15000 });
    await page.waitForTimeout(2000);
  });

  test('clicking map updates last click position', async ({ page }) => {
    // Get the canvas element
    const canvas = page.locator('.maplibregl-canvas');

    // Click in the center of the map
    const box = await canvas.boundingBox();
    if (box) {
      await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    }

    // Wait for click event to propagate
    await page.waitForTimeout(500);

    // Sidebar should show click coordinates
    const latText = page.locator('text=/Lat: \\d+\\.\\d+/');
    await expect(latText).toBeVisible({ timeout: 5000 });
  });

  test('navigation button flies to city', async ({ page }) => {
    // Get initial map center from sidebar
    const initialCenter = await page.locator('text=/Center: \\d+\\.\\d+/').textContent();

    // Click "Oulu" navigation button
    await page.locator('button:has-text("Oulu")').click();

    // Wait for fly animation
    await page.waitForTimeout(2000);

    // Center should have changed
    const newCenter = await page.locator('text=/Center: \\d+\\.\\d+/').textContent();
    expect(newCenter).not.toBe(initialCenter);
  });

  test('dragging map updates center', async ({ page }) => {
    const canvas = page.locator('.maplibregl-canvas');
    const box = await canvas.boundingBox();

    if (box) {
      // Get initial center
      const initialCenter = await page.locator('text=/Center: \\d+\\.\\d+/').textContent();

      // Drag the map
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width / 2 + 100, box.y + box.height / 2);
      await page.mouse.up();

      // Wait for move event
      await page.waitForTimeout(500);

      // Center should have changed
      const newCenter = await page.locator('text=/Center: \\d+\\.\\d+/').textContent();
      expect(newCenter).not.toBe(initialCenter);
    }
  });

  test('zoom updates in sidebar', async ({ page }) => {
    const canvas = page.locator('.maplibregl-canvas');

    // Get initial zoom
    const initialZoom = await page.locator('text=/Zoom: \\d+\\.\\d+/').textContent();

    // Scroll to zoom in
    await canvas.hover();
    await page.mouse.wheel(0, -500);

    // Wait for zoom animation
    await page.waitForTimeout(1000);

    // Zoom should have changed
    const newZoom = await page.locator('text=/Zoom: \\d+\\.\\d+/').textContent();
    expect(newZoom).not.toBe(initialZoom);
  });
});
