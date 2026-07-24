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
    await expect(page.locator('text=Press and hold')).toBeVisible();
    await expect(page.getByTestId('interaction-sequence')).toContainText(
      'Hover → orange',
    );
    await expect(page.getByTestId('interaction-sequence')).toContainText(
      'Mouse down → red',
    );
    await expect(page.getByTestId('interaction-sequence')).toContainText(
      'Mouse up → released',
    );
    await expect(page.getByTestId('interaction-sequence')).toContainText(
      'Click → selected',
    );
  });

  test('reports hover, mouse down, mouse up, and click in order', async ({ page }) => {
    const canvas = page.locator('canvas.maplibregl-canvas');
    const point = await page.evaluate(() => {
      const registry = (window as any).__dioxus_maplibre_maps ?? {};
      const map = [...new Set(Object.values(registry))][0] as any;
      if (!map) {
        throw new Error('MapLibre map was not registered');
      }
      const projected = map.project([24.94, 60.17]);
      return { x: projected.x, y: projected.y };
    });
    const canvasBox = await canvas.boundingBox();
    if (!canvasBox) {
      throw new Error('MapLibre canvas has no bounding box');
    }

    await page.mouse.move(canvasBox.x + point.x, canvasBox.y + point.y);
    await expect(page.getByTestId('hover-info')).toContainText('Senate Square');

    await page.mouse.down();
    await expect(page.getByTestId('press-info')).toContainText('Pressed: Senate Square');
    await expect.poll(async () => page.evaluate(() => {
      const map = [...new Set(Object.values((window as any).__dioxus_maplibre_maps ?? {}))][0] as any;
      return map?.getFeatureState({ source: 'interactive', id: 1 }).pressed;
    })).toBe(true);

    await page.mouse.up();
    await expect(page.getByTestId('press-info')).toHaveCount(0);
    await expect(page.getByTestId('release-info')).toContainText('Released: Senate Square');
    await expect(page.getByTestId('click-info')).toContainText('Clicked: Senate Square');
    await expect.poll(async () => page.evaluate(() => {
      const map = [...new Set(Object.values((window as any).__dioxus_maplibre_maps ?? {}))][0] as any;
      return map?.getFeatureState({ source: 'interactive', id: 1 }).pressed;
    })).toBeUndefined();
  });
});
