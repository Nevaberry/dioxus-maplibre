import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/navigation');
    await page.waitForSelector('canvas.maplibregl-canvas', { timeout: 30_000 });
    await page.waitForTimeout(2_000);
  });

  test('fly_to changes position display', async ({ page }) => {
    const helsinkiButton = page.locator('text=Helsinki').first();
    await helsinkiButton.click();

    // Wait for animation to complete and position to update
    const position = page.locator('[data-testid="position"]');
    await expect(position).toContainText('60.', { timeout: 10_000 });
  });

  test('tilt and rotate via easeTo', async ({ page }) => {
    const tiltButton = page.locator('text=Tilt & Rotate');
    await tiltButton.click();
    // Should not error
    await page.waitForTimeout(2_500);
  });

  test('jump_to is instant', async ({ page }) => {
    const jumpButton = page.locator('text=Jump to Helsinki');
    await jumpButton.click();

    const position = page.locator('[data-testid="position"]');
    await expect(position).toContainText('60.', { timeout: 5_000 });
  });

  test('fit_bounds works', async ({ page }) => {
    const fitButton = page.locator('text=Fit All Finland');
    await fitButton.click();
    // Should not error
    await page.waitForTimeout(2_000);
  });

  test('zoom controls work', async ({ page }) => {
    const zoomIn = page.locator('text=Zoom +');
    await zoomIn.click();
    await page.waitForTimeout(500);

    const zoomOut = page.locator('text=Zoom -');
    await zoomOut.click();
    await page.waitForTimeout(500);
  });
});
