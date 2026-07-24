import { expect, type Page, test } from '@playwright/test';

type BrowserIssue = { kind: 'console' | 'page'; message: string };

function collectBrowserIssues(page: Page): BrowserIssue[] {
  const issues: BrowserIssue[] = [];
  page.on('pageerror', (error) => {
    issues.push({ kind: 'page', message: error.message });
  });
  page.on('console', (message) => {
    if (message.type() === 'error') {
      issues.push({ kind: 'console', message: message.text() });
    }
  });
  return issues;
}

async function waitForMap(page: Page, layer?: string): Promise<void> {
  await page.locator('canvas.maplibregl-canvas').waitFor({ timeout: 30_000 });
  await expect.poll(async () => page.evaluate((layerId) => {
    const registry = (window as any).__dioxus_maplibre_maps ?? {};
    const maps = [...new Set(Object.values(registry))] as any[];
    if (maps.length !== 1) return false;
    return layerId
      ? Boolean(maps[0]?.getLayer(layerId))
      : (maps[0]?.getStyle?.()?.layers?.length ?? 0) > 0;
  }, layer)).toBe(true);
}

async function currentMapValue<T>(page: Page, expression: (map: any) => T): Promise<T> {
  return page.evaluate((source) => {
    const registry = (window as any).__dioxus_maplibre_maps ?? {};
    const maps = [...new Set(Object.values(registry))] as any[];
    if (maps.length !== 1) throw new Error(`Expected one live map, found ${maps.length}`);
    // The callback is test-owned source, not application input.
    return Function('map', `return (${source})(map)`)(maps[0]);
  }, expression.toString()) as Promise<T>;
}

async function startWithLocalMaps(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('dioxus-maplibre-mode', 'offline');
  });
}

test('scene rail and lab controls drive one live MapLibre map', async ({ page }) => {
  const issues = collectBrowserIssues(page);
  await startWithLocalMaps(page);
  await page.goto('/');
  await waitForMap(page);

  await page.getByTestId('scene-3d').click();
  await waitForMap(page, 'mobile-buildings-3d');
  await expect(page.getByTestId('building-details')).toContainText('Query ready');

  await page.getByTestId('nav-lab').click();
  await page.getByTestId('lab-layers').click();
  await waitForMap(page, 'lab-buildings-layer');
  await page.getByTestId('layer-buildings').click();
  await expect.poll(() => currentMapValue(page, (map) => map.getLayoutProperty(
    'lab-buildings-layer',
    'visibility',
  ))).toBe('none');

  await page.getByTestId('layer-terrain').click();
  await expect.poll(() => currentMapValue(page, (map) => map.getTerrain()?.source)).toBe('lab-dem');

  await expect.poll(() => currentMapValue(page, (map) => ({
    symbol: Boolean(map.getLayer('lab-symbol-layer')),
    image: map.hasImage('mobile-pin'),
    fog: Boolean(map.getSky()),
  }))).toEqual({ symbol: true, image: true, fog: true });

  await page.getByRole('button', { name: 'Back to feature lab' }).click();
  await page.getByTestId('lab-camera').click();
  await waitForMap(page);
  await page.getByTestId('camera-pitch').fill('68');
  await page.getByTestId('camera-roll').fill('12');
  await page.getByTestId('camera-globe').click();
  await expect.poll(() => currentMapValue(page, (map) => ({
    pitch: Math.round(map.getPitch()),
    roll: Math.round(map.getRoll()),
    projection: map.getProjection().type,
  }))).toEqual({ pitch: 68, roll: 12, projection: 'globe' });
  expect(issues).toEqual([]);
});

test('interaction console reports hover, press, release, tap, and touch phases', async ({ page }) => {
  const issues = collectBrowserIssues(page);
  await startWithLocalMaps(page);
  await page.goto('/');
  await waitForMap(page);
  await page.getByTestId('nav-lab').click();
  await page.getByTestId('lab-interaction').click();
  await waitForMap(page, 'interaction-fill');
  await expect.poll(() => page.evaluate(() => {
    const roots = Object.values((window as any).__dioxus_maplibre_layer_handlers ?? {}) as any[];
    const handlers = roots.find((value) => value?.['interaction-fill'])?.['interaction-fill'];
    return Boolean(
      handlers?.mouseenter
      && handlers?.mousedown
      && handlers?.mouseup
      && handlers?.touchstart
      && handlers?.touchend
      && handlers?.click,
    );
  })).toBe(true);

  const point = await page.evaluate(() => {
    const maps = [...new Set(Object.values((window as any).__dioxus_maplibre_maps ?? {}))] as any[];
    const map = maps[0];
    const projected = map.project([24.94, 60.172]);
    const box = map.getCanvas().getBoundingClientRect();
    return { x: box.left + projected.x, y: box.top + projected.y };
  });

  await page.mouse.move(point.x, point.y);
  await expect(page.locator('.event-list li strong')).toHaveText(['HOVER']);
  await page.mouse.down();
  await expect(page.locator('.event-list li strong').first()).toHaveText('PRESS');
  await page.mouse.up();

  await expect.poll(async () => page.locator('.event-list li strong').allTextContents()).toEqual([
    'TAP',
    'RELEASE',
    'PRESS',
    'HOVER',
  ]);

  await page.touchscreen.tap(point.x, point.y);
  await expect.poll(async () => page.locator('.event-list li strong').allTextContents()).toEqual([
    'TAP',
    'RELEASE',
    'PRESS',
    'TAP',
    'RELEASE',
    'PRESS',
    'HOVER',
  ]);
  expect(issues).toEqual([]);
});

test('offline pack pauses, resumes, and survives a network-free reload', async ({ context, page }) => {
  const issues = collectBrowserIssues(page);
  await startWithLocalMaps(page);
  await page.goto('/');
  await waitForMap(page);
  await page.evaluate(async () => navigator.serviceWorker.ready);

  await page.getByTestId('nav-offline').click();
  await page.getByTestId('pack-matterhorn').click();
  await page.getByTestId('start-download').click();
  await expect(page.getByTestId('downloading-screen')).toBeVisible();

  await page.waitForTimeout(250);
  await page.getByTestId('pause-download').click();
  await expect(page.getByTestId('download-progress')).toContainText('Paused');
  await expect(page.getByTestId('pause-download')).toContainText('Resume');
  await page.getByTestId('pause-download').click();

  await expect(page.getByTestId('offline-ready-screen')).toBeVisible({ timeout: 20_000 });
  await expect(page.getByTestId('offline-ready-badge')).toHaveText(/Offline ready/);
  await page.getByRole('button', { name: 'Use offline mode' }).click();
  await expect.poll(() => page.evaluate(() => localStorage.getItem('dioxus-maplibre-mode'))).toBe(
    'offline',
  );

  await context.setOffline(true);
  await page.reload({ waitUntil: 'domcontentloaded' });
  await page.getByTestId('mobile-showcase').waitFor({ timeout: 20_000 });
  await waitForMap(page);
  await expect(page.getByTestId('mode-offline')).toHaveAttribute('aria-pressed', 'true');
  await expect.poll(() => currentMapValue(page, (map) => map.getStyle()?.name)).toBe(
    'Helsinki offline',
  );
  expect(issues).toEqual([]);
});
